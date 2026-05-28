use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const BACKUP_SCHEMA_VERSION: &str = "1.0";

// ─── Backup file structure ────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupData {
    pub schema_version: String,
    pub exported_at: DateTime<Utc>,
    pub user_id: Uuid,
    pub ledgers: Vec<BackupLedger>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupLedger {
    pub id: Uuid,
    pub name: String,
    pub icon: String,
    pub base_currency: String,
    pub initial_balance: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub categories: Vec<BackupCategory>,
    pub tags: Vec<BackupTag>,
    pub transactions: Vec<BackupTransaction>,
    pub budgets: Vec<BackupBudget>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupCategory {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    #[serde(rename = "type")]
    pub category_type: String,
    pub sort_order: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupTag {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupTransaction {
    pub id: Uuid,
    #[serde(rename = "type")]
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupBudget {
    pub id: Uuid,
    pub category_id: Option<Uuid>,
    pub period: String,
    pub amount: Decimal,
}

// ─── Restore response ─────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct RestoreResponse {
    pub counts: RestoredCounts,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RestoredCounts {
    pub ledgers: i64,
    pub categories: i64,
    pub tags: i64,
    pub transactions: i64,
    pub budgets: i64,
}
