use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use shared::dto::budgets::{BudgetResponse, CreateBudgetRequest, UpdateBudgetRequest};

use crate::{error::AppResult, middleware::AuthUser, service, AppState};

pub async fn list(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(ledger_id): Path<Uuid>,
) -> AppResult<Json<Vec<BudgetResponse>>> {
    let user = state.repos.users.find_by_id(auth.user_id).await?.unwrap();
    let budgets =
        service::budgets::list_budgets(&state, auth.user_id, ledger_id, &user.timezone).await?;
    Ok(Json(budgets))
}

pub async fn create(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(ledger_id): Path<Uuid>,
    Json(req): Json<CreateBudgetRequest>,
) -> AppResult<Json<BudgetResponse>> {
    let budget =
        service::budgets::create_budget(&state, auth.user_id, ledger_id, req).await?;
    Ok(Json(budget))
}

pub async fn update(
    State(state): State<AppState>,
    auth: AuthUser,
    Path((ledger_id, budget_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<UpdateBudgetRequest>,
) -> AppResult<Json<BudgetResponse>> {
    let user = state.repos.users.find_by_id(auth.user_id).await?.unwrap();
    let budget = service::budgets::update_budget(
        &state,
        auth.user_id,
        ledger_id,
        budget_id,
        req,
        &user.timezone,
    )
    .await?;
    Ok(Json(budget))
}

pub async fn delete(
    State(state): State<AppState>,
    auth: AuthUser,
    Path((ledger_id, budget_id)): Path<(Uuid, Uuid)>,
) -> AppResult<Json<serde_json::Value>> {
    service::budgets::delete_budget(&state, auth.user_id, ledger_id, budget_id).await?;
    Ok(Json(serde_json::json!({ "message": "预算已删除" })))
}
