use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use shared::dto::ledgers::{CreateLedgerRequest, LedgerResponse, UpdateLedgerRequest};

use crate::{error::AppResult, middleware::AuthUser, service, AppState};

pub async fn list(
    State(state): State<AppState>,
    auth: AuthUser,
) -> AppResult<Json<Vec<LedgerResponse>>> {
    // Use the user's own timezone for monthly stats.
    let user = state.repos.users.find_by_id(auth.user_id).await?.unwrap();
    let ledgers = service::ledgers::list_ledgers(&state, auth.user_id, &user.timezone).await?;
    Ok(Json(ledgers))
}

pub async fn get(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<LedgerResponse>> {
    let user = state.repos.users.find_by_id(auth.user_id).await?.unwrap();
    let ledger = service::ledgers::get_ledger(&state, auth.user_id, id, &user.timezone).await?;
    Ok(Json(ledger))
}

pub async fn create(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CreateLedgerRequest>,
) -> AppResult<Json<LedgerResponse>> {
    let ledger = service::ledgers::create_ledger(&state, auth.user_id, req).await?;
    Ok(Json(ledger))
}

pub async fn update(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateLedgerRequest>,
) -> AppResult<Json<LedgerResponse>> {
    let user = state.repos.users.find_by_id(auth.user_id).await?.unwrap();
    let ledger =
        service::ledgers::update_ledger(&state, auth.user_id, id, req).await?;
    let _ = user;
    Ok(Json(ledger))
}

pub async fn delete(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    service::ledgers::delete_ledger(&state, auth.user_id, id).await?;
    Ok(Json(serde_json::json!({ "message": "账本已删除" })))
}
