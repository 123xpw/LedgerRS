use leptos::prelude::*;
use leptos::task::spawn_local;

use shared::dto::auth::{ChangePasswordRequest, UpdateProfileRequest, UserResponse};

use crate::api;

#[component]
pub fn SettingsPage() -> impl IntoView {
    let me = LocalResource::new(|| async move {
        api::get::<UserResponse>("/auth/me").await
    });

    view! {
        <div>
            <div class="page-header">
                <h1 class="page-title">"账户设置"</h1>
            </div>
            <Suspense fallback=|| view! { <div class="loading">"加载中…"</div> }>
                { move || me.get().map(|res| match res.take() {
                    Err(e) => view! { <div class="error-msg">{ e }</div> }.into_any(),
                    Ok(user) => view! {
                        <div class="grid-2">
                            <ProfileForm user=user.clone() on_saved=move || me.refetch() />
                            <PasswordForm />
                        </div>
                    }.into_any()
                })}
            </Suspense>
        </div>
    }
}

#[component]
fn ProfileForm(
    user: UserResponse,
    on_saved: impl Fn() + 'static + Clone,
) -> impl IntoView {
    let username = create_rw_signal(user.username.clone());
    let timezone = create_rw_signal(user.timezone.clone());
    let currency = create_rw_signal(user.default_currency.clone());
    let error    = create_rw_signal(Option::<String>::None);
    let success  = create_rw_signal(false);

    let save = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let req = UpdateProfileRequest {
            username: Some(username.get()),
            default_currency: Some(currency.get()),
            timezone: Some(timezone.get()),
        };
        let on_s = on_saved.clone();
        spawn_local(async move {
            match api::patch::<_, UserResponse>("/auth/me", &req).await {
                Ok(_) => { success.set(true); on_s(); }
                Err(e) => error.set(Some(e)),
            }
        });
    };

    view! {
        <div class="card">
            <h2 style="font-size:16px;font-weight:600;margin-bottom:16px">"基本信息"</h2>
            <form on:submit=save>
                <div class="form-group">
                    <label>"邮箱（不可修改）"</label>
                    <input type="email" disabled prop:value=user.email.clone() />
                </div>
                <div class="form-group">
                    <label>"用户名"</label>
                    <input prop:value=move || username.get()
                        on:input=move |ev| username.set(event_target_value(&ev)) />
                </div>
                <div class="form-group">
                    <label>"默认币种"</label>
                    <select on:change=move |ev| currency.set(event_target_value(&ev))>
                        <option value="CNY">"CNY - 人民币"</option>
                        <option value="USD">"USD - 美元"</option>
                        <option value="EUR">"EUR - 欧元"</option>
                        <option value="JPY">"JPY - 日元"</option>
                        <option value="GBP">"GBP - 英镑"</option>
                    </select>
                </div>
                <div class="form-group">
                    <label>"时区"</label>
                    <select on:change=move |ev| timezone.set(event_target_value(&ev))>
                        <option value="Asia/Shanghai">"Asia/Shanghai (UTC+8)"</option>
                        <option value="America/New_York">"America/New_York (UTC-5)"</option>
                        <option value="Europe/London">"Europe/London (UTC+0)"</option>
                        <option value="Asia/Tokyo">"Asia/Tokyo (UTC+9)"</option>
                        <option value="UTC">"UTC"</option>
                    </select>
                </div>
                { move || error.get().map(|e| view! { <p class="error-msg">{ e }</p> }) }
                { move || success.get().then(|| view! {
                    <p style="color:var(--success);font-size:13px">"保存成功！"</p>
                })}
                <button type="submit" class="btn-primary">"保存"</button>
            </form>
        </div>
    }
}

#[component]
fn PasswordForm() -> impl IntoView {
    let current  = create_rw_signal(String::new());
    let new_pw   = create_rw_signal(String::new());
    let confirm  = create_rw_signal(String::new());
    let error    = create_rw_signal(Option::<String>::None);
    let success  = create_rw_signal(false);

    let save = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        if new_pw.get() != confirm.get() {
            error.set(Some("两次输入的新密码不一致".into()));
            return;
        }
        if new_pw.get().len() < 8 {
            error.set(Some("新密码至少8位".into()));
            return;
        }
        let req = ChangePasswordRequest {
            current_password: current.get(),
            new_password: new_pw.get(),
        };
        spawn_local(async move {
            match api::patch::<_, serde_json::Value>("/auth/me/password", &req).await {
                Ok(_) => {
                    current.set(String::new());
                    new_pw.set(String::new());
                    confirm.set(String::new());
                    success.set(true);
                }
                Err(e) => error.set(Some(e)),
            }
        });
    };

    view! {
        <div class="card">
            <h2 style="font-size:16px;font-weight:600;margin-bottom:16px">"修改密码"</h2>
            <form on:submit=save>
                <div class="form-group">
                    <label>"当前密码"</label>
                    <input type="password"
                        prop:value=move || current.get()
                        on:input=move |ev| current.set(event_target_value(&ev)) />
                </div>
                <div class="form-group">
                    <label>"新密码"</label>
                    <input type="password"
                        prop:value=move || new_pw.get()
                        on:input=move |ev| new_pw.set(event_target_value(&ev)) />
                </div>
                <div class="form-group">
                    <label>"确认新密码"</label>
                    <input type="password"
                        prop:value=move || confirm.get()
                        on:input=move |ev| confirm.set(event_target_value(&ev)) />
                </div>
                { move || error.get().map(|e| view! { <p class="error-msg">{ e }</p> }) }
                { move || success.get().then(|| view! {
                    <p style="color:var(--success);font-size:13px">"密码修改成功！"</p>
                })}
                <button type="submit" class="btn-primary">"修改密码"</button>
            </form>
        </div>
    }
}
