use leptos::prelude::RwSignal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AuthState {
    pub token: Option<String>,
    pub refresh_token: Option<String>,
    pub user_id: Option<Uuid>,
    pub username: Option<String>,
    pub timezone: Option<String>,
}

impl AuthState {
    const STORAGE_KEY: &'static str = "ledger_auth";

    pub fn load_from_storage() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(storage) = web_sys::window()
                .and_then(|w| w.local_storage().ok().flatten())
            {
                if let Ok(Some(json)) = storage.get_item(Self::STORAGE_KEY) {
                    if let Ok(state) = serde_json::from_str(&json) {
                        return state;
                    }
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(storage) = web_sys::window()
                .and_then(|w| w.local_storage().ok().flatten())
            {
                if let Ok(json) = serde_json::to_string(self) {
                    let _ = storage.set_item(Self::STORAGE_KEY, &json);
                }
            }
        }
    }

    pub fn clear(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(storage) = web_sys::window()
                .and_then(|w| w.local_storage().ok().flatten())
            {
                let _ = storage.remove_item(Self::STORAGE_KEY);
            }
        }
    }
}

/// Context wrapper passed via provide_context.
#[derive(Clone, Copy)]
pub struct AuthContext(pub RwSignal<AuthState>);
