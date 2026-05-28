use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;

use crate::state::AuthContext;

#[component]
pub fn Sidebar() -> impl IntoView {
    let auth_ctx = use_context::<AuthContext>().expect("AuthContext");
    let navigate = use_navigate();

    let username = move || {
        auth_ctx
            .0
            .get()
            .username
            .clone()
            .unwrap_or_else(|| "用户".into())
    };

    let logout = move |_| {
        auth_ctx.0.update(|s| {
            s.clear();
            *s = crate::state::AuthState::default();
        });
        navigate("/login", Default::default());
    };

    view! {
        <nav class="sidebar">
            <div class="sidebar-logo">"💰 LedgerRS"</div>
            <div class="sidebar-nav">
                <A href="/" exact=true>"🏠 总览"</A>
                <A href="/ledgers">"📒 账本"</A>
                <A href="/reports">"📊 报表"</A>
                <A href="/settings">"⚙️ 设置"</A>
            </div>
            <div class="sidebar-user">
                <div class="sidebar-username">{ username }</div>
                <button class="btn-ghost" style="width: 100%;" on:click=logout>
                    "退出登录"
                </button>
            </div>
        </nav>
    }
}
