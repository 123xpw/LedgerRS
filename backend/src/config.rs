use std::env;

use anyhow::Context;

/// All runtime configuration values read from environment variables.
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    /// Access token lifetime in seconds (default: 7 days).
    pub jwt_expiry_seconds: u64,
    /// Refresh token lifetime in seconds (default: 30 days).
    pub refresh_expiry_seconds: u64,
    /// exchangerate.host or compatible API base URL.
    pub exchange_rate_api_url: String,
    pub server_port: u16,
    /// Path to Trunk-built frontend assets.
    pub frontend_dist: String,
}

impl AppConfig {
    /// Load config from environment (supports `.env` via `dotenvy`).
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok(); // ignore error if .env not present

        Ok(Self {
            database_url: required("DATABASE_URL")?,
            jwt_secret: required("JWT_SECRET")?,
            jwt_expiry_seconds: optional_u64("JWT_EXPIRY_SECONDS", 7 * 24 * 3600),
            refresh_expiry_seconds: optional_u64("REFRESH_EXPIRY_SECONDS", 30 * 24 * 3600),
            exchange_rate_api_url: env::var("EXCHANGE_RATE_API_URL")
                .unwrap_or_else(|_| "https://api.exchangerate.host".into()),
            server_port: optional_u64("PORT", 8080) as u16,
            frontend_dist: env::var("FRONTEND_DIST")
                .unwrap_or_else(|_| "frontend/dist".into()),
        })
    }
}

fn required(key: &str) -> anyhow::Result<String> {
    env::var(key).with_context(|| format!("missing required env var: {key}"))
}

fn optional_u64(key: &str, default: u64) -> u64 {
    env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}
