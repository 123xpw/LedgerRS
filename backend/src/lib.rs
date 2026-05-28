pub mod config;
pub mod error;
pub mod handler;
pub mod middleware;
pub mod model;
pub mod repository;
pub mod service;

use std::sync::Arc;
use config::AppConfig;

/// Shared application state injected into every handler.
#[derive(Clone)]
pub struct AppState {
    pub cfg: Arc<AppConfig>,
    pub db: sqlx::PgPool,
    pub repos: Repos,
}

/// All repository handles, pre-constructed at startup.
#[derive(Clone)]
pub struct Repos {
    pub users: repository::UserRepository,
    pub ledgers: repository::LedgerRepository,
    pub transactions: repository::TransactionRepository,
    pub categories: repository::CategoryRepository,
    pub tags: repository::TagRepository,
    pub budgets: repository::BudgetRepository,
    pub exchange_rates: repository::ExchangeRateRepository,
    pub reports: repository::ReportRepository,
}
