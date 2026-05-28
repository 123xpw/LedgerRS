use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

// ─── Responses ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRateRecord {
    pub base_currency: String,
    pub quote_currency: String,
    pub rate: Decimal,
    pub date: NaiveDate,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyInfo {
    pub code: String,
    /// Latest rate against the query base currency. None if not available.
    pub rate: Option<Decimal>,
}

/// Returned by GET /exchange-rates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatestRatesResponse {
    pub base: String,
    pub is_stale: bool,
    pub stale_days: i64,
    pub currencies: Vec<CurrencyInfo>,
}
