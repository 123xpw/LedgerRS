use chrono::NaiveDate;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::AppResult, model::BudgetModel};

/// Budget row joined with its current-period spend.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct BudgetWithSpendRow {
    pub id: Uuid,
    pub ledger_id: Uuid,
    pub category_id: Option<Uuid>,
    pub period: String,
    pub amount: Decimal,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Sum of base_amount for expense transactions in the current period.
    pub spent: Decimal,
    /// Inclusive start of the current period window (in UTC midnight).
    pub period_start: NaiveDate,
    /// Inclusive end of the current period window.
    pub period_end: NaiveDate,
}

#[derive(Clone)]
pub struct BudgetRepository {
    pool: PgPool,
}

impl BudgetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Returns all budgets for a ledger with their current-period spent amount.
    pub async fn find_with_spend(
        &self,
        ledger_id: Uuid,
        timezone: &str,
    ) -> AppResult<Vec<BudgetWithSpendRow>> {
        // Period windows are computed in the user's timezone, then converted to UTC for comparison.
        // week  → Monday–Sunday of current week
        // month → 1st–last day of current month
        // year  → Jan 1 – Dec 31 of current year
        let rows = sqlx::query_as!(
            BudgetWithSpendRow,
            r#"WITH period_bounds AS (
                SELECT
                    b.id,
                    CASE b.period
                        WHEN 'week'  THEN date_trunc('week',  NOW() AT TIME ZONE $2)::date
                        WHEN 'month' THEN date_trunc('month', NOW() AT TIME ZONE $2)::date
                        WHEN 'year'  THEN date_trunc('year',  NOW() AT TIME ZONE $2)::date
                    END AS period_start,
                    CASE b.period
                        WHEN 'week'  THEN (date_trunc('week',  NOW() AT TIME ZONE $2) + INTERVAL '6 days')::date
                        WHEN 'month' THEN (date_trunc('month', NOW() AT TIME ZONE $2) + INTERVAL '1 month' - INTERVAL '1 day')::date
                        WHEN 'year'  THEN (date_trunc('year',  NOW() AT TIME ZONE $2) + INTERVAL '1 year'  - INTERVAL '1 day')::date
                    END AS period_end
                FROM budgets b
                WHERE b.ledger_id = $1
            ),
            spend_agg AS (
                SELECT
                    b.id AS budget_id,
                    COALESCE(SUM(t.base_amount), 0) AS spent
                FROM budgets b
                JOIN period_bounds pb ON pb.id = b.id
                LEFT JOIN transactions t
                    ON t.ledger_id = b.ledger_id
                    AND t.type = 'expense'
                    AND (b.category_id IS NULL OR t.category_id = b.category_id)
                    AND (t.occurred_at AT TIME ZONE $2)::date >= pb.period_start
                    AND (t.occurred_at AT TIME ZONE $2)::date <= pb.period_end
                WHERE b.ledger_id = $1
                GROUP BY b.id
            )
            SELECT
                b.id,
                b.ledger_id,
                b.category_id,
                b.period,
                b.amount,
                b.created_at,
                b.updated_at,
                sa.spent                        AS "spent!: Decimal",
                pb.period_start                 AS "period_start!: NaiveDate",
                pb.period_end                   AS "period_end!: NaiveDate"
            FROM budgets b
            JOIN period_bounds pb ON pb.id = b.id
            JOIN spend_agg     sa ON sa.budget_id = b.id
            WHERE b.ledger_id = $1
            ORDER BY b.created_at ASC"#,
            ledger_id,
            timezone
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Option<BudgetModel>> {
        let row = sqlx::query_as!(
            BudgetModel,
            "SELECT id, ledger_id, category_id, period, amount, created_at, updated_at
             FROM budgets WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn create(
        &self,
        ledger_id: Uuid,
        category_id: Option<Uuid>,
        period: &str,
        amount: Decimal,
    ) -> AppResult<BudgetModel> {
        let row = sqlx::query_as!(
            BudgetModel,
            "INSERT INTO budgets (ledger_id, category_id, period, amount)
             VALUES ($1, $2, $3, $4)
             RETURNING id, ledger_id, category_id, period, amount, created_at, updated_at",
            ledger_id,
            category_id,
            period,
            amount
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn update(&self, id: Uuid, amount: Decimal) -> AppResult<BudgetModel> {
        let row = sqlx::query_as!(
            BudgetModel,
            "UPDATE budgets SET amount = $2, updated_at = NOW()
             WHERE id = $1
             RETURNING id, ledger_id, category_id, period, amount, created_at, updated_at",
            id,
            amount
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query!("DELETE FROM budgets WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn find_all_by_ledger(&self, ledger_id: Uuid) -> AppResult<Vec<BudgetModel>> {
        let rows = sqlx::query_as!(
            BudgetModel,
            "SELECT id, ledger_id, category_id, period, amount, created_at, updated_at
             FROM budgets WHERE ledger_id = $1
             ORDER BY created_at ASC",
            ledger_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }
}
