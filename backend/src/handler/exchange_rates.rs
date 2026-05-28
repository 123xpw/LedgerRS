use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;

use shared::dto::exchange_rates::LatestRatesResponse;

use crate::{error::AppResult, middleware::AuthUser, service, AppState};

#[derive(Deserialize)]
pub struct RateQuery {
    #[serde(default = "default_base")]
    pub base: String,
}

fn default_base() -> String {
    "CNY".to_string()
}

pub async fn get_rates(
    State(state): State<AppState>,
    _auth: AuthUser,
    Query(q): Query<RateQuery>,
) -> AppResult<Json<LatestRatesResponse>> {
    let rates = service::exchange_rates::get_latest_rates(&state, &q.base).await?;
    Ok(Json(rates))
}
