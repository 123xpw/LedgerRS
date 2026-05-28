use std::sync::Arc;

use anyhow::Context;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use backend::{
    config::AppConfig,
    handler::build_router,
    repository,
    AppState, Repos,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = AppConfig::from_env().context("failed to load config")?;

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&cfg.database_url)
        .await
        .context("failed to connect to database")?;

    sqlx::migrate!("../migrations")
        .run(&pool)
        .await
        .context("failed to run migrations")?;

    let repos = Repos {
        users: repository::UserRepository::new(pool.clone()),
        ledgers: repository::LedgerRepository::new(pool.clone()),
        transactions: repository::TransactionRepository::new(pool.clone()),
        categories: repository::CategoryRepository::new(pool.clone()),
        tags: repository::TagRepository::new(pool.clone()),
        budgets: repository::BudgetRepository::new(pool.clone()),
        exchange_rates: repository::ExchangeRateRepository::new(pool.clone()),
        reports: repository::ReportRepository::new(pool.clone()),
    };

    let state = AppState {
        cfg: Arc::new(cfg.clone()),
        db: pool.clone(),
        repos,
    };

    backend::service::exchange_rates::start_cron(state.clone()).await?;

    let app = build_router(state);

    let addr = format!("0.0.0.0:{}", cfg.server_port);
    tracing::info!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context("failed to bind")?;
    axum::serve(listener, app).await?;
    Ok(())
}
