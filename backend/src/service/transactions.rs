use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

use shared::dto::transactions::{
    CreateTransactionRequest, TransactionResponse, TransferPeerInfo, UpdateTransactionRequest,
};
use shared::dto::pagination::{PaginatedResponse, PaginationMeta};
use shared::types::{is_valid_currency, TransactionType};

use crate::{
    error::{AppError, AppResult},
    model::TransactionWithCategoryRow,
    repository::transactions::TransactionFilter,
    AppState,
};

pub struct ListParams {
    pub category_id: Option<Uuid>,
    pub tag_id: Option<Uuid>,
    pub transaction_type: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub keyword: Option<String>,
    pub page: i64,
    pub per_page: i64,
}

pub async fn list_transactions(
    state: &AppState,
    user_id: Uuid,
    ledger_id: Uuid,
    params: ListParams,
) -> AppResult<PaginatedResponse<TransactionResponse>> {
    check_ledger_owner(state, user_id, ledger_id).await?;

    let offset = (params.page - 1) * params.per_page;
    let filter = TransactionFilter {
        category_id: params.category_id,
        tag_id: params.tag_id,
        transaction_type: params.transaction_type,
        start_date: params.start_date,
        end_date: params.end_date,
        keyword: params.keyword,
        limit: params.per_page,
        offset,
    };

    let (rows, total) = state
        .repos
        .transactions
        .find_paginated(ledger_id, &filter)
        .await?;

    let tx_ids: Vec<Uuid> = rows.iter().map(|r| r.id).collect();
    let tag_pairs = state.repos.tags.find_by_transactions(&tx_ids).await?;

    let items = rows
        .iter()
        .map(|row| {
            let tags = tag_pairs
                .iter()
                .filter(|(tid, _)| *tid == row.id)
                .map(|(_, t)| shared::dto::tags::TagResponse {
                    id: t.id,
                    ledger_id: t.ledger_id,
                    name: t.name.clone(),
                    created_at: t.created_at,
                })
                .collect();
            row_to_response(row, tags)
        })
        .collect();

    Ok(PaginatedResponse {
        items,
        meta: PaginationMeta {
            page: params.page,
            per_page: params.per_page,
            total,
            total_pages: (total + params.per_page - 1) / params.per_page,
        },
    })
}

pub async fn get_transaction(
    state: &AppState,
    user_id: Uuid,
    ledger_id: Uuid,
    tx_id: Uuid,
) -> AppResult<TransactionResponse> {
    check_ledger_owner(state, user_id, ledger_id).await?;
    let row = state
        .repos
        .transactions
        .find_by_id(tx_id)
        .await?
        .ok_or_else(|| AppError::NotFound("账目不存在".into()))?;
    if row.ledger_id != ledger_id {
        return Err(AppError::Forbidden("无权访问该账目".into()));
    }
    let tags = state.repos.tags.find_by_transaction(tx_id).await?;
    let tags = tags
        .iter()
        .map(|t| shared::dto::tags::TagResponse {
            id: t.id,
            ledger_id: t.ledger_id,
            name: t.name.clone(),
            created_at: t.created_at,
        })
        .collect();
    Ok(row_to_response(&row, tags))
}

pub async fn create_transaction(
    state: &AppState,
    user_id: Uuid,
    ledger_id: Uuid,
    req: CreateTransactionRequest,
) -> AppResult<TransactionResponse> {
    check_ledger_owner(state, user_id, ledger_id).await?;

    if !is_valid_currency(&req.currency) {
        return Err(AppError::Validation(format!("不支持的币种: {}", req.currency)));
    }

    let ledger = state
        .repos
        .ledgers
        .find_by_id(ledger_id)
        .await?
        .ok_or_else(|| AppError::NotFound("账本不存在".into()))?;

    match req.transaction_type {
        TransactionType::Transfer => {
            create_transfer(state, ledger_id, &ledger.base_currency, user_id, req).await
        }
        _ => {
            create_regular(state, ledger_id, &ledger.base_currency, req).await
        }
    }
}

