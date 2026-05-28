use chrono::{DateTime, Utc};
use uuid::Uuid;

use shared::dto::reports::{
    CategoryBreakdownItem, CategoryBreakdownResponse, ComparisonResponse, MonthlyStats,
    MultiLedgerReportRequest, MultiLedgerReportResponse, OverviewResponse, PeriodStats,
    TopCategoriesResponse, TopCategoryItem, TrendResponse,
};

use crate::{
    error::{AppError, AppResult},
    AppState,
};

pub async fn overview(
    state: &AppState,
    user_id: Uuid,
    ledger_id: Uuid,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> AppResult<OverviewResponse> {
    check_ledger_owner(state, user_id, ledger_id).await?;
    let stats = state.repos.reports.overview(ledger_id, start, end).await?;
    Ok(OverviewResponse {
        total_income: stats.total_income,
        total_expense: stats.total_expense,
        net: stats.net,
        transaction_count: stats.transaction_count,
    })
}

pub async fn trend(
    state: &AppState,
    user_id: Uuid,
    ledger_id: Uuid,
    timezone: &str,
    months: i32,
) -> AppResult<TrendResponse> {
    check_ledger_owner(state, user_id, ledger_id).await?;
    let months = months.clamp(1, 24);
    let rows = state
        .repos
        .reports
        .monthly_trend(ledger_id, timezone, months)
        .await?;
    Ok(TrendResponse {
        months: rows
            .iter()
            .map(|r| MonthlyStats {
                year_month: r.year_month.clone(),
                income: r.income,
                expense: r.expense,
                net: r.income - r.expense,
            })
            .collect(),
    })
}

pub async fn category_breakdown(
    state: &AppState,
    user_id: Uuid,
    ledger_id: Uuid,
    transaction_type: &str,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> AppResult<CategoryBreakdownResponse> {
    check_ledger_owner(state, user_id, ledger_id).await?;
    if transaction_type != "income" && transaction_type != "expense" {
        return Err(AppError::Validation("type must be income or expense".into()));
    }

    let rows = state
        .repos
        .reports
        .category_breakdown(ledger_id, transaction_type, start, end)
        .await?;

    let total: rust_decimal::Decimal = rows.iter().map(|r| r.total).sum();

    let items = rows
        .iter()
        .map(|r| {
            let pct = if total.is_zero() {
                0.0
            } else {
                (r.total / total * rust_decimal::Decimal::from(100))
                    .to_string()
                    .parse::<f64>()
                    .unwrap_or(0.0)
            };
            CategoryBreakdownItem {
                category_id: r.category_id,
                category_name: r.category_name.clone(),
                total: r.total,
                count: r.count,
                percentage: pct,
            }
        })
        .collect();

    Ok(CategoryBreakdownResponse { items, total })
}

pub async fn top_categories(
    state: &AppState,
    user_id: Uuid,
    ledger_id: Uuid,
    timezone: &str,
    limit: i64,
) -> AppResult<TopCategoriesResponse> {
    check_ledger_owner(state, user_id, ledger_id).await?;
    let rows = state
        .repos
        .reports
        .top_categories(ledger_id, timezone, limit.clamp(1, 20))
        .await?;
    Ok(TopCategoriesResponse {
        items: rows
            .iter()
            .map(|r| TopCategoryItem {
                category_id: r.category_id,
                category_name: r.category_name.clone(),
                total: r.total,
                count: r.count,
            })
            .collect(),
    })
}

pub async fn mom_comparison(
    state: &AppState,
    user_id: Uuid,
    ledger_id: Uuid,
    timezone: &str,
) -> AppResult<ComparisonResponse> {
    check_ledger_owner(state, user_id, ledger_id).await?;
    let (this, last) = state
        .repos
        .reports
        .mom_comparison(ledger_id, timezone)
        .await?;

    let income_change = pct_change(last.total_income, this.total_income);
    let expense_change = pct_change(last.total_expense, this.total_expense);

    Ok(ComparisonResponse {
        this_period: PeriodStats {
            income: this.total_income,
            expense: this.total_expense,
            net: this.net,
            transaction_count: this.transaction_count,
        },
        last_period: PeriodStats {
            income: last.total_income,
            expense: last.total_expense,
            net: last.net,
            transaction_count: last.transaction_count,
        },
        income_change_pct: income_change,
        expense_change_pct: expense_change,
    })
}

pub async fn multi_ledger_report(
    state: &AppState,
    user_id: Uuid,
    req: MultiLedgerReportRequest,
    timezone: &str,
) -> AppResult<MultiLedgerReportResponse> {
    let mut ledger_stats = Vec::new();
    for lid in &req.ledger_ids {
        check_ledger_owner(state, user_id, *lid).await?;
        let stats = state
            .repos
            .reports
            .overview(*lid, req.start, req.end)
            .await?;
        let ledger = state.repos.ledgers.find_by_id(*lid).await?.unwrap();
        ledger_stats.push(shared::dto::reports::LedgerStats {
            ledger_id: *lid,
            ledger_name: ledger.name.clone(),
            base_currency: ledger.base_currency.clone(),
            total_income: stats.total_income,
            total_expense: stats.total_expense,
            net: stats.net,
        });
    }
    let _ = timezone; // available for future use

    Ok(MultiLedgerReportResponse {
        ledgers: ledger_stats,
        start: req.start,
        end: req.end,
    })
}

fn pct_change(
    old: rust_decimal::Decimal,
    new: rust_decimal::Decimal,
) -> Option<f64> {
    if old.is_zero() {
        return None;
    }
    ((new - old) / old * rust_decimal::Decimal::from(100))
        .to_string()
        .parse::<f64>()
        .ok()
}

async fn check_ledger_owner(state: &AppState, user_id: Uuid, ledger_id: Uuid) -> AppResult<()> {
    let ledger = state
        .repos
        .ledgers
        .find_by_id(ledger_id)
        .await?
        .ok_or_else(|| AppError::NotFound("账本不存在".into()))?;
    if ledger.user_id != user_id {
        return Err(AppError::Forbidden("无权访问该账本".into()));
    }
    Ok(())
}
