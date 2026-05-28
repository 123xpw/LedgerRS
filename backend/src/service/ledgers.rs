use rust_decimal::Decimal;
use uuid::Uuid;

use shared::dto::ledgers::{CreateLedgerRequest, LedgerResponse, UpdateLedgerRequest};
use shared::types::is_valid_currency;

use crate::{
    error::{AppError, AppResult},
    AppState,
};

fn to_response(row: &crate::model::LedgerSummaryRow) -> LedgerResponse {
    let budget_usage_rate = row.budget_total.and_then(|total| {
        if total.is_zero() {
            None
        } else {
            let rate = (row.this_month_expense / total * Decimal::from(100))
                .to_string()
                .parse::<f64>()
                .ok()?;
            Some(rate)
        }
    });
    LedgerResponse {
        id: row.id,
        user_id: row.user_id,
        name: row.name.clone(),
        icon: row.icon.clone(),
        base_currency: row.base_currency.clone(),
        initial_balance: row.initial_balance,
        balance: row.balance,
        this_month_income: row.this_month_income,
        this_month_expense: row.this_month_expense,
        budget_usage_rate,
        created_at: row.created_at,
        updated_at: row.updated_at,
    }
}

pub async fn list_ledgers(state: &AppState, user_id: Uuid, timezone: &str) -> AppResult<Vec<LedgerResponse>> {
    let rows = state.repos.ledgers.find_all_by_user(user_id, timezone).await?;
    Ok(rows.iter().map(to_response).collect())
}

pub async fn get_ledger(state: &AppState, user_id: Uuid, ledger_id: Uuid, timezone: &str) -> AppResult<LedgerResponse> {
    let row = state
        .repos
        .ledgers
        .find_summary_by_id(ledger_id, timezone)
        .await?
        .ok_or_else(|| AppError::NotFound("账本不存在".into()))?;
    if row.user_id != user_id {
        return Err(AppError::Forbidden("无权访问该账本".into()));
    }
    Ok(to_response(&row))
}

pub async fn create_ledger(
    state: &AppState,
    user_id: Uuid,
    req: CreateLedgerRequest,
) -> AppResult<LedgerResponse> {
    if !is_valid_currency(&req.base_currency) {
        return Err(AppError::Validation(format!("不支持的币种: {}", req.base_currency)));
    }
    let model = state
        .repos
        .ledgers
        .create(
            user_id,
            &req.name,
            &req.icon.unwrap_or_else(|| "💰".into()),
            &req.base_currency,
            req.initial_balance.unwrap_or(Decimal::ZERO),
        )
        .await?;

    // Re-fetch as summary row (balance = initial_balance, monthly = 0).
    let row = state
        .repos
        .ledgers
        .find_summary_by_id(model.id, "UTC")
        .await?
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("ledger vanished after insert")))?;
    Ok(to_response(&row))
}

pub async fn update_ledger(
    state: &AppState,
    user_id: Uuid,
    ledger_id: Uuid,
    req: UpdateLedgerRequest,
) -> AppResult<LedgerResponse> {
    let existing = state
        .repos
        .ledgers
        .find_by_id(ledger_id)
        .await?
        .ok_or_else(|| AppError::NotFound("账本不存在".into()))?;
    if existing.user_id != user_id {
        return Err(AppError::Forbidden("无权修改该账本".into()));
    }

    state
        .repos
        .ledgers
        .update(
            ledger_id,
            req.name.as_deref(),
            req.icon.as_deref(),
            req.initial_balance,
        )
        .await?;

    let row = state
        .repos
        .ledgers
        .find_summary_by_id(ledger_id, "UTC")
        .await?
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("ledger vanished after update")))?;
    Ok(to_response(&row))
}

pub async fn delete_ledger(state: &AppState, user_id: Uuid, ledger_id: Uuid) -> AppResult<()> {
    let existing = state
        .repos
        .ledgers
        .find_by_id(ledger_id)
        .await?
        .ok_or_else(|| AppError::NotFound("账本不存在".into()))?;
    if existing.user_id != user_id {
        return Err(AppError::Forbidden("无权删除该账本".into()));
    }
    let count = state.repos.ledgers.count_by_user(user_id).await?;
    if count <= 1 {
        return Err(AppError::LastLedger);
    }
    state.repos.ledgers.delete(ledger_id).await?;
    Ok(())
}
