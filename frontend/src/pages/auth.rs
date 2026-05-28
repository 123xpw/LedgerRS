use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;

use shared::dto::auth::{AuthResponse, LoginRequest, RegisterRequest};

use crate::{api, state::AuthContext};

#[component]
pub fn LoginPage() -> impl IntoView {
    let credential = create_rw_signal(String::new());
    let password = create_rw_signal(String::new());
    let error = create_rw_signal(Option::<String>::None);
    let loading = create_rw_signal(false);
    let auth_ctx = use_context::<AuthContext>().expect("AuthContext");
    let navigate = use_navigate();

    let submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let cred = credential.get();
        let pwd  = password.get();
        if cred.is_empty() || pwd.is_empty() {
            error.set(Some("请填写所有字段".into()));
            return;
        }
        loading.set(true);
        error.set(None);
        let nav = navigate.clone();
        spawn_local(async move {
            let body = LoginRequest { credential: cred, password: pwd };
            match api::post_no_auth::<_, AuthResponse>("/auth/login", &body).await {
                Ok(resp) => {
                    auth_ctx.0.update(|s| {
                        s.token = Some(resp.access_token.clone());
                        s.refresh_token = Some(resp.refresh_token.clone());
                        s.user_id = Some(resp.user.id);
                        s.username = Some(resp.user.username.clone());
                        s.timezone = Some(resp.user.timezone.clone());
                        s.save();
                    });
                    nav("/", Default::default());
                }
                Err(e) => {
                    error.set(Some(e));
                    loading.set(false);
                }
            }
        });
    };

    view! {
        <div style="min-height:100vh;display:flex;align-items:center;justify-content:center;background:var(--bg)">
            <div class="card" style="width:380px">
                <h1 style="font-size:22px;font-weight:700;margin-bottom:24px;text-align:center">
                    "💰 LedgerRS 登录"
                </h1>
                <form on:submit=submit>
                    <div class="form-group">
                        <label>"邮箱 / 用户名"</label>
                        <input type="text" placeholder="输入邮箱或用户名"
                            prop:value=move || credential.get()
                            on:input=move |ev| credential.set(event_target_value(&ev)) />
                    </div>
                    <div class="form-group">
                        <label>"密码"</label>
                        <input type="password" placeholder="输入密码"
                            prop:value=move || password.get()
                            on:input=move |ev| password.set(event_target_value(&ev)) />
                    </div>
                    { move || error.get().map(|e| view! { <p class="error-msg">{ e }</p> }) }
                    <button type="submit" class="btn-primary"
                        style="width:100%;margin-top:8px"
                        disabled=move || loading.get()>
                        { move || if loading.get() { "登录中…" } else { "登录" } }
                    </button>
                </form>
                <p style="text-align:center;margin-top:16px;color:var(--muted)">
                    "没有账号? " <a href="/register">"立即注册"</a>
                </p>
            </div>
        </div>
    }
}

#[component]
pub fn RegisterPage() -> impl IntoView {
    let email    = create_rw_signal(String::new());
    let username = create_rw_signal(String::new());
    let password = create_rw_signal(String::new());
    let error    = create_rw_signal(Option::<String>::None);
    let loading  = create_rw_signal(false);
    let auth_ctx = use_context::<AuthContext>().expect("AuthContext");
    let navigate = use_navigate();

    let submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let em = email.get();
        let un = username.get();
        let pw = password.get();
        if em.is_empty() || un.is_empty() || pw.is_empty() {
            error.set(Some("请填写所有字段".into()));
            return;
        }
        if pw.len() < 8 {
            error.set(Some("密码至少8位".into()));
            return;
        }
        loading.set(true);
        error.set(None);
        let nav = navigate.clone();
        spawn_local(async move {
            let body = RegisterRequest {
                email: em,
                username: un,
                password: pw,
                default_currency: None,
                timezone: None,
            };
            match api::post_no_auth::<_, AuthResponse>("/auth/register", &body).await {
                Ok(resp) => {
                    auth_ctx.0.update(|s| {
                        s.token = Some(resp.access_token.clone());
                        s.refresh_token = Some(resp.refresh_token.clone());
                        s.user_id = Some(resp.user.id);
                        s.username = Some(resp.user.username.clone());
                        s.timezone = Some(resp.user.timezone.clone());
                        s.save();
                    });
                    nav("/", Default::default());
                }
                Err(e) => {
                    error.set(Some(e));
                    loading.set(false);
                }
            }
        });
    };

    view! {
        <div style="min-height:100vh;display:flex;align-items:center;justify-content:center;background:var(--bg)">
            <div class="card" style="width:380px">
                <h1 style="font-size:22px;font-weight:700;margin-bottom:24px;text-align:center">
                    "💰 注册账号"
                </h1>
                <form on:submit=submit>
                    <div class="form-group">
                        <label>"邮箱"</label>
                        <input type="email" placeholder="your@email.com"
                            prop:value=move || email.get()
                            on:input=move |ev| email.set(event_target_value(&ev)) />
                    </div>
                    <div class="form-group">
                        <label>"用户名"</label>
                        <input type="text" placeholder="选择用户名"
                            prop:value=move || username.get()
                            on:input=move |ev| username.set(event_target_value(&ev)) />
                    </div>
                    <div class="form-group">
                        <label>"密码（至少8位）"</label>
                        <input type="password" placeholder="设置密码"
                            prop:value=move || password.get()
                            on:input=move |ev| password.set(event_target_value(&ev)) />
                    </div>
                    { move || error.get().map(|e| view! { <p class="error-msg">{ e }</p> }) }
                    <button type="submit" class="btn-primary"
                        style="width:100%;margin-top:8px"
                        disabled=move || loading.get()>
                        { move || if loading.get() { "注册中…" } else { "注册" } }
                    </button>
                </form>
                <p style="text-align:center;margin-top:16px;color:var(--muted)">
                    "已有账号? " <a href="/login">"立即登录"</a>
                </p>
            </div>
        </div>
    }
}
