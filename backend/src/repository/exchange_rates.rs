use chrono::NaiveDate;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::AppResult, model::ExchangeRateModel};

#[derive(Clone)]
pub struct ExchangeRateRepository {
    pool: PgPool,
}

impl ExchangeRateRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Retrieve the most recent rate on or before `date` for a currency pair.
    pub async fn find_rate(
        &self,
        base: &str,
        quote: &str,
        date: NaiveDate,
    ) -> AppResult<Option<Decimal>> {
        let row = sqlx::query!(
            "SELECT rate FROM exchange_rates
             WHERE base_currency = $1 AND quote_currency = $2 AND date <= $3
             ORDER BY date DESC
             LIMIT 1",
            base,
            quote,
            date
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|r| r.rate))
    }

    /// Return the most recent available date for any rate data.
    pub async fn latest_date(&self) -> AppResult<Option<NaiveDate>> {
        let row = sqlx::query!("SELECT MAX(date) AS latest FROM exchange_rates")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.latest)
    }

    /// Latest rates for all pairs where base_currency = `base` on the most recent date.
    pub async fn find_latest_rates(
        &self,
        base: &str,
    ) -> AppResult<Vec<ExchangeRateModel>> {
        let rows = sqlx::query_as!(
            ExchangeRateModel,
            r#"SELECT er.id, er.base_currency, er.quote_currency, er.rate, er.date, er.created_at
               FROM exchange_rates er
               INNER JOIN (
                   SELECT quote_currency, MAX(date) AS max_date
                   FROM exchange_rates
                   WHERE base_currency = $1
                   GROUP BY quote_currency
               ) latest ON latest.quote_currency = er.quote_currency
                        AND latest.max_date = er.date
               WHERE er.base_currency = $1
               ORDER BY er.quote_currency ASC"#,
            base
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    /// Upsert a rate row (insert or update on conflict).
    pub async fn upsert(
        &self,
        base: &str,
        quote: &str,
        rate: Decimal,
        date: NaiveDate,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"INSERT INTO exchange_rates (base_currency, quote_currency, rate, date)
               VALUES ($1, $2, $3, $4)
               ON CONFLICT ON CONSTRAINT uq_exchange_rate
               DO UPDATE SET rate = EXCLUDED.rate"#,
            base,
            quote,
            rate,
            date
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Bulk upsert for the daily cron job.
    pub async fn bulk_upsert(
        &self,
        base: &str,
        entries: &[(String, Decimal, NaiveDate)],
    ) -> AppResult<()> {
        for (quote, rate, date) in entries {
            self.upsert(base, quote, *rate, *date).await?;
        }
        Ok(())
    }

    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Option<ExchangeRateModel>> {
        let row = sqlx::query_as!(
            ExchangeRateModel,
            "SELECT id, base_currency, quote_currency, rate, date, created_at
             FROM exchange_rates WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }
}
