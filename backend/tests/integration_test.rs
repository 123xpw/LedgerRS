/// Integration tests for LedgerRS backend.
///
/// Requires DATABASE_URL to point to a real PostgreSQL instance with migrations applied.
/// Run with: cargo test -p backend -- --test-threads=1
use axum::{body::Body, http::Request, http::StatusCode};
use serde_json::{json, Value};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower::ServiceExt;

use backend::handler::build_router;
use backend::{AppState, Repos};

// ─── Helpers ─────────────────────────────────────────────────────────────────

async fn test_app() -> (axum::Router, sqlx::PgPool) {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL required for integration tests");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("connect to test DB");

    sqlx::migrate!("../migrations")
        .run(&pool)
        .await
        .expect("run migrations");

    let cfg = Arc::new(backend::config::AppConfig {
        database_url: db_url,
        jwt_secret: "test_secret_key_minimum_32_chars_!!".to_string(),
        jwt_expiry_seconds: 3600,
        refresh_expiry_seconds: 86400,
        exchange_rate_api_url: "https://api.exchangerate.host".to_string(),
        server_port: 8080,
        frontend_dist: "frontend/dist".to_string(),
    });

    let repos = Repos {
        users: backend::repository::UserRepository::new(pool.clone()),
        ledgers: backend::repository::LedgerRepository::new(pool.clone()),
        transactions: backend::repository::TransactionRepository::new(pool.clone()),
        categories: backend::repository::CategoryRepository::new(pool.clone()),
        tags: backend::repository::TagRepository::new(pool.clone()),
        budgets: backend::repository::BudgetRepository::new(pool.clone()),
        exchange_rates: backend::repository::ExchangeRateRepository::new(pool.clone()),
        reports: backend::repository::ReportRepository::new(pool.clone()),
    };

    let state = AppState { cfg, db: pool.clone(), repos };
    let app = build_router(state);
    (app, pool)
}

async fn json_request(app: &axum::Router, method: &str, path: &str, token: Option<&str>, body: Option<Value>) -> (StatusCode, Value) {
    let mut req = Request::builder().method(method).uri(path);
    if let Some(tok) = token {
        req = req.header("Authorization", format!("Bearer {tok}"));
    }
    let body = if let Some(b) = body {
        req = req.header("Content-Type", "application/json");
        Body::from(b.to_string())
    } else {
        Body::empty()
    };
    let response = app.clone().oneshot(req.body(body).unwrap()).await.unwrap();
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&bytes).unwrap_or_default();
    (status, json)
}

fn unique_email() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
    format!("test_{}@example.com", ts)
}

fn unique_username() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
    format!("user_{}", ts)
}

// ─── Auth tests ───────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_register_and_login() {
    let (app, _pool) = test_app().await;
    let email = unique_email();
    let username = unique_username();

    // Register
    let (status, body) = json_request(
        &app, "POST", "/api/v1/auth/register", None,
        Some(json!({ "email": email, "username": username, "password": "password123" })),
    ).await;
    assert_eq!(status, StatusCode::OK, "register: {body}");
    assert!(body["access_token"].is_string());
    assert!(body["user"]["id"].is_string());

    // Login
    let (status, body) = json_request(
        &app, "POST", "/api/v1/auth/login", None,
        Some(json!({ "credential": email, "password": "password123" })),
    ).await;
    assert_eq!(status, StatusCode::OK, "login: {body}");
    assert!(body["access_token"].is_string());
}

#[tokio::test]
async fn test_duplicate_email_rejected() {
    let (app, _pool) = test_app().await;
    let email = unique_email();
    let body = json!({ "email": email, "username": unique_username(), "password": "password123" });
    json_request(&app, "POST", "/api/v1/auth/register", None, Some(body.clone())).await;
    let body2 = json!({ "email": email, "username": unique_username(), "password": "password123" });
    let (status, resp) = json_request(&app, "POST", "/api/v1/auth/register", None, Some(body2)).await;
    assert_eq!(status, StatusCode::CONFLICT, "expected conflict: {resp}");
}

