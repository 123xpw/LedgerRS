use rust_decimal::Decimal;
use uuid::Uuid;

use shared::dto::budgets::{BudgetResponse, CreateBudgetRequest, UpdateBudgetRequest};
use shared::types::Period;

use crate::{
    error::{AppError, AppResult},
    repository::budgets::BudgetWithSpendRow,
    AppState,
};

fn to_response(row: &BudgetWithSpendRow) -> BudgetResponse {
    let remaining = row.amount - row.spent;
    let usage_rate = if row.amount.is_zero() {
        0.0
    } else {
        (row.spent / row.amount * Decimal::from(100))
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0)
    };
    BudgetResponse {
        id: row.id,
        ledger_id: row.ledger_id,
        category_id: row.category_id,
        period: row.period.clone(),
        amount: row.amount,
        spent: row.spent,
        remaining,
        usage_rate,
        period_start: row.period_start,
        period_end: row.period_end,
        created_at: row.created_at,
        updated_at: row.updated_at,
    }
}

pub async fn list_budgets(
    state: &AppState,
    user_id: Uuid,
    ledger_id: Uuid,
    timezone: &str,
) -> AppResult<Vec<BudgetResponse>> {
    check_ledger_owner(state, user_id, ledger_id).await?;
    let rows = state.repos.budgets.find_with_spend(ledger_id, timezone).await?;
    Ok(rows.iter().map(to_response).collect())
}

pub async fn create_budget(
    state: &AppState,
    user_id: Uuid,
    ledger_id: Uuid,
    req: CreateBudgetRequest,
) -> AppResult<BudgetResponse> {
    check_ledger_owner(state, user_id, ledger_id).await?;

    let period_str = req.period.as_str();

    if req.amount <= Decimal::ZERO {
        return Err(AppError::Validation("预算金额必须大于零".into()));
    }

    let _ = Period::try_from(period_str)
        .map_err(|_| AppError::Validation("period must be week/month/year".into()))?;

    state
        .repos
        .budgets
        .create(ledger_id, req.category_id, period_str, req.amount)
        .await
        .map_err(|e| {
            // Unique constraint violation maps to Conflict.
            if let crate::error::AppError::Database(ref sqlx_err) = e {
                if sqlx_err.to_string().contains("uq_budget") {
                    return AppError::Conflict("该周期预算已存在".into());
                }
            }
            e
        })?;

    // Re-fetch with spend data.
    let rows = state.repos.budgets.find_with_spend(ledger_id, "UTC").await?;
    rows.into_iter()
        .find(|r| r.category_id == req.category_id && r.period == period_str)
        .map(|r| to_response(&r))
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("budget vanished after insert")))
}

pub async fn update_budget(
    state: &AppState,
    user_id: Uuid,
    ledger_id: Uuid,
    budget_id: Uuid,
    req: UpdateBudgetRequest,
    timezone: &str,
) -> AppResult<BudgetResponse> {
    check_ledger_owner(state, user_id, ledger_id).await?;
    let existing = state
        .repos
        .budgets
        .find_by_id(budget_id)
        .await?
        .ok_or_else(|| AppError::NotFound("预算不存在".into()))?;
    if existing.ledger_id != ledger_id {
        return Err(AppError::Forbidden("无权修改该预算".into()));
    }
    if req.amount <= Decimal::ZERO {
        return Err(AppError::Validation("预算金额必须大于零".into()));
    }

    state.repos.budgets.update(budget_id, req.amount).await?;

    let rows = state.repos.budgets.find_with_spend(ledger_id, timezone).await?;
    rows.into_iter()
        .find(|r| r.id == budget_id)
        .map(|r| to_response(&r))
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("budget vanished after update")))
}

pub async fn delete_budget(
    state: &AppState,
    user_id: Uuid,
    ledger_id: Uuid,
    budget_id: Uuid,
) -> AppResult<()> {
    check_ledger_owner(state, user_id, ledger_id).await?;
    let existing = state
        .repos
        .budgets
        .find_by_id(budget_id)
        .await?
        .ok_or_else(|| AppError::NotFound("预算不存在".into()))?;
    if existing.ledger_id != ledger_id {
        return Err(AppError::Forbidden("无权删除该预算".into()));
    }
    state.repos.budgets.delete(budget_id).await?;
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
