use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use shared::dto::tags::{CreateTagRequest, TagResponse, UpdateTagRequest};

use crate::{error::AppResult, middleware::AuthUser, service, AppState};

pub async fn list(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(ledger_id): Path<Uuid>,
) -> AppResult<Json<Vec<TagResponse>>> {
    let tags = service::tags::list_tags(&state, auth.user_id, ledger_id).await?;
    Ok(Json(tags))
}

pub async fn create(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(ledger_id): Path<Uuid>,
    Json(req): Json<CreateTagRequest>,
) -> AppResult<Json<TagResponse>> {
    let tag = service::tags::create_tag(&state, auth.user_id, ledger_id, req).await?;
    Ok(Json(tag))
}

pub async fn update(
    State(state): State<AppState>,
    auth: AuthUser,
    Path((ledger_id, tag_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<UpdateTagRequest>,
) -> AppResult<Json<TagResponse>> {
    let tag =
        service::tags::update_tag(&state, auth.user_id, ledger_id, tag_id, req).await?;
    Ok(Json(tag))
}

pub async fn delete(
    State(state): State<AppState>,
    auth: AuthUser,
    Path((ledger_id, tag_id)): Path<(Uuid, Uuid)>,
) -> AppResult<Json<serde_json::Value>> {
    service::tags::delete_tag(&state, auth.user_id, ledger_id, tag_id).await?;
    Ok(Json(serde_json::json!({ "message": "标签已删除" })))
}
