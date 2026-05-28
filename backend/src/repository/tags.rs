use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::AppResult, model::TagModel};

#[derive(Clone)]
pub struct TagRepository {
    pool: PgPool,
}

impl TagRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_ledger(&self, ledger_id: Uuid) -> AppResult<Vec<TagModel>> {
        let rows = sqlx::query_as!(
            TagModel,
            "SELECT id, ledger_id, name, created_at
             FROM tags WHERE ledger_id = $1
             ORDER BY name ASC",
            ledger_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Option<TagModel>> {
        let row = sqlx::query_as!(
            TagModel,
            "SELECT id, ledger_id, name, created_at FROM tags WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn find_by_ids(&self, ids: &[Uuid]) -> AppResult<Vec<TagModel>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let rows = sqlx::query_as!(
            TagModel,
            "SELECT id, ledger_id, name, created_at
             FROM tags WHERE id = ANY($1)",
            ids
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    /// Returns all tags attached to a given transaction.
    pub async fn find_by_transaction(&self, transaction_id: Uuid) -> AppResult<Vec<TagModel>> {
        let rows = sqlx::query_as!(
            TagModel,
            "SELECT t.id, t.ledger_id, t.name, t.created_at
             FROM tags t
             JOIN transaction_tags tt ON tt.tag_id = t.id
             WHERE tt.transaction_id = $1
             ORDER BY t.name ASC",
            transaction_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    /// Returns tags for multiple transactions at once; returns (transaction_id, TagModel) pairs.
    pub async fn find_by_transactions(
        &self,
        transaction_ids: &[Uuid],
    ) -> AppResult<Vec<(Uuid, TagModel)>> {
        if transaction_ids.is_empty() {
            return Ok(vec![]);
        }
        let rows = sqlx::query!(
            "SELECT tt.transaction_id, t.id, t.ledger_id, t.name, t.created_at
             FROM tags t
             JOIN transaction_tags tt ON tt.tag_id = t.id
             WHERE tt.transaction_id = ANY($1)
             ORDER BY t.name ASC",
            transaction_ids
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| {
                (
                    r.transaction_id,
                    TagModel {
                        id: r.id,
                        ledger_id: r.ledger_id,
                        name: r.name,
                        created_at: r.created_at,
                    },
                )
            })
            .collect())
    }

    pub async fn create(&self, ledger_id: Uuid, name: &str) -> AppResult<TagModel> {
        let row = sqlx::query_as!(
            TagModel,
            "INSERT INTO tags (ledger_id, name) VALUES ($1, $2)
             RETURNING id, ledger_id, name, created_at",
            ledger_id,
            name
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn update(&self, id: Uuid, name: &str) -> AppResult<TagModel> {
        let row = sqlx::query_as!(
            TagModel,
            "UPDATE tags SET name = $2 WHERE id = $1
             RETURNING id, ledger_id, name, created_at",
            id,
            name
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query!("DELETE FROM tags WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_transaction_tags(
        &self,
        transaction_id: Uuid,
        tag_ids: &[Uuid],
    ) -> AppResult<()> {
        sqlx::query!(
            "DELETE FROM transaction_tags WHERE transaction_id = $1",
            transaction_id
        )
        .execute(&self.pool)
        .await?;

        if tag_ids.is_empty() {
            return Ok(());
        }

        for tag_id in tag_ids {
            sqlx::query!(
                "INSERT INTO transaction_tags (transaction_id, tag_id) VALUES ($1, $2)
                 ON CONFLICT DO NOTHING",
                transaction_id,
                tag_id
            )
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }
}
