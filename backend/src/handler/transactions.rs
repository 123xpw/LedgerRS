use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use shared::dto::pagination::PaginatedResponse;
use shared::dto::transactions::{
    CreateTransactionRequest, TransactionResponse, UpdateTransactionRequest,
};

use crate::{
    error::AppResult,
    middleware::AuthUser,
    service::transactions::{self, ListParams},
    AppState,
};

#[derive(Deserialize)]
pub struct TxQuery {
    pub category_id: Option<Uuid>,
    pub tag_id: Option<Uuid>,
    #[serde(rename = "type")]
    pub transaction_type: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub keyword: Option<String>,
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_per_page")]
    pub per_page: i64,
}

fn default_page() -> i64 { 1 }
fn default_per_page() -> i64 { 20 }

pub async fn list(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(ledger_id): Path<Uuid>,
    Query(q): Query<TxQuery>,
) -> AppResult<Json<PaginatedResponse<TransactionResponse>>> {
    let result = transactions::list_transactions(
        &state,
        auth.user_id,
        ledger_id,
        ListParams {
            category_id: q.category_id,
            tag_id: q.tag_id,
            transaction_type: q.transaction_type,
            start_date: q.start_date,
            end_date: q.end_date,
            keyword: q.keyword,
            page: q.page.max(1),
            per_page: q.per_page.clamp(1, 100),
        },
    )
    .await?;
    Ok(Json(result))
}

pub async fn get(
    State(state): State<AppState>,
    auth: AuthUser,
    Path((ledger_id, tx_id)): Path<(Uuid, Uuid)>,
) -> AppResult<Json<TransactionResponse>> {
    let tx = transactions::get_transaction(&state, auth.user_id, ledger_id, tx_id).await?;
    Ok(Json(tx))
}

pub async fn create(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(ledger_id): Path<Uuid>,
    Json(req): Json<CreateTransactionRequest>,
) -> AppResult<Json<TransactionResponse>> {
    let tx = transactions::create_transaction(&state, auth.user_id, ledger_id, req).await?;
    Ok(Json(tx))
}

pub async fn update(
    State(state): State<AppState>,
    auth: AuthUser,
    Path((ledger_id, tx_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<UpdateTransactionRequest>,
) -> AppResult<Json<TransactionResponse>> {
    let tx =
        transactions::update_transaction(&state, auth.user_id, ledger_id, tx_id, req).await?;
    Ok(Json(tx))
}

pub async fn delete(
    State(state): State<AppState>,
    auth: AuthUser,
    Path((ledger_id, tx_id)): Path<(Uuid, Uuid)>,
) -> AppResult<Json<serde_json::Value>> {
    transactions::delete_transaction(&state, auth.user_id, ledger_id, tx_id).await?;
    Ok(Json(serde_json::json!({ "message": "账目已删除" })))
}
