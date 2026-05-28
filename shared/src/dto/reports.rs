use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ─── Overview ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverviewResponse {
    pub total_income: Decimal,
    pub total_expense: Decimal,
    pub net: Decimal,
    pub transaction_count: i64,
}

// ─── Trend ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyStats {
    pub year_month: String,
    pub income: Decimal,
    pub expense: Decimal,
    pub net: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendResponse {
    pub months: Vec<MonthlyStats>,
}

// ─── Category Breakdown ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryBreakdownItem {
    pub category_id: Option<Uuid>,
    pub category_name: Option<String>,
    pub total: Decimal,
    pub count: i64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryBreakdownResponse {
    pub items: Vec<CategoryBreakdownItem>,
    pub total: Decimal,
}

// ─── MoM Comparison ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodStats {
    pub income: Decimal,
    pub expense: Decimal,
    pub net: Decimal,
    pub transaction_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResponse {
    pub this_period: PeriodStats,
    pub last_period: PeriodStats,
    /// Percentage change (None if last period had zero income).
    pub income_change_pct: Option<f64>,
    /// Percentage change (None if last period had zero expense).
    pub expense_change_pct: Option<f64>,
}

// ─── Top Categories ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopCategoryItem {
    pub category_id: Option<Uuid>,
    pub category_name: Option<String>,
    pub total: Decimal,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopCategoriesResponse {
    pub items: Vec<TopCategoryItem>,
}

// ─── Multi-ledger ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiLedgerReportRequest {
    pub ledger_ids: Vec<Uuid>,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerStats {
    pub ledger_id: Uuid,
    pub ledger_name: String,
    pub base_currency: String,
    pub total_income: Decimal,
    pub total_expense: Decimal,
    pub net: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiLedgerReportResponse {
    pub ledgers: Vec<LedgerStats>,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}
