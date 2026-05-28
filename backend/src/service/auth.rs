use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use shared::dto::auth::{AuthResponse, JwtClaims, TokenRefreshResponse, UserResponse};
use shared::types::is_valid_currency;

use crate::{
    error::{AppError, AppResult},
    model::RefreshTokenModel,
    AppState,
};

pub fn hash_password(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("argon2 hash: {e}")))?
        .to_string();
    Ok(hash)
}

pub fn verify_password(password: &str, hash: &str) -> AppResult<bool> {
    let parsed = PasswordHash::new(hash)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("invalid hash: {e}")))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

pub fn hash_token(token: &str) -> String {
    let digest = Sha256::digest(token.as_bytes());
    hex::encode(digest)
}

pub fn issue_access_token(
    user_id: Uuid,
    secret: &str,
    expiry_secs: u64,
) -> AppResult<(String, Uuid)> {
    let jti = Uuid::new_v4();
    let now = Utc::now().timestamp() as u64;
    let claims = JwtClaims {
        sub: user_id,
        exp: now + expiry_secs,
        iat: now,
        jti,
    };
    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(anyhow::anyhow!("jwt encode: {e}")))?;
    Ok((token, jti))
}

pub fn decode_access_token(token: &str, secret: &str) -> AppResult<JwtClaims> {
    let mut validation = Validation::default();
    validation.validate_exp = true;
    jsonwebtoken::decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map(|d| d.claims)
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::Unauthorized,
        _ => AppError::Unauthorized,
    })
}

/// Register a new user, return AuthResponse.
pub async fn register(
    state: &AppState,
    email: &str,
    username: &str,
    password: &str,
    default_currency: Option<&str>,
    timezone: Option<&str>,
) -> AppResult<AuthResponse> {
    let currency = default_currency.unwrap_or("CNY");
    if !is_valid_currency(currency) {
        return Err(AppError::Validation(format!("unsupported currency: {currency}")));
    }

    if state.repos.users.email_exists(email).await? {
        return Err(AppError::Conflict("邮箱已被注册".into()));
    }
    if state.repos.users.username_exists(username).await? {
        return Err(AppError::Conflict("用户名已存在".into()));
    }

    let password_hash = hash_password(password)?;
    let tz = timezone.unwrap_or("Asia/Shanghai");
    let user = state
        .repos
        .users
        .create(email, username, &password_hash, currency, tz)
        .await?;

    // Seed a default ledger for the new user.
    state
        .repos
        .ledgers
        .create(user.id, "我的账本", "💰", currency, rust_decimal::Decimal::ZERO)
        .await?;

    let (access_token, _jti) =
        issue_access_token(user.id, &state.cfg.jwt_secret, state.cfg.jwt_expiry_seconds)?;
    let (refresh_token, token_hash) = new_refresh_token();

    state
        .repos
        .users
        .reset_failed_login(user.id) // ensure clean state
        .await
        .ok();

    store_refresh_token(state, user.id, &token_hash, state.cfg.refresh_expiry_seconds).await?;

    Ok(AuthResponse {
        access_token,
        refresh_token,
        user: user_response_from_model(&user),
    })
}

/// Login with email or username + password.
pub async fn login(
    state: &AppState,
    credential: &str,
    password: &str,
) -> AppResult<AuthResponse> {
    let user = state
        .repos
        .users
        .find_by_credential(credential)
        .await?
        .ok_or(AppError::Unauthorized)?;

    // Check lock.
    if let Some(locked_until) = user.locked_until {
        if locked_until > Utc::now() {
            let secs = (locked_until - Utc::now()).num_seconds().max(0) as u64;
            return Err(AppError::AccountLocked {
                retry_after_seconds: secs,
            });
        }
    }

    if !verify_password(password, &user.password_hash)? {
        let count = state.repos.users.increment_failed_login(user.id).await?;
        if count >= 5 {
            let until = Utc::now() + Duration::minutes(15);
            state.repos.users.lock_until(user.id, until).await?;
            return Err(AppError::AccountLocked {
                retry_after_seconds: 15 * 60,
            });
        }
        return Err(AppError::Unauthorized);
    }

    // Reset on successful login.
    state.repos.users.reset_failed_login(user.id).await?;

    let (access_token, _jti) =
        issue_access_token(user.id, &state.cfg.jwt_secret, state.cfg.jwt_expiry_seconds)?;
    let (refresh_token, token_hash) = new_refresh_token();
    store_refresh_token(state, user.id, &token_hash, state.cfg.refresh_expiry_seconds).await?;

    Ok(AuthResponse {
        access_token,
        refresh_token,
        user: user_response_from_model(&user),
    })
}

pub async fn refresh(
    state: &AppState,
    refresh_token: &str,
) -> AppResult<TokenRefreshResponse> {
    let token_hash = hash_token(refresh_token);

    let rt = find_valid_refresh_token(state, &token_hash).await?;

    let user = state
        .repos
        .users
        .find_by_id(rt.user_id)
        .await?
        .ok_or(AppError::Unauthorized)?;

    // Rotate: revoke old, issue new.
    revoke_refresh_token(state, rt.id).await?;
    let (new_access, _) =
        issue_access_token(user.id, &state.cfg.jwt_secret, state.cfg.jwt_expiry_seconds)?;
    let (new_refresh, new_hash) = new_refresh_token();
    store_refresh_token(state, user.id, &new_hash, state.cfg.refresh_expiry_seconds).await?;

    Ok(TokenRefreshResponse {
        access_token: new_access,
        refresh_token: new_refresh,
    })
}

pub async fn logout(state: &AppState, refresh_token: &str) -> AppResult<()> {
    let token_hash = hash_token(refresh_token);
    let rt = find_valid_refresh_token(state, &token_hash).await;
    if let Ok(rt) = rt {
        revoke_refresh_token(state, rt.id).await?;
    }
    Ok(())
}

// ─── Internal helpers ────────────────────────────────────────────────────────

fn new_refresh_token() -> (String, String) {
    use rand::Rng;
    let raw: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();
    let hash = hash_token(&raw);
    (raw, hash)
}

async fn store_refresh_token(
    state: &AppState,
    user_id: Uuid,
    token_hash: &str,
    expiry_secs: u64,
) -> AppResult<()> {
    let expires_at = Utc::now() + Duration::seconds(expiry_secs as i64);
    sqlx::query!(
        "INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
        user_id,
        token_hash,
        expires_at
    )
    .execute(&state.db)
    .await?;
    Ok(())
}

async fn find_valid_refresh_token(
    state: &AppState,
    token_hash: &str,
) -> AppResult<RefreshTokenModel> {
    sqlx::query_as!(
        RefreshTokenModel,
        "SELECT id, user_id, token_hash, expires_at, revoked_at, created_at
         FROM refresh_tokens
         WHERE token_hash = $1
           AND revoked_at IS NULL
           AND expires_at > NOW()",
        token_hash
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::Unauthorized)
}

async fn revoke_refresh_token(state: &AppState, id: Uuid) -> AppResult<()> {
    sqlx::query!(
        "UPDATE refresh_tokens SET revoked_at = NOW() WHERE id = $1",
        id
    )
    .execute(&state.db)
    .await?;
    Ok(())
}

fn user_response_from_model(u: &crate::model::UserModel) -> UserResponse {
    UserResponse {
        id: u.id,
        email: u.email.clone(),
        username: u.username.clone(),
        default_currency: u.default_currency.clone(),
        timezone: u.timezone.clone(),
        created_at: u.created_at,
        updated_at: u.updated_at,
    }
}
