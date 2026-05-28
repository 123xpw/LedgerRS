use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use shared::dto::reports::{
    CategoryBreakdownResponse, ComparisonResponse, MultiLedgerReportRequest,
    MultiLedgerReportResponse, OverviewResponse, TopCategoriesResponse, TrendResponse,
};

use crate::{error::AppResult, middleware::AuthUser, service, AppState};

#[derive(Deserialize)]
pub struct DateRangeQuery {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct TrendQuery {
    #[serde(default = "default_months")]
    pub months: i32,
}

#[derive(Deserialize)]
pub struct BreakdownQuery {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    #[serde(rename = "type", default = "default_type")]
    pub transaction_type: String,
}

#[derive(Deserialize)]
pub struct TopQuery {
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_months() -> i32 { 12 }
fn default_type() -> String { "expense".to_string() }
fn default_limit() -> i64 { 5 }

pub async fn overview(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(ledger_id): Path<Uuid>,
    Query(q): Query<DateRangeQuery>,
) -> AppResult<Json<OverviewResponse>> {
    let resp = service::reports::overview(&state, auth.user_id, ledger_id, q.start, q.end).await?;
    Ok(Json(resp))
}

pub async fn trend(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(ledger_id): Path<Uuid>,
    Query(q): Query<TrendQuery>,
) -> AppResult<Json<TrendResponse>> {
    let user = state.repos.users.find_by_id(auth.user_id).await?.unwrap();
    let resp =
        service::reports::trend(&state, auth.user_id, ledger_id, &user.timezone, q.months)
            .await?;
    Ok(Json(resp))
}

pub async fn category_breakdown(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(ledger_id): Path<Uuid>,
    Query(q): Query<BreakdownQuery>,
) -> AppResult<Json<CategoryBreakdownResponse>> {
    let resp = service::reports::category_breakdown(
        &state,
        auth.user_id,
        ledger_id,
        &q.transaction_type,
        q.start,
        q.end,
    )
    .await?;
    Ok(Json(resp))
}

pub async fn top_categories(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(ledger_id): Path<Uuid>,
    Query(q): Query<TopQuery>,
) -> AppResult<Json<TopCategoriesResponse>> {
    let user = state.repos.users.find_by_id(auth.user_id).await?.unwrap();
    let resp =
        service::reports::top_categories(&state, auth.user_id, ledger_id, &user.timezone, q.limit)
            .await?;
    Ok(Json(resp))
}

pub async fn comparison(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(ledger_id): Path<Uuid>,
) -> AppResult<Json<ComparisonResponse>> {
    let user = state.repos.users.find_by_id(auth.user_id).await?.unwrap();
    let resp =
        service::reports::mom_comparison(&state, auth.user_id, ledger_id, &user.timezone).await?;
    Ok(Json(resp))
}

pub async fn multi_ledger(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<MultiLedgerReportRequest>,
) -> AppResult<Json<MultiLedgerReportResponse>> {
    let user = state.repos.users.find_by_id(auth.user_id).await?.unwrap();
    let resp =
        service::reports::multi_ledger_report(&state, auth.user_id, req, &user.timezone).await?;
    Ok(Json(resp))
}
