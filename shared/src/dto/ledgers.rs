use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ─── Requests ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLedgerRequest {
    pub name: String,
    pub base_currency: String,
    pub icon: Option<String>,
    pub initial_balance: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateLedgerRequest {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub initial_balance: Option<Decimal>,
    // base_currency intentionally omitted — triggers BASE_CURRENCY_IMMUTABLE if sent
}

// ─── Response ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub icon: String,
    pub base_currency: String,
    pub initial_balance: Decimal,
    /// Running balance = initial_balance + Σincome_base − Σexpense_base.
    pub balance: Decimal,
    pub this_month_income: Decimal,
    pub this_month_expense: Decimal,
    /// None if no total budget is set for this ledger.
    pub budget_usage_rate: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
