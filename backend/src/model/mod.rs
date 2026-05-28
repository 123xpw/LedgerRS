//! Database row types — one struct per table, one-to-one field mapping.
//! All structs derive `sqlx::FromRow` so they can be used with `query_as!`.

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

// ─── Users ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserModel {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub default_currency: String,
    pub timezone: String,
    pub failed_login_count: i16,
    pub locked_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ─── Refresh tokens ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RefreshTokenModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

// ─── Ledgers ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct LedgerModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub icon: String,
    pub base_currency: String,
    pub initial_balance: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Enriched ledger row returned by the listing query (includes computed fields).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct LedgerSummaryRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub icon: String,
    pub base_currency: String,
    pub initial_balance: Decimal,
    pub balance: Decimal,
    pub this_month_income: Decimal,
    pub this_month_expense: Decimal,
    pub budget_total: Option<Decimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ─── Categories ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CategoryModel {
    pub id: Uuid,
    pub ledger_id: Option<Uuid>,
    pub parent_id: Option<Uuid>,
    pub name: String,
    // Aliased from `type` (reserved keyword) in all queries.
    pub category_type: String,
    pub is_system: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}

// ─── Tags ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TagModel {
    pub id: Uuid,
    pub ledger_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

// ─── Transactions ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TransactionModel {
    pub id: Uuid,
    pub ledger_id: Uuid,
    // Aliased from `type` in all queries.
    pub transaction_type: String,
    pub amount: Decimal,
    pub currency: String,
    pub base_amount: Decimal,
    pub applied_rate: Decimal,
    pub category_id: Option<Uuid>,
    pub note: Option<String>,
    pub occurred_at: DateTime<Utc>,
    pub custom_rate: Option<Decimal>,
    pub transfer_pair_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Transaction row joined with its category (used in list/detail queries).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TransactionWithCategoryRow {
    // Transaction fields
    pub id: Uuid,
    pub ledger_id: Uuid,
    pub transaction_type: String,
    pub amount: Decimal,
    pub currency: String,
    pub base_amount: Decimal,
    pub applied_rate: Decimal,
    pub category_id: Option<Uuid>,
    pub note: Option<String>,
    pub occurred_at: DateTime<Utc>,
    pub custom_rate: Option<Decimal>,
    pub transfer_pair_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Category fields (nullable — transaction may have no category)
    pub cat_name: Option<String>,
    pub cat_type: Option<String>,
    // Transfer peer fields (nullable)
    pub peer_ledger_id: Option<Uuid>,
    pub peer_amount: Option<Decimal>,
    pub peer_currency: Option<String>,
}

// ─── Budgets ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct BudgetModel {
    pub id: Uuid,
    pub ledger_id: Uuid,
    pub category_id: Option<Uuid>,
    pub period: String,
    pub amount: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ─── Exchange rates ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ExchangeRateModel {
    pub id: Uuid,
    pub base_currency: String,
    pub quote_currency: String,
    pub rate: Decimal,
    pub date: NaiveDate,
    pub created_at: DateTime<Utc>,
}
