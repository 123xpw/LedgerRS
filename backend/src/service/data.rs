use uuid::Uuid;

use shared::dto::data::{
    BackupData, BackupLedger, BackupTransaction, RestoreResponse, RestoredCounts,
    BACKUP_SCHEMA_VERSION,
};

use crate::{
    error::{AppError, AppResult},
    AppState,
};

pub async fn export_backup(state: &AppState, user_id: Uuid) -> AppResult<BackupData> {
    let ledgers = state
        .repos
        .ledgers
        .find_all_by_user(user_id, "UTC")
        .await?;

    let mut backup_ledgers = Vec::new();
    for row in &ledgers {
        let transactions = state
            .repos
            .transactions
            .find_all_by_ledger(row.id)
            .await?;
        let categories = state
            .repos
            .categories
            .find_by_ledger(row.id)
            .await?
            .into_iter()
            .filter(|c| c.ledger_id == Some(row.id)) // exclude system categories
            .collect::<Vec<_>>();
        let tags = state.repos.tags.find_by_ledger(row.id).await?;
        let budgets = state.repos.budgets.find_all_by_ledger(row.id).await?;

        backup_ledgers.push(BackupLedger {
            id: row.id,
            name: row.name.clone(),
            icon: row.icon.clone(),
            base_currency: row.base_currency.clone(),
            initial_balance: row.initial_balance,
            created_at: row.created_at,
            updated_at: row.updated_at,
            categories: categories
                .iter()
                .map(|c| shared::dto::data::BackupCategory {
                    id: c.id,
                    parent_id: c.parent_id,
                    name: c.name.clone(),
                    category_type: c.category_type.clone(),
                    sort_order: c.sort_order,
                })
                .collect(),
            tags: tags
                .iter()
                .map(|t| shared::dto::data::BackupTag {
                    id: t.id,
                    name: t.name.clone(),
                })
                .collect(),
            transactions: transactions
                .iter()
                .map(|t| BackupTransaction {
                    id: t.id,
                    transaction_type: t.transaction_type.clone(),
                    amount: t.amount,
                    currency: t.currency.clone(),
                    base_amount: t.base_amount,
                    applied_rate: t.applied_rate,
                    category_id: t.category_id,
                    note: t.note.clone(),
                    occurred_at: t.occurred_at,
                    custom_rate: t.custom_rate,
                    transfer_pair_id: t.transfer_pair_id,
                    created_at: t.created_at,
                })
                .collect(),
            budgets: budgets
                .iter()
                .map(|b| shared::dto::data::BackupBudget {
                    id: b.id,
                    category_id: b.category_id,
                    period: b.period.clone(),
                    amount: b.amount,
                })
                .collect(),
        });
    }

    Ok(BackupData {
        schema_version: BACKUP_SCHEMA_VERSION.to_string(),
        exported_at: chrono::Utc::now(),
        user_id,
        ledgers: backup_ledgers,
    })
}

pub async fn import_backup(
    state: &AppState,
    user_id: Uuid,
    data: BackupData,
) -> AppResult<RestoreResponse> {
    if data.schema_version != BACKUP_SCHEMA_VERSION {
        return Err(AppError::InvalidBackupFormat(format!(
            "schema version mismatch: expected {}, got {}",
            BACKUP_SCHEMA_VERSION, data.schema_version
        )));
    }

    let mut counts = RestoredCounts::default();

    for bl in &data.ledgers {
        // Upsert ledger (skip if ID already exists for this user).
        let existing = state.repos.ledgers.find_by_id(bl.id).await?;
        if existing.is_none() {
            sqlx::query!(
                "INSERT INTO ledgers (id, user_id, name, icon, base_currency, initial_balance)
                 VALUES ($1, $2, $3, $4, $5, $6)
                 ON CONFLICT (id) DO NOTHING",
                bl.id,
                user_id,
                bl.name,
                bl.icon,
                bl.base_currency,
                bl.initial_balance
            )
            .execute(&state.db)
            .await?;
            counts.ledgers += 1;
        }

        // Categories.
        for bc in &bl.categories {
            sqlx::query!(
                "INSERT INTO categories (id, ledger_id, parent_id, name, type, is_system, sort_order)
                 VALUES ($1, $2, $3, $4, $5, FALSE, $6)
                 ON CONFLICT (id) DO NOTHING",
                bc.id,
                bl.id,
                bc.parent_id,
                bc.name,
                bc.category_type,
                bc.sort_order
            )
            .execute(&state.db)
            .await?;
            counts.categories += 1;
        }

        // Tags.
        for bt in &bl.tags {
            sqlx::query!(
                "INSERT INTO tags (id, ledger_id, name) VALUES ($1, $2, $3)
                 ON CONFLICT (id) DO NOTHING",
                bt.id,
                bl.id,
                bt.name
            )
            .execute(&state.db)
            .await?;
            counts.tags += 1;
        }

        // Transactions (ID-based deduplication).
        for bt in &bl.transactions {
            state
                .repos
                .transactions
                .insert_with_id(
                    bt.id,
                    bl.id,
                    &bt.transaction_type,
                    bt.amount,
                    &bt.currency,
                    bt.base_amount,
                    bt.applied_rate,
                    bt.category_id,
                    bt.note.as_deref(),
                    bt.occurred_at,
                    bt.custom_rate,
                    bt.transfer_pair_id,
                )
                .await?;
            counts.transactions += 1;
        }

        // Budgets.
        for bb in &bl.budgets {
            sqlx::query!(
                "INSERT INTO budgets (id, ledger_id, category_id, period, amount)
                 VALUES ($1, $2, $3, $4, $5)
                 ON CONFLICT (id) DO NOTHING",
                bb.id,
                bl.id,
                bb.category_id,
                bb.period,
                bb.amount
            )
            .execute(&state.db)
            .await?;
            counts.budgets += 1;
        }
    }

    Ok(RestoreResponse { counts })
}
