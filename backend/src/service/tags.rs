use uuid::Uuid;

use shared::dto::tags::{CreateTagRequest, TagResponse, UpdateTagRequest};

use crate::{
    error::{AppError, AppResult},
    AppState,
};

fn to_response(m: &crate::model::TagModel) -> TagResponse {
    TagResponse {
        id: m.id,
        ledger_id: m.ledger_id,
        name: m.name.clone(),
        created_at: m.created_at,
    }
}

pub async fn list_tags(state: &AppState, user_id: Uuid, ledger_id: Uuid) -> AppResult<Vec<TagResponse>> {
    check_ledger_owner(state, user_id, ledger_id).await?;
    let rows = state.repos.tags.find_by_ledger(ledger_id).await?;
    Ok(rows.iter().map(to_response).collect())
}

pub async fn create_tag(
    state: &AppState,
    user_id: Uuid,
    ledger_id: Uuid,
    req: CreateTagRequest,
) -> AppResult<TagResponse> {
    check_ledger_owner(state, user_id, ledger_id).await?;
    let row = state.repos.tags.create(ledger_id, &req.name).await?;
    Ok(to_response(&row))
}

pub async fn update_tag(
    state: &AppState,
    user_id: Uuid,
    ledger_id: Uuid,
    tag_id: Uuid,
    req: UpdateTagRequest,
) -> AppResult<TagResponse> {
    check_ledger_owner(state, user_id, ledger_id).await?;
    let tag = state
        .repos
        .tags
        .find_by_id(tag_id)
        .await?
        .ok_or_else(|| AppError::NotFound("标签不存在".into()))?;
    if tag.ledger_id != ledger_id {
        return Err(AppError::Forbidden("无权修改该标签".into()));
    }
    let row = state.repos.tags.update(tag_id, &req.name).await?;
    Ok(to_response(&row))
}

pub async fn delete_tag(
    state: &AppState,
    user_id: Uuid,
    ledger_id: Uuid,
    tag_id: Uuid,
) -> AppResult<()> {
    check_ledger_owner(state, user_id, ledger_id).await?;
    let tag = state
        .repos
        .tags
        .find_by_id(tag_id)
        .await?
        .ok_or_else(|| AppError::NotFound("标签不存在".into()))?;
    if tag.ledger_id != ledger_id {
        return Err(AppError::Forbidden("无权删除该标签".into()));
    }
    state.repos.tags.delete(tag_id).await?;
    Ok(())
}

async fn check_ledger_owner(state: &AppState, user_id: Uuid, ledger_id: Uuid) -> AppResult<()> {
    let ledger = state
        .repos
        .ledgers
        .find_by_id(ledger_id)
        .await?
        .ok_or_else(|| AppError::NotFound("账本不存在".into()))?;
    if ledger.user_id != user_id {
        return Err(AppError::Forbidden("无权访问该账本".into()));
    }
    Ok(())
}
