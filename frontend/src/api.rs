use gloo_net::http::{Method, RequestBuilder};
use leptos::prelude::*;
use serde::{de::DeserializeOwned, Serialize};

use crate::state::AuthContext;

const BASE: &str = "/api/v1";

async fn fetch<T: DeserializeOwned>(
    method: Method,
    path: &str,
    token: Option<&str>,
    body: Option<&impl Serialize>,
) -> Result<T, String> {
    let url = format!("{BASE}{path}");
    let mut builder = RequestBuilder::new(&url).method(method);

    if let Some(tok) = token {
        builder = builder.header("Authorization", &format!("Bearer {tok}"));
    }

    let resp = if let Some(b) = body {
        let json = serde_json::to_string(b).map_err(|e| e.to_string())?;
        builder
            .header("Content-Type", "application/json")
            .body(json)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?
    } else {
        builder.send().await.map_err(|e| e.to_string())?
    };

    if resp.ok() {
        resp.json::<T>().await.map_err(|e| e.to_string())
    } else {
        let status = resp.status();
        let body: serde_json::Value = resp.json().await.unwrap_or_default();
        let msg = body["error"]["message"]
            .as_str()
            .unwrap_or("请求失败")
            .to_string();
        if status == 401 {
            if let Some(w) = web_sys::window() {
                if let Ok(Some(storage)) = w.local_storage() {
                    let _ = storage.remove_item("ledger_auth");
                }
                let _ = w.location().replace("/login");
            }
        }
        Err(msg)
    }
}

fn token() -> Option<String> {
    use_context::<AuthContext>().map(|ctx| ctx.0.get().token.clone()).flatten()
}

pub async fn get<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    fetch(Method::GET, path, token().as_deref(), None::<&()>).await
}

pub async fn post<B: Serialize, T: DeserializeOwned>(path: &str, body: &B) -> Result<T, String> {
    fetch(Method::POST, path, token().as_deref(), Some(body)).await
}

pub async fn patch<B: Serialize, T: DeserializeOwned>(path: &str, body: &B) -> Result<T, String> {
    fetch(Method::PATCH, path, token().as_deref(), Some(body)).await
}

pub async fn delete<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    fetch(Method::DELETE, path, token().as_deref(), None::<&()>).await
}

pub async fn post_no_auth<B: Serialize, T: DeserializeOwned>(
    path: &str,
    body: &B,
) -> Result<T, String> {
    fetch(Method::POST, path, None, Some(body)).await
}
