use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppResult,
    model::{LedgerModel, LedgerSummaryRow},
};

#[derive(Clone)]
pub struct LedgerRepository {
    pool: PgPool,
}

impl LedgerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Returns enriched summary rows for the ledger list page.
    pub async fn find_all_by_user(
        &self,
        user_id: Uuid,
        timezone: &str,
    ) -> AppResult<Vec<LedgerSummaryRow>> {
        // Compute balance (initial_balance + SUM income base_amounts - SUM expense base_amounts),
        // current-month income/expense in the user's timezone, and total active budget amount.
        let rows = sqlx::query_as!(
            LedgerSummaryRow,
            r#"
            WITH tx_agg AS (
                SELECT
                    t.ledger_id,
                    SUM(CASE WHEN t.type = 'income'  THEN t.base_amount ELSE 0 END) AS total_income,
                    SUM(CASE WHEN t.type = 'expense' THEN t.base_amount ELSE 0 END) AS total_expense,
                    SUM(CASE WHEN t.type = 'income'
                              AND date_trunc('month', t.occurred_at AT TIME ZONE $2) =
                                  date_trunc('month', NOW() AT TIME ZONE $2)
                              THEN t.base_amount ELSE 0 END) AS month_income,
                    SUM(CASE WHEN t.type = 'expense'
                              AND date_trunc('month', t.occurred_at AT TIME ZONE $2) =
                                  date_trunc('month', NOW() AT TIME ZONE $2)
                              THEN t.base_amount ELSE 0 END) AS month_expense
                FROM transactions t
                WHERE t.type IN ('income', 'expense')
                GROUP BY t.ledger_id
            ),
            budget_agg AS (
                SELECT ledger_id, SUM(amount) AS budget_total
                FROM budgets
                GROUP BY ledger_id
            )
            SELECT
                l.id,
                l.user_id,
                l.name,
                l.icon,
                l.base_currency,
                l.initial_balance,
                l.initial_balance
                    + COALESCE(ta.total_income,  0)
                    - COALESCE(ta.total_expense, 0) AS "balance!: Decimal",
                COALESCE(ta.month_income,  0) AS "this_month_income!: Decimal",
                COALESCE(ta.month_expense, 0) AS "this_month_expense!: Decimal",
                ba.budget_total                AS "budget_total?: Decimal",
                l.created_at,
                l.updated_at
            FROM ledgers l
            LEFT JOIN tx_agg    ta ON ta.ledger_id = l.id
            LEFT JOIN budget_agg ba ON ba.ledger_id = l.id
            WHERE l.user_id = $1
            ORDER BY l.created_at ASC
            "#,
            user_id,
            timezone
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Option<LedgerModel>> {
        let row = sqlx::query_as!(
            LedgerModel,
            "SELECT id, user_id, name, icon, base_currency, initial_balance, created_at, updated_at
             FROM ledgers WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn find_summary_by_id(
        &self,
        id: Uuid,
        timezone: &str,
    ) -> AppResult<Option<LedgerSummaryRow>> {
        let row = sqlx::query_as!(
            LedgerSummaryRow,
            r#"
            WITH tx_agg AS (
                SELECT
                    SUM(CASE WHEN type = 'income'  THEN base_amount ELSE 0 END) AS total_income,
                    SUM(CASE WHEN type = 'expense' THEN base_amount ELSE 0 END) AS total_expense,
                    SUM(CASE WHEN type = 'income'
                              AND date_trunc('month', occurred_at AT TIME ZONE $2) =
                                  date_trunc('month', NOW() AT TIME ZONE $2)
                              THEN base_amount ELSE 0 END) AS month_income,
                    SUM(CASE WHEN type = 'expense'
                              AND date_trunc('month', occurred_at AT TIME ZONE $2) =
                                  date_trunc('month', NOW() AT TIME ZONE $2)
                              THEN base_amount ELSE 0 END) AS month_expense
                FROM transactions
                WHERE ledger_id = $1 AND type IN ('income', 'expense')
            ),
            budget_agg AS (
                SELECT SUM(amount) AS budget_total
                FROM budgets
                WHERE ledger_id = $1
            )
            SELECT
                l.id,
                l.user_id,
                l.name,
                l.icon,
                l.base_currency,
                l.initial_balance,
                l.initial_balance
                    + COALESCE(ta.total_income,  0)
                    - COALESCE(ta.total_expense, 0) AS "balance!: Decimal",
                COALESCE(ta.month_income,  0) AS "this_month_income!: Decimal",
                COALESCE(ta.month_expense, 0) AS "this_month_expense!: Decimal",
                ba.budget_total                AS "budget_total?: Decimal",
                l.created_at,
                l.updated_at
            FROM ledgers l, tx_agg ta, budget_agg ba
            WHERE l.id = $1
            "#,
            id,
            timezone
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        name: &str,
        icon: &str,
        base_currency: &str,
        initial_balance: Decimal,
    ) -> AppResult<LedgerModel> {
        let row = sqlx::query_as!(
            LedgerModel,
            "INSERT INTO ledgers (user_id, name, icon, base_currency, initial_balance)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id, user_id, name, icon, base_currency, initial_balance, created_at, updated_at",
            user_id,
            name,
            icon,
            base_currency,
            initial_balance
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn update(
        &self,
        id: Uuid,
        name: Option<&str>,
        icon: Option<&str>,
        initial_balance: Option<Decimal>,
    ) -> AppResult<LedgerModel> {
        let row = sqlx::query_as!(
            LedgerModel,
            "UPDATE ledgers
             SET name            = COALESCE($2, name),
                 icon            = COALESCE($3, icon),
                 initial_balance = COALESCE($4, initial_balance),
                 updated_at      = NOW()
             WHERE id = $1
             RETURNING id, user_id, name, icon, base_currency, initial_balance, created_at, updated_at",
            id,
            name,
            icon,
            initial_balance
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query!("DELETE FROM ledgers WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn count_by_user(&self, user_id: Uuid) -> AppResult<i64> {
        let row = sqlx::query!(
            "SELECT COUNT(*) AS cnt FROM ledgers WHERE user_id = $1",
            user_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row.cnt.unwrap_or(0))
    }
}
