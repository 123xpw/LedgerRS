use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ─── Requests ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBudgetRequest {
    /// None = total budget; Some = category-specific budget.
    pub category_id: Option<Uuid>,
    /// "week" | "month" | "year"
    pub period: String,
    pub amount: Decimal,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateBudgetRequest {
    pub amount: Decimal,
}

// ─── Response ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetResponse {
    pub id: Uuid,
    pub ledger_id: Uuid,
    pub category_id: Option<Uuid>,
    /// "week" | "month" | "year"
    pub period: String,
    pub amount: Decimal,
    /// Current-period expenditure (base currency, transfers excluded).
    pub spent: Decimal,
    /// amount − spent (negative = overspent).
    pub remaining: Decimal,
    /// spent / amount × 100 (percentage).
    pub usage_rate: f64,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