async fn create_regular(
    state: &AppState,
    ledger_id: Uuid,
    base_currency: &str,
    req: CreateTransactionRequest,
) -> AppResult<TransactionResponse> {
    let (_base_amount, applied_rate) = resolve_rate(
        state,
        &req.currency,
        base_currency,
        req.occurred_at,
        req.custom_rate,
    )
    .await?;

    let tx_type_str = match req.transaction_type {
        TransactionType::Income => "income",
        TransactionType::Expense => "expense",
        TransactionType::Transfer => unreachable!(),
    };

    let row = state
        .repos
        .transactions
        .create(
            ledger_id,
            tx_type_str,
            req.amount,
            &req.currency,
            req.amount * applied_rate,
            applied_rate,
            req.category_id,
            req.note.as_deref(),
            req.occurred_at,
            req.custom_rate,
            None,
        )
        .await?;

    state
        .repos
        .tags
        .set_transaction_tags(row.id, &req.tag_ids)
        .await?;

    let full = state
        .repos
        .transactions
        .find_by_id(row.id)
        .await?
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("tx vanished")))?;
    let tags = state.repos.tags.find_by_transaction(row.id).await?;
    let tags = tags
        .iter()
        .map(|t| shared::dto::tags::TagResponse {
            id: t.id,
            ledger_id: t.ledger_id,
            name: t.name.clone(),
            created_at: t.created_at,
        })
        .collect();
    Ok(row_to_response(&full, tags))
}

async fn create_transfer(
    state: &AppState,
    source_ledger_id: Uuid,
    source_base_currency: &str,
    user_id: Uuid,
    req: CreateTransactionRequest,
) -> AppResult<TransactionResponse> {
    let target_ledger_id = req
        .target_ledger_id
        .ok_or_else(|| AppError::Validation("transfer requires target_ledger_id".into()))?;

    // Verify target ledger belongs to user.
    let target_ledger = state
        .repos
        .ledgers
        .find_by_id(target_ledger_id)
        .await?
        .ok_or_else(|| AppError::NotFound("目标账本不存在".into()))?;
    if target_ledger.user_id != user_id {
        return Err(AppError::Forbidden("无权访问目标账本".into()));
    }

    let target_currency = req
        .target_currency
        .as_deref()
        .unwrap_or(&target_ledger.base_currency);
    let target_amount = req.target_amount.unwrap_or(req.amount);

    let pair_id = Uuid::new_v4();

    let (source_base, source_rate) = resolve_rate(
        state,
        &req.currency,
        source_base_currency,
        req.occurred_at,
        req.custom_rate,
    )
    .await?;
    let _ = source_base; // used indirectly

    // Source side (debit from source ledger).
    let src_row = state
        .repos
        .transactions
        .create(
            source_ledger_id,
            "transfer",
            req.amount,
            &req.currency,
            req.amount * source_rate,
            source_rate,
            None,
            req.note.as_deref(),
            req.occurred_at,
            req.custom_rate,
            Some(pair_id),
        )
        .await?;

    // Target side (credit to target ledger).
    let (target_base_amount, target_rate) = resolve_rate(
        state,
        target_currency,
        &target_ledger.base_currency,
        req.occurred_at,
        None,
    )
    .await?;
    let _ = target_base_amount;

    state
        .repos
        .transactions
        .create(
            target_ledger_id,
            "transfer",
            target_amount,
            target_currency,
            target_amount * target_rate,
            target_rate,
            None,
            req.note.as_deref(),
            req.occurred_at,
            None,
            Some(pair_id),
        )
        .await?;

    let full = state
        .repos
        .transactions
        .find_by_id(src_row.id)
        .await?
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("tx vanished")))?;
    Ok(row_to_response(&full, vec![]))
}

