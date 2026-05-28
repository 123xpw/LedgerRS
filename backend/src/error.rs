use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use shared::error_codes as codes;
use thiserror::Error;

/// All errors that can originate from the application.
/// Every variant maps to a specific HTTP status code and error-code string
/// (see `IntoResponse` below).
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Validation: {0}")]
    Validation(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Last ledger cannot be deleted")]
    LastLedger,

    #[error("Base currency is immutable after ledger creation")]
    BaseCurrencyImmutable,

    #[error("Category has {count} transaction(s); use force=true to proceed")]
    CategoryHasTransactions { count: i64 },

    #[error("Account locked")]
    AccountLocked { retry_after_seconds: u64 },

    #[error("No exchange rate available for this date")]
    ExchangeRateUnavailable,

    #[error("Invalid backup file format: {0}")]
    InvalidBackupFormat(String),

    #[error("Database error")]
    Database(#[from] sqlx::Error),

    #[error("External API error: {0}")]
    ExternalApi(String),

    #[error("Internal error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message, details) = match &self {
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                codes::NOT_FOUND,
                msg.clone(),
                None,
            ),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                codes::UNAUTHORIZED,
                "请先登录".into(),
                None,
            ),
            AppError::Forbidden(msg) => (
                StatusCode::FORBIDDEN,
                codes::FORBIDDEN,
                msg.clone(),
                None,
            ),
            AppError::Validation(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                codes::VALIDATION_ERROR,
                msg.clone(),
                None,
            ),
            AppError::Conflict(msg) => (
                StatusCode::CONFLICT,
                codes::CONFLICT,
                msg.clone(),
                None,
            ),
            AppError::LastLedger => (
                StatusCode::CONFLICT,
                codes::LAST_LEDGER,
                "至少保留一个账本".into(),
                None,
            ),
            AppError::BaseCurrencyImmutable => (
                StatusCode::UNPROCESSABLE_ENTITY,
                codes::BASE_CURRENCY_IMMUTABLE,
                "账本基准币种创建后不可修改".into(),
                None,
            ),
            AppError::CategoryHasTransactions { count } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                codes::CATEGORY_HAS_TRANSACTIONS,
                format!("该分类下有 {count} 条账目，请先迁移或使用 force=true"),
                Some(json!({ "transaction_count": count })),
            ),
            AppError::AccountLocked { retry_after_seconds } => (
                StatusCode::LOCKED,
                codes::ACCOUNT_LOCKED,
                format!("账号已锁定，请 {retry_after_seconds} 秒后重试"),
                Some(json!({ "retry_after_seconds": retry_after_seconds })),
            ),
            AppError::ExchangeRateUnavailable => (
                StatusCode::UNPROCESSABLE_ENTITY,
                codes::EXCHANGE_RATE_UNAVAILABLE,
                "账目日期无可用汇率".into(),
                None,
            ),
            AppError::InvalidBackupFormat(msg) => (
                StatusCode::BAD_REQUEST,
                codes::INVALID_BACKUP_FORMAT,
                msg.clone(),
                None,
            ),
            AppError::Database(e) => {
                tracing::error!(error = %e, "数据库错误");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    codes::DATABASE_ERROR,
                    "内部错误".into(),
                    None,
                )
            }
            AppError::ExternalApi(msg) => (
                StatusCode::BAD_GATEWAY,
                codes::EXTERNAL_API_ERROR,
                msg.clone(),
                None,
            ),
            AppError::Internal(e) => {
                tracing::error!(error = %e, "内部错误");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    codes::INTERNAL_ERROR,
                    "内部错误".into(),
                    None,
                )
            }
        };

        let body = Json(json!({
            "error": {
                "code": code,
                "message": message,
                "details": details
            }
        }));
        (status, body).into_response()
    }
}

/// Convenience alias.
pub type AppResult<T> = Result<T, AppError>;
