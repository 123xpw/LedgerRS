use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::AppResult, model::TransactionModel, model::TransactionWithCategoryRow};

#[derive(Clone)]
pub struct TransactionRepository {
    pool: PgPool,
}

/// Filter parameters for the transaction list query.
#[derive(Default)]
pub struct TransactionFilter {
    pub category_id: Option<Uuid>,
    pub tag_id: Option<Uuid>,
    pub transaction_type: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub keyword: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

impl TransactionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Option<TransactionWithCategoryRow>> {
        let row = sqlx::query_as!(
            TransactionWithCategoryRow,
            r#"SELECT
                t.id, t.ledger_id,
                t.type AS transaction_type,
                t.amount, t.currency, t.base_amount, t.applied_rate,
                t.category_id, t.note, t.occurred_at,
                t.custom_rate, t.transfer_pair_id, t.created_at, t.updated_at,
                c.name     AS "cat_name: Option<String>",
                c.type     AS "cat_type: Option<String>",
                peer.ledger_id  AS "peer_ledger_id: Option<Uuid>",
                peer.amount     AS "peer_amount: Option<Decimal>",
                peer.currency   AS "peer_currency: Option<String>"
            FROM transactions t
            LEFT JOIN categories c ON c.id = t.category_id
            LEFT JOIN transactions peer
                ON peer.transfer_pair_id = t.transfer_pair_id
                AND peer.id <> t.id
                AND t.transfer_pair_id IS NOT NULL
            WHERE t.id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn find_paginated(
        &self,
        ledger_id: Uuid,
        f: &TransactionFilter,
    ) -> AppResult<(Vec<TransactionWithCategoryRow>, i64)> {
        // Count query for pagination.
        let total: i64 = sqlx::query_scalar!(
            r#"SELECT COUNT(*) AS "cnt!"
               FROM transactions t
               LEFT JOIN transaction_tags tt ON tt.transaction_id = t.id
               WHERE t.ledger_id = $1
                 AND ($2::uuid IS NULL OR t.category_id = $2)
                 AND ($3::uuid IS NULL OR tt.tag_id = $3)
                 AND ($4::text IS NULL OR t.type = $4)
                 AND ($5::timestamptz IS NULL OR t.occurred_at >= $5)
                 AND ($6::timestamptz IS NULL OR t.occurred_at <= $6)
                 AND ($7::text IS NULL OR
                      to_tsvector('simple', COALESCE(t.note, '')) @@ plainto_tsquery('simple', $7))"#,
            ledger_id,
            f.category_id,
            f.tag_id,
            f.transaction_type,
            f.start_date,
            f.end_date,
            f.keyword
        )
        .fetch_one(&self.pool)
        .await?;

        let rows = sqlx::query_as!(
            TransactionWithCategoryRow,
            r#"SELECT DISTINCT ON (t.occurred_at, t.id)
                t.id, t.ledger_id,
                t.type AS transaction_type,
                t.amount, t.currency, t.base_amount, t.applied_rate,
                t.category_id, t.note, t.occurred_at,
                t.custom_rate, t.transfer_pair_id, t.created_at, t.updated_at,
                c.name     AS "cat_name: Option<String>",
                c.type     AS "cat_type: Option<String>",
                peer.ledger_id  AS "peer_ledger_id: Option<Uuid>",
                peer.amount     AS "peer_amount: Option<Decimal>",
                peer.currency   AS "peer_currency: Option<String>"
            FROM transactions t
            LEFT JOIN categories c ON c.id = t.category_id
            LEFT JOIN transaction_tags tt ON tt.transaction_id = t.id
            LEFT JOIN transactions peer
                ON peer.transfer_pair_id = t.transfer_pair_id
                AND peer.id <> t.id
                AND t.transfer_pair_id IS NOT NULL
            WHERE t.ledger_id = $1
              AND ($2::uuid IS NULL OR t.category_id = $2)
              AND ($3::uuid IS NULL OR tt.tag_id = $3)
              AND ($4::text IS NULL OR t.type = $4)
              AND ($5::timestamptz IS NULL OR t.occurred_at >= $5)
              AND ($6::timestamptz IS NULL OR t.occurred_at <= $6)
              AND ($7::text IS NULL OR
                   to_tsvector('simple', COALESCE(t.note, '')) @@ plainto_tsquery('simple', $7))
            ORDER BY t.occurred_at DESC, t.id
            LIMIT $8 OFFSET $9"#,
            ledger_id,
            f.category_id,
            f.tag_id,
            f.transaction_type,
            f.start_date,
            f.end_date,
            f.keyword,
            f.limit,
            f.offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok((rows, total))
    }

    pub async fn create(
        &self,
        ledger_id: Uuid,
        transaction_type: &str,
        amount: Decimal,
        currency: &str,
        base_amount: Decimal,
        applied_rate: Decimal,
        category_id: Option<Uuid>,
        note: Option<&str>,
        occurred_at: DateTime<Utc>,
        custom_rate: Option<Decimal>,
        transfer_pair_id: Option<Uuid>,
    ) -> AppResult<TransactionModel> {
        let row = sqlx::query_as!(
            TransactionModel,
            r#"INSERT INTO transactions
                (ledger_id, type, amount, currency, base_amount, applied_rate,
                 category_id, note, occurred_at, custom_rate, transfer_pair_id)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
               RETURNING id, ledger_id,
                         type AS transaction_type,
                         amount, currency, base_amount, applied_rate,
                         category_id, note, occurred_at, custom_rate,
                         transfer_pair_id, created_at, updated_at"#,
            ledger_id,
            transaction_type,
            amount,
            currency,
            base_amount,
            applied_rate,
            category_id,
            note,
            occurred_at,
            custom_rate,
            transfer_pair_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn update(
        &self,
        id: Uuid,
        category_id: Option<Option<Uuid>>,
        note: Option<Option<String>>,
        occurred_at: Option<DateTime<Utc>>,
        custom_rate: Option<Option<Decimal>>,
        base_amount: Option<Decimal>,
        applied_rate: Option<Decimal>,
    ) -> AppResult<TransactionModel> {
        // Build update for nullable fields using CASE expressions.
        let row = sqlx::query_as!(
            TransactionModel,
            r#"UPDATE transactions SET
                category_id  = CASE WHEN $2 THEN $3 ELSE category_id END,
                note         = CASE WHEN $4 THEN $5 ELSE note END,
                occurred_at  = COALESCE($6, occurred_at),
                custom_rate  = CASE WHEN $7 THEN $8 ELSE custom_rate END,
                base_amount  = COALESCE($9, base_amount),
                applied_rate = COALESCE($10, applied_rate),
                updated_at   = NOW()
               WHERE id = $1
               RETURNING id, ledger_id,
                         type AS transaction_type,
                         amount, currency, base_amount, applied_rate,
                         category_id, note, occurred_at, custom_rate,
                         transfer_pair_id, created_at, updated_at"#,
            id,
            category_id.is_some(),
            category_id.flatten(),
            note.is_some(),
            note.as_ref().and_then(|n| n.as_deref().map(String::from)),
            occurred_at,
            custom_rate.is_some(),
            custom_rate.flatten(),
            base_amount,
            applied_rate
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query!("DELETE FROM transactions WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Delete both sides of a transfer pair.
    pub async fn delete_transfer_pair(&self, transfer_pair_id: Uuid) -> AppResult<()> {
        sqlx::query!(
            "DELETE FROM transactions WHERE transfer_pair_id = $1",
            transfer_pair_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn find_by_transfer_pair(
        &self,
        transfer_pair_id: Uuid,
    ) -> AppResult<Vec<TransactionModel>> {
        let rows = sqlx::query_as!(
            TransactionModel,
            r#"SELECT id, ledger_id,
                      type AS transaction_type,
                      amount, currency, base_amount, applied_rate,
                      category_id, note, occurred_at, custom_rate,
                      transfer_pair_id, created_at, updated_at
               FROM transactions WHERE transfer_pair_id = $1"#,
            transfer_pair_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    /// Find all transactions in a ledger (for backup export).
    pub async fn find_all_by_ledger(&self, ledger_id: Uuid) -> AppResult<Vec<TransactionModel>> {
        let rows = sqlx::query_as!(
            TransactionModel,
            r#"SELECT id, ledger_id,
                      type AS transaction_type,
                      amount, currency, base_amount, applied_rate,
                      category_id, note, occurred_at, custom_rate,
                      transfer_pair_id, created_at, updated_at
               FROM transactions WHERE ledger_id = $1
               ORDER BY occurred_at ASC"#,
            ledger_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    /// Insert a transaction record with an explicit ID (used in backup restore).
    pub async fn insert_with_id(
        &self,
        id: Uuid,
        ledger_id: Uuid,
        transaction_type: &str,
        amount: Decimal,
        currency: &str,
        base_amount: Decimal,
        applied_rate: Decimal,
        category_id: Option<Uuid>,
        note: Option<&str>,
        occurred_at: DateTime<Utc>,
        custom_rate: Option<Decimal>,
        transfer_pair_id: Option<Uuid>,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"INSERT INTO transactions
                (id, ledger_id, type, amount, currency, base_amount, applied_rate,
                 category_id, note, occurred_at, custom_rate, transfer_pair_id)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
               ON CONFLICT (id) DO NOTHING"#,
            id,
            ledger_id,
            transaction_type,
            amount,
            currency,
            base_amount,
            applied_rate,
            category_id,
            note,
            occurred_at,
            custom_rate,
            transfer_pair_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
