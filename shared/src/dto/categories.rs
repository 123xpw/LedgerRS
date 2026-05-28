use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ─── Requests ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub category_type: String,
    pub parent_id: Option<Uuid>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
    pub sort_order: Option<i32>,
}

// ─── Response ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryResponse {
    pub id: Uuid,
    pub ledger_id: Option<Uuid>,
    pub parent_id: Option<Uuid>,
    pub name: String,
    #[serde(rename = "type")]
    pub category_type: String,
    pub is_system: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}
