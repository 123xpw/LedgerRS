use std::collections::HashMap;

use chrono::Utc;
use rust_decimal::Decimal;
use tokio_cron_scheduler::{Job, JobScheduler};

use shared::dto::exchange_rates::{CurrencyInfo, LatestRatesResponse};
use shared::types::SUPPORTED_CURRENCIES;

use crate::{
    error::{AppError, AppResult},
    AppState,
};

pub async fn get_latest_rates(
    state: &AppState,
    base: &str,
) -> AppResult<LatestRatesResponse> {
    if !shared::types::is_valid_currency(base) {
        return Err(AppError::Validation(format!("不支持的基准币种: {base}")));
    }

    let rows = state.repos.exchange_rates.find_latest_rates(base).await?;
    let latest_date = state.repos.exchange_rates.latest_date().await?;

    let (is_stale, stale_days) = if let Some(d) = latest_date {
        let days = (Utc::now().date_naive() - d).num_days();
        (days > 1, days)
    } else {
        (true, 0)
    };

    let mut rates: HashMap<String, Decimal> = rows
        .iter()
        .map(|r| (r.quote_currency.clone(), r.rate))
        .collect();

    // Always include the base currency itself.
    rates.insert(base.to_string(), Decimal::ONE);

    let currencies = SUPPORTED_CURRENCIES
        .iter()
        .map(|&code| CurrencyInfo {
            code: code.to_string(),
            rate: rates.get(code).copied(),
        })
        .collect();

    Ok(LatestRatesResponse {
        base: base.to_string(),
        is_stale,
        stale_days,
        currencies,
    })
}

/// Fetch rates from exchangerate.host and upsert into DB.
pub async fn fetch_and_store_rates(state: &AppState) -> AppResult<()> {
    let today = Utc::now().date_naive();
    let url = format!(
        "{}/live?base=CNY&currencies={}&places=8",
        state.cfg.exchange_rate_api_url,
        SUPPORTED_CURRENCIES.join(",")
    );

    let resp: serde_json::Value = reqwest::Client::new()
        .get(&url)
        .timeout(std::time::Duration::from_secs(15))
        .send()
        .await
        .map_err(|e| AppError::ExternalApi(e.to_string()))?
        .json()
        .await
        .map_err(|e| AppError::ExternalApi(e.to_string()))?;

    let quotes = resp
        .get("quotes")
        .and_then(|q| q.as_object())
        .ok_or_else(|| AppError::ExternalApi("unexpected response shape".into()))?;

    let mut entries: Vec<(String, Decimal, chrono::NaiveDate)> = Vec::new();
    for (key, val) in quotes {
        // Key format: "CNYUSD" — strip the base prefix (first 3 chars).
        let quote = &key[3..];
        if let Some(rate_f) = val.as_f64() {
            if let Ok(rate) = Decimal::try_from(rate_f) {
                entries.push((quote.to_string(), rate, today));
            }
        }
    }

    state
        .repos
        .exchange_rates
        .bulk_upsert("CNY", &entries)
        .await?;

    tracing::info!(count = entries.len(), "exchange rates updated");
    Ok(())
}

/// Register the daily cron job (UTC 00:05).
pub async fn start_cron(state: AppState) -> anyhow::Result<()> {
    let sched = JobScheduler::new().await?;

    let job = Job::new_async("0 5 0 * * *", move |_uuid, _l| {
        let s = state.clone();
        Box::pin(async move {
            if let Err(e) = fetch_and_store_rates(&s).await {
                tracing::error!(error = %e, "daily exchange rate fetch failed");
            }
        })
    })?;

    sched.add(job).await?;
    sched.start().await?;
    Ok(())
}
