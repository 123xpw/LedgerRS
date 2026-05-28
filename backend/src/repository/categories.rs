use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::AppResult, model::CategoryModel};

#[derive(Clone)]
pub struct CategoryRepository {
    pool: PgPool,
}

impl CategoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Returns system categories (ledger_id IS NULL) plus ledger-specific ones.
    pub async fn find_by_ledger(&self, ledger_id: Uuid) -> AppResult<Vec<CategoryModel>> {
        let rows = sqlx::query_as!(
            CategoryModel,
            r#"SELECT id, ledger_id, parent_id, name,
                      type AS category_type, is_system, sort_order, created_at
               FROM categories
               WHERE ledger_id = $1 OR ledger_id IS NULL
               ORDER BY sort_order ASC, created_at ASC"#,
            ledger_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Option<CategoryModel>> {
        let row = sqlx::query_as!(
            CategoryModel,
            r#"SELECT id, ledger_id, parent_id, name,
                      type AS category_type, is_system, sort_order, created_at
               FROM categories WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn create(
        &self,
        ledger_id: Uuid,
        parent_id: Option<Uuid>,
        name: &str,
        category_type: &str,
        sort_order: i32,
    ) -> AppResult<CategoryModel> {
        let row = sqlx::query_as!(
            CategoryModel,
            r#"INSERT INTO categories (ledger_id, parent_id, name, type, is_system, sort_order)
               VALUES ($1, $2, $3, $4, FALSE, $5)
               RETURNING id, ledger_id, parent_id, name,
                         type AS category_type, is_system, sort_order, created_at"#,
            ledger_id,
            parent_id,
            name,
            category_type,
            sort_order
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn update(
        &self,
        id: Uuid,
        name: Option<&str>,
        sort_order: Option<i32>,
    ) -> AppResult<CategoryModel> {
        let row = sqlx::query_as!(
            CategoryModel,
            r#"UPDATE categories
               SET name       = COALESCE($2, name),
                   sort_order = COALESCE($3, sort_order)
               WHERE id = $1
               RETURNING id, ledger_id, parent_id, name,
                         type AS category_type, is_system, sort_order, created_at"#,
            id,
            name,
            sort_order
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query!("DELETE FROM categories WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Count transactions referencing this category.
    pub async fn count_transactions(&self, id: Uuid) -> AppResult<i64> {
        let row = sqlx::query!(
            "SELECT COUNT(*) AS cnt FROM transactions WHERE category_id = $1",
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row.cnt.unwrap_or(0))
    }

    /// Reassign all transactions from one category to another (used with force delete).
    pub async fn reassign_transactions(
        &self,
        from_id: Uuid,
        to_id: Option<Uuid>,
    ) -> AppResult<()> {
        sqlx::query!(
            "UPDATE transactions SET category_id = $2 WHERE category_id = $1",
            from_id,
            to_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Max sort_order within a ledger (for appending new categories).
    pub async fn max_sort_order(&self, ledger_id: Uuid) -> AppResult<i32> {
        let row = sqlx::query!(
            "SELECT COALESCE(MAX(sort_order), 0) AS max_order
             FROM categories
             WHERE ledger_id = $1",
            ledger_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row.max_order.unwrap_or(0))
    }
}
