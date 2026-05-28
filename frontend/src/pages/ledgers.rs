use leptos::prelude::*;
use leptos::task::spawn_local;

use shared::dto::ledgers::{CreateLedgerRequest, LedgerResponse, UpdateLedgerRequest};

use crate::api;

#[component]
pub fn LedgersPage() -> impl IntoView {
    let ledgers = LocalResource::new(|| async move {
        api::get::<Vec<LedgerResponse>>("/ledgers").await
    });

    let show_create = create_rw_signal(false);
    let edit_target = create_rw_signal(Option::<LedgerResponse>::None);

    view! {
        <div>
            <div class="page-header">
                <h1 class="page-title">"账本管理"</h1>
                <button class="btn-primary" on:click=move |_| show_create.set(true)>
                    "+ 新建账本"
                </button>
            </div>

            <Suspense fallback=|| view! { <div class="loading">"加载中…"</div> }>
                { move || ledgers.get().map(|res| match res.take() {
                    Err(e) => view! { <div class="error-msg">{ e }</div> }.into_any(),
                    Ok(list) => view! {
                        <div class="grid-3">
                            { list.iter().map(|l| {
                                let l2 = l.clone();
                                let l3 = l.clone();
                                let name = l.name.clone();
                                let icon = l.icon.clone();
                                let curr = l.base_currency.clone();
                                let bal  = l.balance;
                                view! {
                                    <div class="card">
                                        <div style="display:flex;align-items:center;gap:10px;margin-bottom:12px">
                                            <span style="font-size:28px">{ icon }</span>
                                            <div>
                                                <div style="font-weight:600">{ name }</div>
                                                <div style="color:var(--muted);font-size:12px">{ curr }</div>
                                            </div>
                                        </div>
                                        <div style="font-size:20px;font-weight:700;margin-bottom:12px">
                                            { format!("{:.2}", bal) }
                                        </div>
                                        <div style="display:flex;gap:8px">
                                            <button class="btn-ghost"
                                                on:click=move |_| edit_target.set(Some(l2.clone()))>
                                                "编辑"
                                            </button>
                                            <button class="btn-danger"
                                                on:click={
                                                    let id = l3.id;
                                                    let r = ledgers.clone();
                                                    move |_| {
                                                        let r2 = r.clone();
                                                        spawn_local(async move {
                                                            if let Ok(_) = api::delete::<serde_json::Value>(
                                                                &format!("/ledgers/{id}")
                                                            ).await {
                                                                r2.refetch();
                                                            }
                                                        });
                                                    }
                                                }>
                                                "删除"
                                            </button>
                                        </div>
                                    </div>
                                }
                            }).collect_view() }
                        </div>
                    }.into_any(),
                })}
            </Suspense>

            { move || show_create.get().then(|| view! {
                <CreateLedgerModal
                    on_close=move || show_create.set(false)
                    on_saved=move || { show_create.set(false); ledgers.refetch(); }
                />
            })}

            { move || edit_target.get().map(|target| view! {
                <EditLedgerModal
                    ledger=target.clone()
                    on_close=move || edit_target.set(None)
                    on_saved=move || { edit_target.set(None); ledgers.refetch(); }
                />
            })}
        </div>
    }
}

#[component]
fn CreateLedgerModal(
    on_close: impl Fn() + 'static + Clone,
    on_saved: impl Fn() + 'static + Clone,
) -> impl IntoView {
    let name     = create_rw_signal(String::new());
    let icon     = create_rw_signal("💰".to_string());
    let currency = create_rw_signal("CNY".to_string());
    let error    = create_rw_signal(Option::<String>::None);

    let save = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let req = CreateLedgerRequest {
            name: name.get(),
            icon: Some(icon.get()),
            base_currency: currency.get(),
            initial_balance: None,
        };
        let on_s = on_saved.clone();
        spawn_local(async move {
            match api::post::<_, LedgerResponse>("/ledgers", &req).await {
                Ok(_) => on_s(),
                Err(e) => error.set(Some(e)),
            }
        });
    };

    let close = on_close.clone();
    view! {
        <div class="modal-overlay" on:click=move |_| close()>
            <div class="modal" on:click=|ev| ev.stop_propagation()>
                <h2 class="modal-title">"新建账本"</h2>
                <form on:submit=save>
                    <div class="form-group">
                        <label>"名称"</label>
                        <input prop:value=move || name.get()
                            on:input=move |ev| name.set(event_target_value(&ev)) />
                    </div>
                    <div class="form-group">
                        <label>"图标（emoji）"</label>
                        <input prop:value=move || icon.get()
                            on:input=move |ev| icon.set(event_target_value(&ev)) />
                    </div>
                    <div class="form-group">
                        <label>"基准币种"</label>
                        <select on:change=move |ev| currency.set(event_target_value(&ev))>
                            <option value="CNY">"CNY - 人民币"</option>
                            <option value="USD">"USD - 美元"</option>
                            <option value="EUR">"EUR - 欧元"</option>
                            <option value="JPY">"JPY - 日元"</option>
                            <option value="GBP">"GBP - 英镑"</option>
                            <option value="HKD">"HKD - 港元"</option>
                        </select>
                    </div>
                    { move || error.get().map(|e| view! { <p class="error-msg">{ e }</p> }) }
                    <div style="display:flex;gap:8px;justify-content:flex-end;margin-top:16px">
                        <button type="button" class="btn-ghost" on:click=move |_| on_close()>
                            "取消"
                        </button>
                        <button type="submit" class="btn-primary">"保存"</button>
                    </div>
                </form>
            </div>
        </div>
    }
}

#[component]
fn EditLedgerModal(
    ledger: LedgerResponse,
    on_close: impl Fn() + 'static + Clone,
    on_saved: impl Fn() + 'static + Clone,
) -> impl IntoView {
    let name  = create_rw_signal(ledger.name.clone());
    let icon  = create_rw_signal(ledger.icon.clone());
    let error = create_rw_signal(Option::<String>::None);
    let id    = ledger.id;

    let save = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let req = UpdateLedgerRequest {
            name: Some(name.get()),
            icon: Some(icon.get()),
            initial_balance: None,
        };
        let on_s = on_saved.clone();
        spawn_local(async move {
            match api::patch::<_, LedgerResponse>(&format!("/ledgers/{id}"), &req).await {
                Ok(_) => on_s(),
                Err(e) => error.set(Some(e)),
            }
        });
    };

    let close = on_close.clone();
    view! {
        <div class="modal-overlay" on:click=move |_| close()>
            <div class="modal" on:click=|ev| ev.stop_propagation()>
                <h2 class="modal-title">"编辑账本"</h2>
                <form on:submit=save>
                    <div class="form-group">
                        <label>"名称"</label>
                        <input prop:value=move || name.get()
                            on:input=move |ev| name.set(event_target_value(&ev)) />
                    </div>
                    <div class="form-group">
                        <label>"图标（emoji）"</label>
                        <input prop:value=move || icon.get()
                            on:input=move |ev| icon.set(event_target_value(&ev)) />
                    </div>
                    { move || error.get().map(|e| view! { <p class="error-msg">{ e }</p> }) }
                    <div style="display:flex;gap:8px;justify-content:flex-end;margin-top:16px">
                        <button type="button" class="btn-ghost" on:click=move |_| on_close()>
                            "取消"
                        </button>
                        <button type="submit" class="btn-primary">"保存"</button>
                    </div>
                </form>
            </div>
        </div>
    }
}
