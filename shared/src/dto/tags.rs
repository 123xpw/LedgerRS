use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ─── Requests ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTagRequest {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTagRequest {
    pub name: String,
}

// ─── Response ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagResponse {
    pub id: Uuid,
    pub ledger_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}