pub async fn update_transaction(
    state: &AppState,
    user_id: Uuid,
    ledger_id: Uuid,
    tx_id: Uuid,
    req: UpdateTransactionRequest,
) -> AppResult<TransactionResponse> {
    check_ledger_owner(state, user_id, ledger_id).await?;
    let existing = state
        .repos
        .transactions
        .find_by_id(tx_id)
        .await?
        .ok_or_else(|| AppError::NotFound("账目不存在".into()))?;
    if existing.ledger_id != ledger_id {
        return Err(AppError::Forbidden("无权修改该账目".into()));
    }

    // Recompute base_amount if occurred_at or custom_rate changed.
    let (new_base, new_rate) = if req.occurred_at.is_some() || req.custom_rate.is_some() {
        let ledger = state.repos.ledgers.find_by_id(ledger_id).await?.unwrap();
        let occurred = req.occurred_at.unwrap_or(existing.occurred_at);
        let custom = req.custom_rate.as_ref().and_then(|r| *r);
        let (_, rate) = resolve_rate(
            state,
            &existing.currency,
            &ledger.base_currency,
            occurred,
            custom,
        )
        .await?;
        (Some(existing.amount * rate), Some(rate))
    } else {
        (None, None)
    };

    state
        .repos
        .transactions
        .update(
            tx_id,
            req.category_id,
            req.note,
            req.occurred_at,
            req.custom_rate,
            new_base,
            new_rate,
        )
        .await?;

    if let Some(tag_ids) = req.tag_ids {
        state.repos.tags.set_transaction_tags(tx_id, &tag_ids).await?;
    }

    let full = state.repos.transactions.find_by_id(tx_id).await?.unwrap();
    let tags = state.repos.tags.find_by_transaction(tx_id).await?;
    let tags = tags
        .iter()
        .map(|t| shared::dto::tags::TagResponse {
            id: t.id,
            ledger_id: t.ledger_id,
            name: t.name.clone(),
            created_at: t.created_at,
        })
        .collect();
    Ok(row_to_response(&full, tags))
}

pub async fn delete_transaction(
    state: &AppState,
    user_id: Uuid,
    ledger_id: Uuid,
    tx_id: Uuid,
) -> AppResult<()> {
    check_ledger_owner(state, user_id, ledger_id).await?;
    let existing = state
        .repos
        .transactions
        .find_by_id(tx_id)
        .await?
        .ok_or_else(|| AppError::NotFound("账目不存在".into()))?;
    if existing.ledger_id != ledger_id {
        return Err(AppError::Forbidden("无权删除该账目".into()));
    }

    if let Some(pair_id) = existing.transfer_pair_id {
        state.repos.transactions.delete_transfer_pair(pair_id).await?;
    } else {
        state.repos.transactions.delete(tx_id).await?;
    }
    Ok(())
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

async fn resolve_rate(
    state: &AppState,
    from: &str,
    to: &str,
    date: DateTime<Utc>,
    custom_rate: Option<Decimal>,
) -> AppResult<(Decimal, Decimal)> {
    if from == to {
        return Ok((Decimal::ONE, Decimal::ONE));
    }
    if let Some(r) = custom_rate {
        return Ok((Decimal::ONE, r));
    }
    let d = date.date_naive();
    let rate = state
        .repos
        .exchange_rates
        .find_rate(from, to, d)
        .await?
        .ok_or(AppError::ExchangeRateUnavailable)?;
    Ok((rate, rate))
}

fn row_to_response(
    row: &TransactionWithCategoryRow,
    tags: Vec<shared::dto::tags::TagResponse>,
) -> TransactionResponse {
    let transfer_peer = match (row.peer_ledger_id, row.peer_amount, row.peer_currency.as_ref()) {
        (Some(lid), Some(amt), Some(cur)) => Some(TransferPeerInfo {
            ledger_id: lid,
            amount: amt,
            currency: cur.clone(),
        }),
        _ => None,
    };
    TransactionResponse {
        id: row.id,
        ledger_id: row.ledger_id,
        transaction_type: row.transaction_type.clone(),
        amount: row.amount,
        currency: row.currency.clone(),
        base_amount: row.base_amount,
        exchange_rate: row.applied_rate,
        category_id: row.category_id,
        category_name: row.cat_name.clone(),
        category_type: row.cat_type.clone(),
        tags,
        note: row.note.clone(),
        occurred_at: row.occurred_at,
        custom_rate: row.custom_rate,
        transfer_pair_id: row.transfer_pair_id,
        transfer_peer,
        created_at: row.created_at,
        updated_at: row.updated_at,
    }
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