#[tokio::test]
async fn test_wrong_password_rejected() {
    let (app, _pool) = test_app().await;
    let email = unique_email();
    json_request(
        &app, "POST", "/api/v1/auth/register", None,
        Some(json!({ "email": email, "username": unique_username(), "password": "correct_pw" })),
    ).await;
    let (status, _) = json_request(
        &app, "POST", "/api/v1/auth/login", None,
        Some(json!({ "credential": email, "password": "wrong_pw" })),
    ).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

// ─── Ledger tests ─────────────────────────────────────────────────────────────

async fn register_and_get_token(app: &axum::Router) -> String {
    let (_, body) = json_request(
        app, "POST", "/api/v1/auth/register", None,
        Some(json!({
            "email": unique_email(),
            "username": unique_username(),
            "password": "password123"
        })),
    ).await;
    body["access_token"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn test_default_ledger_created_on_register() {
    let (app, _pool) = test_app().await;
    let token = register_and_get_token(&app).await;

    let (status, body) = json_request(&app, "GET", "/api/v1/ledgers", Some(&token), None).await;
    assert_eq!(status, StatusCode::OK);
    let ledgers = body.as_array().unwrap();
    assert_eq!(ledgers.len(), 1, "expected one default ledger");
    assert_eq!(ledgers[0]["name"], "我的账本");
}

#[tokio::test]
async fn test_create_and_delete_ledger() {
    let (app, _pool) = test_app().await;
    let token = register_and_get_token(&app).await;

    // Create a second ledger
    let (status, body) = json_request(
        &app, "POST", "/api/v1/ledgers", Some(&token),
        Some(json!({ "name": "Travel", "base_currency": "USD" })),
    ).await;
    assert_eq!(status, StatusCode::OK, "create ledger: {body}");
    let id = body["id"].as_str().unwrap().to_string();

    // Delete it
    let (status, _) = json_request(&app, "DELETE", &format!("/api/v1/ledgers/{id}"), Some(&token), None).await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn test_cannot_delete_last_ledger() {
    let (app, _pool) = test_app().await;
    let token = register_and_get_token(&app).await;

    let (_, body) = json_request(&app, "GET", "/api/v1/ledgers", Some(&token), None).await;
    let id = body[0]["id"].as_str().unwrap().to_string();

    let (status, resp) = json_request(&app, "DELETE", &format!("/api/v1/ledgers/{id}"), Some(&token), None).await;
    assert_eq!(status, StatusCode::CONFLICT, "expected LAST_LEDGER: {resp}");
    assert_eq!(resp["error"]["code"], "LAST_LEDGER");
}

// ─── Transaction tests ────────────────────────────────────────────────────────

#[tokio::test]
async fn test_create_and_list_transaction() {
    let (app, _pool) = test_app().await;
    let token = register_and_get_token(&app).await;

    let (_, ledgers) = json_request(&app, "GET", "/api/v1/ledgers", Some(&token), None).await;
    let lid = ledgers[0]["id"].as_str().unwrap().to_string();

    // Create expense
    let (status, tx) = json_request(
        &app, "POST", &format!("/api/v1/ledgers/{lid}/transactions"), Some(&token),
        Some(json!({
            "type": "expense",
            "amount": "50.00",
            "currency": "CNY",
            "occurred_at": "2026-05-01T00:00:00Z"
        })),
    ).await;
    assert_eq!(status, StatusCode::OK, "create tx: {tx}");
    assert_eq!(tx["type"], "expense");
    let returned: rust_decimal::Decimal = tx["amount"].as_str().unwrap().parse().unwrap();
    assert_eq!(returned, "50.00".parse::<rust_decimal::Decimal>().unwrap());

    // List
    let (status, paged) = json_request(
        &app, "GET", &format!("/api/v1/ledgers/{lid}/transactions"), Some(&token), None
    ).await;
    assert_eq!(status, StatusCode::OK);
    assert!(paged["meta"]["total"].as_i64().unwrap() >= 1);
}

#[tokio::test]
async fn test_unauthorized_access_rejected() {
    let (app, _pool) = test_app().await;
    let (status, body) = json_request(&app, "GET", "/api/v1/ledgers", None, None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED, "{body}");
}

// ─── Category tests ───────────────────────────────────────────────────────────

#[tokio::test]
async fn test_system_categories_seeded() {
    let (app, _pool) = test_app().await;
    let token = register_and_get_token(&app).await;

    let (_, ledgers) = json_request(&app, "GET", "/api/v1/ledgers", Some(&token), None).await;
    let lid = ledgers[0]["id"].as_str().unwrap().to_string();

    let (status, cats) = json_request(
        &app, "GET", &format!("/api/v1/ledgers/{lid}/categories"), Some(&token), None
    ).await;
    assert_eq!(status, StatusCode::OK);
    let list = cats.as_array().unwrap();
    // Expect 13 system categories
    let system: Vec<_> = list.iter().filter(|c| c["is_system"] == true).collect();
    assert_eq!(system.len(), 13, "expected 13 system categories, got {}", system.len());
}

#[tokio::test]
async fn test_cannot_delete_system_category() {
    let (app, _pool) = test_app().await;
    let token = register_and_get_token(&app).await;

    let (_, ledgers) = json_request(&app, "GET", "/api/v1/ledgers", Some(&token), None).await;
    let lid = ledgers[0]["id"].as_str().unwrap().to_string();

    let (_, cats) = json_request(
        &app, "GET", &format!("/api/v1/ledgers/{lid}/categories"), Some(&token), None
    ).await;
    let sys_id = cats.as_array().unwrap()
        .iter()
        .find(|c| c["is_system"] == true)
        .unwrap()["id"].as_str().unwrap().to_string();

    let (status, resp) = json_request(
        &app, "DELETE",
        &format!("/api/v1/ledgers/{lid}/categories/{sys_id}"),
        Some(&token), None
    ).await;
    assert_eq!(status, StatusCode::FORBIDDEN, "should not delete system category: {resp}");
}

// ─── Budget tests ─────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_create_and_list_budget() {
    let (app, _pool) = test_app().await;
    let token = register_and_get_token(&app).await;

    let (_, ledgers) = json_request(&app, "GET", "/api/v1/ledgers", Some(&token), None).await;
    let lid = ledgers[0]["id"].as_str().unwrap().to_string();

    let (status, b) = json_request(
        &app, "POST", &format!("/api/v1/ledgers/{lid}/budgets"), Some(&token),
        Some(json!({ "period": "month", "amount": "3000.00" })),
    ).await;
    assert_eq!(status, StatusCode::OK, "create budget: {b}");
    assert_eq!(b["period"], "month");
    assert!(b["usage_rate"].is_number());
}
