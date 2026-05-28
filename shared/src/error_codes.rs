//! Error code string constants that appear in `error.code` JSON fields.

// ─── Auth ────────────────────────────────────────────────────────────────────
pub const UNAUTHORIZED:           &str = "UNAUTHORIZED";
pub const INVALID_CREDENTIALS:    &str = "INVALID_CREDENTIALS";
pub const TOKEN_EXPIRED:          &str = "TOKEN_EXPIRED";
pub const REFRESH_TOKEN_INVALID:  &str = "REFRESH_TOKEN_INVALID";
pub const ACCOUNT_LOCKED:         &str = "ACCOUNT_LOCKED";

// ─── Access control ──────────────────────────────────────────────────────────
pub const FORBIDDEN:              &str = "FORBIDDEN";
pub const NOT_FOUND:              &str = "NOT_FOUND";
pub const CONFLICT:               &str = "CONFLICT";

// ─── Ledger ───────────────────────────────────────────────────────────────────
pub const LAST_LEDGER:            &str = "LAST_LEDGER";
pub const BASE_CURRENCY_IMMUTABLE:&str = "BASE_CURRENCY_IMMUTABLE";

// ─── Validation ──────────────────────────────────────────────────────────────
pub const VALIDATION_ERROR:            &str = "VALIDATION_ERROR";
pub const EXCHANGE_RATE_UNAVAILABLE:   &str = "EXCHANGE_RATE_UNAVAILABLE";
pub const CATEGORY_HAS_TRANSACTIONS:   &str = "CATEGORY_HAS_TRANSACTIONS";
pub const INVALID_BACKUP_FORMAT:       &str = "INVALID_BACKUP_FORMAT";

// ─── Server ──────────────────────────────────────────────────────────────────
pub const DATABASE_ERROR:         &str = "DATABASE_ERROR";
pub const INTERNAL_ERROR:         &str = "INTERNAL_ERROR";
pub const EXTERNAL_API_ERROR:     &str = "EXTERNAL_API_ERROR";
