use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::AppResult, model::UserModel};

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Option<UserModel>> {
        let row = sqlx::query_as!(
            UserModel,
            "SELECT id, email, username, password_hash, default_currency, timezone,
                    failed_login_count, locked_until, created_at, updated_at
             FROM users WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn find_by_email(&self, email: &str) -> AppResult<Option<UserModel>> {
        let row = sqlx::query_as!(
            UserModel,
            "SELECT id, email, username, password_hash, default_currency, timezone,
                    failed_login_count, locked_until, created_at, updated_at
             FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn find_by_username(&self, username: &str) -> AppResult<Option<UserModel>> {
        let row = sqlx::query_as!(
            UserModel,
            "SELECT id, email, username, password_hash, default_currency, timezone,
                    failed_login_count, locked_until, created_at, updated_at
             FROM users WHERE username = $1",
            username
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    /// Find by email OR username (for login with `credential` field).
    pub async fn find_by_credential(&self, credential: &str) -> AppResult<Option<UserModel>> {
        let row = sqlx::query_as!(
            UserModel,
            "SELECT id, email, username, password_hash, default_currency, timezone,
                    failed_login_count, locked_until, created_at, updated_at
             FROM users WHERE email = $1 OR username = $1",
            credential
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn create(
        &self,
        email: &str,
        username: &str,
        password_hash: &str,
        default_currency: &str,
        timezone: &str,
    ) -> AppResult<UserModel> {
        let row = sqlx::query_as!(
            UserModel,
            "INSERT INTO users (email, username, password_hash, default_currency, timezone)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id, email, username, password_hash, default_currency, timezone,
                       failed_login_count, locked_until, created_at, updated_at",
            email,
            username,
            password_hash,
            default_currency,
            timezone
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn update_profile(
        &self,
        id: Uuid,
        username: Option<&str>,
        default_currency: Option<&str>,
        timezone: Option<&str>,
    ) -> AppResult<UserModel> {
        let row = sqlx::query_as!(
            UserModel,
            "UPDATE users
             SET username         = COALESCE($2, username),
                 default_currency = COALESCE($3, default_currency),
                 timezone         = COALESCE($4, timezone),
                 updated_at       = NOW()
             WHERE id = $1
             RETURNING id, email, username, password_hash, default_currency, timezone,
                       failed_login_count, locked_until, created_at, updated_at",
            id,
            username,
            default_currency,
            timezone
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn update_password(&self, id: Uuid, password_hash: &str) -> AppResult<()> {
        sqlx::query!(
            "UPDATE users SET password_hash = $2, updated_at = NOW() WHERE id = $1",
            id,
            password_hash
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn increment_failed_login(&self, id: Uuid) -> AppResult<i16> {
        let row = sqlx::query!(
            "UPDATE users
             SET failed_login_count = failed_login_count + 1, updated_at = NOW()
             WHERE id = $1
             RETURNING failed_login_count",
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row.failed_login_count)
    }

    pub async fn lock_until(
        &self,
        id: Uuid,
        until: chrono::DateTime<chrono::Utc>,
    ) -> AppResult<()> {
        sqlx::query!(
            "UPDATE users SET locked_until = $2, updated_at = NOW() WHERE id = $1",
            id,
            until
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn reset_failed_login(&self, id: Uuid) -> AppResult<()> {
        sqlx::query!(
            "UPDATE users
             SET failed_login_count = 0, locked_until = NULL, updated_at = NOW()
             WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn email_exists(&self, email: &str) -> AppResult<bool> {
        let row = sqlx::query!("SELECT 1 AS one FROM users WHERE email = $1", email)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.is_some())
    }

    pub async fn username_exists(&self, username: &str) -> AppResult<bool> {
        let row = sqlx::query!("SELECT 1 AS one FROM users WHERE username = $1", username)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.is_some())
    }
}
