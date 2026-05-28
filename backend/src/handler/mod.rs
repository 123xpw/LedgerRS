pub mod auth;
pub mod budgets;
pub mod categories;
pub mod data;
pub mod exchange_rates;
pub mod ledgers;
pub mod reports;
pub mod tags;
pub mod transactions;

use axum::{
    routing::{get, patch, post},
    Router,
};
use tower_http::{cors::CorsLayer, services::{ServeDir, ServeFile}, trace::TraceLayer};

use crate::AppState;

pub fn build_router(state: AppState) -> Router {
    let api = Router::new()
        // Auth
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        .route("/auth/refresh", post(auth::refresh))
        .route("/auth/logout", post(auth::logout))
        .route("/auth/me", get(auth::me).patch(auth::update_profile))
        .route("/auth/me/password", patch(auth::change_password))
        // Ledgers
        .route("/ledgers", get(ledgers::list).post(ledgers::create))
        .route(
            "/ledgers/:id",
            get(ledgers::get).patch(ledgers::update).delete(ledgers::delete),
        )
        // Transactions
        .route(
            "/ledgers/:id/transactions",
            get(transactions::list).post(transactions::create),
        )
        .route(
            "/ledgers/:id/transactions/:tx_id",
            get(transactions::get)
                .patch(transactions::update)
                .delete(transactions::delete),
        )
        // Categories
        .route(
            "/ledgers/:id/categories",
            get(categories::list).post(categories::create),
        )
        .route(
            "/ledgers/:id/categories/:cat_id",
            patch(categories::update).delete(categories::delete),
        )
        // Tags
        .route("/ledgers/:id/tags", get(tags::list).post(tags::create))
        .route(
            "/ledgers/:id/tags/:tag_id",
            patch(tags::update).delete(tags::delete),
        )
        // Budgets
        .route(
            "/ledgers/:id/budgets",
            get(budgets::list).post(budgets::create),
        )
        .route(
            "/ledgers/:id/budgets/:budget_id",
            patch(budgets::update).delete(budgets::delete),
        )
        // Reports
        .route("/ledgers/:id/reports/overview", get(reports::overview))
        .route("/ledgers/:id/reports/trend", get(reports::trend))
        .route(
            "/ledgers/:id/reports/categories",
            get(reports::category_breakdown),
        )
        .route(
            "/ledgers/:id/reports/top-categories",
            get(reports::top_categories),
        )
        .route("/ledgers/:id/reports/comparison", get(reports::comparison))
        .route("/reports/multi-ledger", post(reports::multi_ledger))
        // Exchange rates
        .route("/exchange-rates", get(exchange_rates::get_rates))
        // Data
        .route("/data/backup", get(data::export))
        .route("/data/restore", post(data::import))
        // Export
        .route("/ledgers/:id/export/csv", get(data::export_csv))
        .route("/ledgers/:id/export/excel", get(data::export_excel));

    let api = api.with_state(state.clone());

    Router::new()
        .nest("/api/v1", api)
        // Serve swagger UI at /swagger-ui
        .route("/swagger-ui", get(|| async { "swagger placeholder" }))
        // Serve Leptos frontend (SPA fallback — all unmatched paths → index.html)
        .fallback_service(
            ServeDir::new(&state.cfg.frontend_dist)
                .fallback(ServeFile::new(format!("{}/index.html", &state.cfg.frontend_dist)))
        )
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}
