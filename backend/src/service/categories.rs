use uuid::Uuid;

use shared::dto::categories::{CategoryResponse, CreateCategoryRequest, UpdateCategoryRequest};

use crate::{
    error::{AppError, AppResult},
    AppState,
};

fn to_response(m: &crate::model::CategoryModel) -> CategoryResponse {
    CategoryResponse {
        id: m.id,
        ledger_id: m.ledger_id,
        parent_id: m.parent_id,
        name: m.name.clone(),
        category_type: m.category_type.clone(),
        is_system: m.is_system,
        sort_order: m.sort_order,
        created_at: m.created_at,
    }
}

pub async fn list_categories(
    state: &AppState,
    user_id: Uuid,
    ledger_id: Uuid,
) -> AppResult<Vec<CategoryResponse>> {
    check_ledger_owner(state, user_id, ledger_id).await?;
    let rows = state.repos.categories.find_by_ledger(ledger_id).await?;
    Ok(rows.iter().map(to_response).collect())
}

pub async fn create_category(
    state: &AppState,
    user_id: Uuid,
    ledger_id: Uuid,
    req: CreateCategoryRequest,
) -> AppResult<CategoryResponse> {
    check_ledger_owner(state, user_id, ledger_id).await?;

    let cat_type = req.category_type.as_str();
    if cat_type != "income" && cat_type != "expense" {
        return Err(AppError::Validation("category_type must be income or expense".into()));
    }

    // Validate parent belongs to same ledger (if provided).
    if let Some(parent_id) = req.parent_id {
        let parent = state.repos.categories.find_by_id(parent_id).await?
            .ok_or_else(|| AppError::NotFound("父分类不存在".into()))?;
        if parent.ledger_id != Some(ledger_id) {
            return Err(AppError::Validation("父分类不属于该账本".into()));
        }
    }

    let sort_order = req.sort_order.unwrap_or_else(|| {
        // Will be filled below; placeholder
        0
    });
    let sort_order = if sort_order == 0 {
        state.repos.categories.max_sort_order(ledger_id).await? + 1
    } else {
        sort_order
    };

    let row = state
        .repos
        .categories
        .create(ledger_id, req.parent_id, &req.name, cat_type, sort_order)
        .await?;
    Ok(to_response(&row))
}

pub async fn update_category(
    state: &AppState,
    user_id: Uuid,
    ledger_id: Uuid,
    category_id: Uuid,
    req: UpdateCategoryRequest,
) -> AppResult<CategoryResponse> {
    check_ledger_owner(state, user_id, ledger_id).await?;
    let cat = state
        .repos
        .categories
        .find_by_id(category_id)
        .await?
        .ok_or_else(|| AppError::NotFound("分类不存在".into()))?;
    if cat.is_system {
        return Err(AppError::Forbidden("系统分类不可修改".into()));
    }
    if cat.ledger_id != Some(ledger_id) {
        return Err(AppError::Forbidden("无权修改该分类".into()));
    }

    let row = state
        .repos
        .categories
        .update(category_id, req.name.as_deref(), req.sort_order)
        .await?;
    Ok(to_response(&row))
}

pub async fn delete_category(
    state: &AppState,
    user_id: Uuid,
    ledger_id: Uuid,
    category_id: Uuid,
    force: bool,
) -> AppResult<()> {
    check_ledger_owner(state, user_id, ledger_id).await?;
    let cat = state
        .repos
        .categories
        .find_by_id(category_id)
        .await?
        .ok_or_else(|| AppError::NotFound("分类不存在".into()))?;
    if cat.is_system {
        return Err(AppError::Forbidden("系统分类不可删除".into()));
    }
    if cat.ledger_id != Some(ledger_id) {
        return Err(AppError::Forbidden("无权删除该分类".into()));
    }

    let count = state.repos.categories.count_transactions(category_id).await?;
    if count > 0 && !force {
        return Err(AppError::CategoryHasTransactions { count });
    }
    if force {
        state.repos.categories.reassign_transactions(category_id, None).await?;
    }
    state.repos.categories.delete(category_id).await?;
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
