use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

/// Monthly income/expense totals for a ledger.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MonthlyStatsRow {
    pub year_month: String,
    pub income: Decimal,
    pub expense: Decimal,
}

/// Category breakdown row.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CategoryBreakdownRow {
    pub category_id: Option<Uuid>,
    pub category_name: Option<String>,
    pub total: Decimal,
    pub count: i64,
}

/// Overview stats for a single ledger in a given date range.
#[derive(Debug, Clone)]
pub struct OverviewStats {
    pub total_income: Decimal,
    pub total_expense: Decimal,
    pub net: Decimal,
    pub transaction_count: i64,
}

use crate::error::AppResult;

#[derive(Clone)]
pub struct ReportRepository {
    pool: PgPool,
}

impl ReportRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn overview(
        &self,
        ledger_id: Uuid,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> AppResult<OverviewStats> {
        let row = sqlx::query!(
            r#"SELECT
                COALESCE(SUM(CASE WHEN type = 'income'  THEN base_amount ELSE 0 END), 0) AS "income!: Decimal",
                COALESCE(SUM(CASE WHEN type = 'expense' THEN base_amount ELSE 0 END), 0) AS "expense!: Decimal",
                COUNT(*) AS "count!"
               FROM transactions
               WHERE ledger_id = $1
                 AND type IN ('income', 'expense')
                 AND occurred_at >= $2
                 AND occurred_at <= $3"#,
            ledger_id,
            start,
            end
        )
        .fetch_one(&self.pool)
        .await?;

        let income = row.income;
        let expense = row.expense;
        Ok(OverviewStats {
            total_income: income,
            total_expense: expense,
            net: income - expense,
            transaction_count: row.count,
        })
    }

    /// Monthly trend: income + expense per month for the past N months.
    pub async fn monthly_trend(
        &self,
        ledger_id: Uuid,
        timezone: &str,
        months: i32,
    ) -> AppResult<Vec<MonthlyStatsRow>> {
        let rows = sqlx::query_as!(
            MonthlyStatsRow,
            r#"SELECT
                to_char(date_trunc('month', occurred_at AT TIME ZONE $3), 'YYYY-MM') AS "year_month!: String",
                COALESCE(SUM(CASE WHEN type = 'income'  THEN base_amount ELSE 0 END), 0) AS "income!: Decimal",
                COALESCE(SUM(CASE WHEN type = 'expense' THEN base_amount ELSE 0 END), 0) AS "expense!: Decimal"
               FROM transactions
               WHERE ledger_id = $1
                 AND type IN ('income', 'expense')
                 AND occurred_at >= (date_trunc('month', NOW() AT TIME ZONE $3) - ($2 || ' months')::interval)
               GROUP BY date_trunc('month', occurred_at AT TIME ZONE $3)
               ORDER BY 1 ASC"#,
            ledger_id,
            months.to_string(),
            timezone
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn category_breakdown(
        &self,
        ledger_id: Uuid,
        transaction_type: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> AppResult<Vec<CategoryBreakdownRow>> {
        let rows = sqlx::query_as!(
            CategoryBreakdownRow,
            r#"SELECT
                t.category_id           AS "category_id?: Uuid",
                c.name                  AS "category_name?: String",
                SUM(t.base_amount)      AS "total!: Decimal",
                COUNT(*)                AS "count!"
               FROM transactions t
               LEFT JOIN categories c ON c.id = t.category_id
               WHERE t.ledger_id = $1
                 AND t.type = $2
                 AND t.occurred_at >= $3
                 AND t.occurred_at <= $4
               GROUP BY t.category_id, c.name
               ORDER BY "total!: Decimal" DESC"#,
            ledger_id,
            transaction_type,
            start,
            end
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    /// Top N spending categories in the current month.
    pub async fn top_categories(
        &self,
        ledger_id: Uuid,
        timezone: &str,
        limit: i64,
    ) -> AppResult<Vec<CategoryBreakdownRow>> {
        let rows = sqlx::query_as!(
            CategoryBreakdownRow,
            r#"SELECT
                t.category_id           AS "category_id?: Uuid",
                c.name                  AS "category_name?: String",
                SUM(t.base_amount)      AS "total!: Decimal",
                COUNT(*)                AS "count!"
               FROM transactions t
               LEFT JOIN categories c ON c.id = t.category_id
               WHERE t.ledger_id = $1
                 AND t.type = 'expense'
                 AND date_trunc('month', t.occurred_at AT TIME ZONE $2) =
                     date_trunc('month', NOW() AT TIME ZONE $2)
               GROUP BY t.category_id, c.name
               ORDER BY "total!: Decimal" DESC
               LIMIT $3"#,
            ledger_id,
            timezone,
            limit
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    /// Month-over-month comparison: this month vs last month.
    pub async fn mom_comparison(
        &self,
        ledger_id: Uuid,
        timezone: &str,
    ) -> AppResult<(OverviewStats, OverviewStats)> {
        let row = sqlx::query!(
            r#"SELECT
                COALESCE(SUM(CASE WHEN type = 'income'
                    AND date_trunc('month', occurred_at AT TIME ZONE $2) = date_trunc('month', NOW() AT TIME ZONE $2)
                    THEN base_amount ELSE 0 END), 0) AS "this_income!: Decimal",
                COALESCE(SUM(CASE WHEN type = 'expense'
                    AND date_trunc('month', occurred_at AT TIME ZONE $2) = date_trunc('month', NOW() AT TIME ZONE $2)
                    THEN base_amount ELSE 0 END), 0) AS "this_expense!: Decimal",
                COUNT(CASE WHEN type IN ('income','expense')
                    AND date_trunc('month', occurred_at AT TIME ZONE $2) = date_trunc('month', NOW() AT TIME ZONE $2)
                    THEN 1 END) AS "this_count!",
                COALESCE(SUM(CASE WHEN type = 'income'
                    AND date_trunc('month', occurred_at AT TIME ZONE $2) = date_trunc('month', NOW() AT TIME ZONE $2) - INTERVAL '1 month'
                    THEN base_amount ELSE 0 END), 0) AS "last_income!: Decimal",
                COALESCE(SUM(CASE WHEN type = 'expense'
                    AND date_trunc('month', occurred_at AT TIME ZONE $2) = date_trunc('month', NOW() AT TIME ZONE $2) - INTERVAL '1 month'
                    THEN base_amount ELSE 0 END), 0) AS "last_expense!: Decimal",
                COUNT(CASE WHEN type IN ('income','expense')
                    AND date_trunc('month', occurred_at AT TIME ZONE $2) = date_trunc('month', NOW() AT TIME ZONE $2) - INTERVAL '1 month'
                    THEN 1 END) AS "last_count!"
               FROM transactions
               WHERE ledger_id = $1 AND type IN ('income', 'expense')"#,
            ledger_id,
            timezone
        )
        .fetch_one(&self.pool)
        .await?;

        let this = OverviewStats {
            total_income: row.this_income,
            total_expense: row.this_expense,
            net: row.this_income - row.this_expense,
            transaction_count: row.this_count,
        };
        let last = OverviewStats {
            total_income: row.last_income,
            total_expense: row.last_expense,
            net: row.last_income - row.last_expense,
            transaction_count: row.last_count,
        };
        Ok((this, last))
    }
}
