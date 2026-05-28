use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use shared::dto::categories::{CategoryResponse, CreateCategoryRequest, UpdateCategoryRequest};

use crate::{error::AppResult, middleware::AuthUser, service, AppState};

#[derive(Deserialize)]
pub struct DeleteQuery {
    #[serde(default)]
    pub force: bool,
}

pub async fn list(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(ledger_id): Path<Uuid>,
) -> AppResult<Json<Vec<CategoryResponse>>> {
    let cats = service::categories::list_categories(&state, auth.user_id, ledger_id).await?;
    Ok(Json(cats))
}

pub async fn create(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(ledger_id): Path<Uuid>,
    Json(req): Json<CreateCategoryRequest>,
) -> AppResult<Json<CategoryResponse>> {
    let cat =
        service::categories::create_category(&state, auth.user_id, ledger_id, req).await?;
    Ok(Json(cat))
}

pub async fn update(
    State(state): State<AppState>,
    auth: AuthUser,
    Path((ledger_id, cat_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<UpdateCategoryRequest>,
) -> AppResult<Json<CategoryResponse>> {
    let cat =
        service::categories::update_category(&state, auth.user_id, ledger_id, cat_id, req)
            .await?;
    Ok(Json(cat))
}

pub async fn delete(
    State(state): State<AppState>,
    auth: AuthUser,
    Path((ledger_id, cat_id)): Path<(Uuid, Uuid)>,
    Query(q): Query<DeleteQuery>,
) -> AppResult<Json<serde_json::Value>> {
    service::categories::delete_category(&state, auth.user_id, ledger_id, cat_id, q.force)
        .await?;
    Ok(Json(serde_json::json!({ "message": "分类已删除" })))
}
