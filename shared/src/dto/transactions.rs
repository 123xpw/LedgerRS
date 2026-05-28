use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::TransactionType;
use super::tags::TagResponse;

// ─── Requests ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTransactionRequest {
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    pub amount: Decimal,
    pub currency: String,
    pub category_id: Option<Uuid>,
    #[serde(default)]
    pub tag_ids: Vec<Uuid>,
    pub note: Option<String>,
    pub occurred_at: DateTime<Utc>,
    pub custom_rate: Option<Decimal>,
    // Transfer-only fields
    pub target_ledger_id: Option<Uuid>,
    pub target_currency: Option<String>,
    pub target_amount: Option<Decimal>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTransactionRequest {
    pub amount: Option<Decimal>,
    pub currency: Option<String>,
    /// `Some(None)` clears the category.
    #[serde(default)]
    pub category_id: Option<Option<Uuid>>,
    /// Full replacement of the tag set.
    pub tag_ids: Option<Vec<Uuid>>,
    /// `Some(None)` clears the note.
    #[serde(default)]
    pub note: Option<Option<String>>,
    pub occurred_at: Option<DateTime<Utc>>,
    /// `Some(None)` resets to the system rate.
    #[serde(default)]
    pub custom_rate: Option<Option<Decimal>>,
}

// ─── Embedded ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferPeerInfo {
    pub ledger_id: Uuid,
    pub amount: Decimal,
    pub currency: String,
}

// ─── Response ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub id: Uuid,
    pub ledger_id: Uuid,
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub amount: Decimal,
    pub currency: String,
    pub base_amount: Decimal,
    pub exchange_rate: Decimal,
    pub category_id: Option<Uuid>,
    pub category_name: Option<String>,
    pub category_type: Option<String>,
    pub tags: Vec<TagResponse>,
    pub note: Option<String>,
    pub occurred_at: DateTime<Utc>,
    pub custom_rate: Option<Decimal>,
    pub transfer_pair_id: Option<Uuid>,
    pub transfer_peer: Option<TransferPeerInfo>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
