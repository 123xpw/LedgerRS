use axum::{
    extract::{Path, State},
    http::header,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use shared::dto::data::{BackupData, RestoreResponse};

use crate::{error::AppResult, middleware::AuthUser, service, AppState};

pub async fn export(
    State(state): State<AppState>,
    auth: AuthUser,
) -> AppResult<Json<BackupData>> {
    let data = service::data::export_backup(&state, auth.user_id).await?;
    Ok(Json(data))
}

pub async fn import(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(data): Json<BackupData>,
) -> AppResult<Json<RestoreResponse>> {
    let resp = service::data::import_backup(&state, auth.user_id, data).await?;
    Ok(Json(resp))
}

pub async fn export_csv(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(ledger_id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    use crate::error::AppError;

    let ledger = state
        .repos
        .ledgers
        .find_by_id(ledger_id)
        .await?
        .ok_or_else(|| AppError::NotFound("账本不存在".into()))?;
    if ledger.user_id != auth.user_id {
        return Err(AppError::Forbidden("无权访问该账本".into()));
    }

    let transactions = state.repos.transactions.find_all_by_ledger(ledger_id).await?;

    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.write_record(&[
        "id",
        "type",
        "amount",
        "currency",
        "base_amount",
        "applied_rate",
        "note",
        "occurred_at",
    ])
    .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

    for tx in &transactions {
        wtr.write_record(&[
            tx.id.to_string(),
            tx.transaction_type.clone(),
            tx.amount.to_string(),
            tx.currency.clone(),
            tx.base_amount.to_string(),
            tx.applied_rate.to_string(),
            tx.note.clone().unwrap_or_default(),
            tx.occurred_at.to_rfc3339(),
        ])
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
    }

    let data = wtr
        .into_inner()
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

    let filename = format!("ledger_{}.csv", ledger_id);
    Ok((
        [
            (
                header::CONTENT_TYPE,
                "text/csv; charset=utf-8".to_string(),
            ),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{filename}\""),
            ),
        ],
        data,
    ))
}

pub async fn export_excel(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(ledger_id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    use crate::error::AppError;
    use rust_xlsxwriter::Workbook;

    let ledger = state
        .repos
        .ledgers
        .find_by_id(ledger_id)
        .await?
        .ok_or_else(|| AppError::NotFound("账本不存在".into()))?;
    if ledger.user_id != auth.user_id {
        return Err(AppError::Forbidden("无权访问该账本".into()));
    }

    let transactions = state.repos.transactions.find_all_by_ledger(ledger_id).await?;

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();
    ws.write_row(0, 0, ["ID", "类型", "金额", "币种", "换算金额", "汇率", "备注", "时间"])
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

    for (i, tx) in transactions.iter().enumerate() {
        let row = (i + 1) as u32;
        ws.write(row, 0, tx.id.to_string())
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
        ws.write(row, 1, tx.transaction_type.as_str())
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
        ws.write(row, 2, tx.amount.to_string())
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
        ws.write(row, 3, tx.currency.as_str())
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
        ws.write(row, 4, tx.base_amount.to_string())
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
        ws.write(row, 5, tx.applied_rate.to_string())
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
        ws.write(row, 6, tx.note.as_deref().unwrap_or(""))
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
        ws.write(row, 7, tx.occurred_at.to_rfc3339())
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
    }

    let data = wb
        .save_to_buffer()
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

    let filename = format!("ledger_{}.xlsx", ledger_id);
    Ok((
        [
            (
                header::CONTENT_TYPE,
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string(),
            ),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{filename}\""),
            ),
        ],
        data,
    ))
}
