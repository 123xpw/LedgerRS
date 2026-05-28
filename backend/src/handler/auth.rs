use axum::{extract::State, Json};
use serde_json::{json, Value};

use shared::dto::auth::{
    AuthResponse, ChangePasswordRequest, LoginRequest, LogoutRequest, RefreshTokenRequest,
    RegisterRequest, TokenRefreshResponse, UpdateProfileRequest, UserResponse,
};

use crate::{
    error::{AppError, AppResult},
    middleware::AuthUser,
    service,
    AppState,
};

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<Json<AuthResponse>> {
    let resp = service::auth::register(
        &state,
        &req.email,
        &req.username,
        &req.password,
        req.default_currency.as_deref(),
        req.timezone.as_deref(),
    )
    .await?;
    Ok(Json(resp))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    let resp = service::auth::login(&state, &req.credential, &req.password).await?;
    Ok(Json(resp))
}

pub async fn refresh(
    State(state): State<AppState>,
    Json(req): Json<RefreshTokenRequest>,
) -> AppResult<Json<TokenRefreshResponse>> {
    let resp = service::auth::refresh(&state, &req.refresh_token).await?;
    Ok(Json(resp))
}

pub async fn logout(
    State(state): State<AppState>,
    Json(req): Json<LogoutRequest>,
) -> AppResult<Json<Value>> {
    service::auth::logout(&state, &req.refresh_token).await?;
    Ok(Json(json!({ "message": "已退出登录" })))
}

pub async fn me(
    State(state): State<AppState>,
    auth: AuthUser,
) -> AppResult<Json<UserResponse>> {
    let user = state
        .repos
        .users
        .find_by_id(auth.user_id)
        .await?
        .ok_or(AppError::Unauthorized)?;
    Ok(Json(UserResponse {
        id: user.id,
        email: user.email,
        username: user.username,
        default_currency: user.default_currency,
        timezone: user.timezone,
        created_at: user.created_at,
        updated_at: user.updated_at,
    }))
}

pub async fn update_profile(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<UpdateProfileRequest>,
) -> AppResult<Json<UserResponse>> {
    // Username uniqueness check.
    if let Some(ref new_username) = req.username {
        let exists = state.repos.users.username_exists(new_username).await?;
        if exists {
            let current = state
                .repos
                .users
                .find_by_id(auth.user_id)
                .await?
                .ok_or(AppError::Unauthorized)?;
            if current.username != *new_username {
                return Err(AppError::Conflict("用户名已存在".into()));
            }
        }
    }

    let user = state
        .repos
        .users
        .update_profile(
            auth.user_id,
            req.username.as_deref(),
            req.default_currency.as_deref(),
            req.timezone.as_deref(),
        )
        .await?;
    Ok(Json(UserResponse {
        id: user.id,
        email: user.email,
        username: user.username,
        default_currency: user.default_currency,
        timezone: user.timezone,
        created_at: user.created_at,
        updated_at: user.updated_at,
    }))
}

pub async fn change_password(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<ChangePasswordRequest>,
) -> AppResult<Json<Value>> {
    let user = state
        .repos
        .users
        .find_by_id(auth.user_id)
        .await?
        .ok_or(AppError::Unauthorized)?;

    if !service::auth::verify_password(&req.current_password, &user.password_hash)? {
        return Err(AppError::Validation("当前密码错误".into()));
    }

    if req.new_password.len() < 8 {
        return Err(AppError::Validation("新密码至少8位".into()));
    }

    let new_hash = service::auth::hash_password(&req.new_password)?;
    state
        .repos
        .users
        .update_password(auth.user_id, &new_hash)
        .await?;
    Ok(Json(json!({ "message": "密码修改成功" })))
}
