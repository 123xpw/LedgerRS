use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_params_map;
use uuid::Uuid;
use std::str::FromStr;

use shared::dto::budgets::{BudgetResponse, CreateBudgetRequest};
use shared::dto::categories::CategoryResponse;

use crate::api;

#[component]
pub fn BudgetsPage() -> impl IntoView {
    let params = use_params_map();
    let ledger_id = move || {
        params.with(|p| p.get("id").and_then(|id| Uuid::from_str(&id).ok()))
    };

    let show_form = create_rw_signal(false);
    let budgets = LocalResource::new(move || {
        let lid = ledger_id();
        async move {
            match lid {
                None => Err("invalid ledger id".to_string()),
                Some(id) => api::get::<Vec<BudgetResponse>>(&format!("/ledgers/{id}/budgets")).await,
            }
        }
    });
    let categories = LocalResource::new(move || {
        let lid = ledger_id();
        async move {
            match lid {
                None => Ok(vec![]),
                Some(id) => api::get::<Vec<CategoryResponse>>(&format!("/ledgers/{id}/categories")).await,
            }
        }
    });

    view! {
        <div>
            <div class="page-header">
                <h1 class="page-title">"预算管理"</h1>
                <button class="btn-primary" on:click=move |_| show_form.set(true)>
                    "+ 新建预算"
                </button>
            </div>

            <Suspense fallback=|| view! { <div class="loading">"加载中…"</div> }>
                { move || budgets.get().map(|res| match res.take() {
                    Err(e) => view! { <div class="error-msg">{ e }</div> }.into_any(),
                    Ok(list) => view! {
                        <div class="grid-3">
                            { list.iter().map(|b| {
                                let id  = b.id;
                                let lid = ledger_id();
                                let cat = b.category_id.map(|_| "分类预算").unwrap_or("总预算");
                                let per = b.period.clone();
                                let amt = format!("{:.2}", b.amount);
                                let spent = format!("{:.2}", b.spent);
                                let usage = b.usage_rate;
                                let bar_color = if usage >= 90.0 { "var(--danger)" }
                                                else if usage >= 70.0 { "#f59e0b" }
                                                else { "var(--success)" };
                                let buds = budgets.clone();
                                view! {
                                    <div class="card">
                                        <div style="display:flex;justify-content:space-between">
                                            <span style="font-weight:600">{ cat }</span>
                                            <span class="badge badge-income">{ per }</span>
                                        </div>
                                        <div style="margin:12px 0">
                                            <div style="display:flex;justify-content:space-between;font-size:13px;margin-bottom:4px">
                                                <span>"已用: "{ spent }</span>
                                                <span>"预算: "{ amt }</span>
                                            </div>
                                            <div style="height:8px;background:var(--border);border-radius:4px">
                                                <div style=format!(
                                                    "height:100%;width:{}%;background:{};border-radius:4px;transition:width .3s",
                                                    usage.min(100.0), bar_color
                                                )></div>
                                            </div>
                                            <div style="font-size:12px;color:var(--muted);margin-top:4px">
                                                { format!("{:.1}%", usage) }
                                            </div>
                                        </div>
                                        <button class="btn-danger" style="width:100%"
                                            on:click=move |_| {
                                                if let Some(lid) = lid {
                                                    let b2 = buds.clone();
                                                    spawn_local(async move {
                                                        let _ = api::delete::<serde_json::Value>(
                                                            &format!("/ledgers/{lid}/budgets/{id}")
                                                        ).await;
                                                        b2.refetch();
                                                    });
                                                }
                                            }>"删除"</button>
                                    </div>
                                }
                            }).collect_view() }
                        </div>
                    }.into_any(),
                })}
            </Suspense>

            { move || show_form.get().then(|| {
                let cats = categories.get().map(|sw| sw.take()).unwrap_or(Ok(vec![])).unwrap_or_default();
                view! {
                    <CreateBudgetModal
                        ledger_id=ledger_id().unwrap_or_default()
                        categories=cats
                        on_close=move || show_form.set(false)
                        on_saved=move || { show_form.set(false); budgets.refetch(); }
                    />
                }
            })}
        </div>
    }
}

#[component]
fn CreateBudgetModal(
    ledger_id: Uuid,
    categories: Vec<CategoryResponse>,
    on_close: impl Fn() + 'static + Clone,
    on_saved: impl Fn() + 'static + Clone,
) -> impl IntoView {
    let cat_id = create_rw_signal(Option::<Uuid>::None);
    let period = create_rw_signal("month".to_string());
    let amount = create_rw_signal(String::new());
    let error  = create_rw_signal(Option::<String>::None);

    let save = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let amt: rust_decimal::Decimal = match amount.get().parse() {
            Ok(v) => v,
            Err(_) => { error.set(Some("金额格式错误".into())); return; }
        };
        let req = CreateBudgetRequest {
            category_id: cat_id.get(),
            period: period.get(),
            amount: amt,
        };
        let on_s = on_saved.clone();
        spawn_local(async move {
            match api::post::<_, BudgetResponse>(
                &format!("/ledgers/{ledger_id}/budgets"), &req
            ).await {
                Ok(_) => on_s(),
                Err(e) => error.set(Some(e)),
            }
        });
    };

    let close = on_close.clone();
    view! {
        <div class="modal-overlay" on:click=move |_| close()>
            <div class="modal" on:click=|ev| ev.stop_propagation()>
                <h2 class="modal-title">"新建预算"</h2>
                <form on:submit=save>
                    <div class="form-group">
                        <label>"分类（留空为总预算）"</label>
                        <select on:change=move |ev| {
                            let v = event_target_value(&ev);
                            cat_id.set(if v.is_empty() { None } else { Uuid::from_str(&v).ok() });
                        }>
                            <option value="">"-- 总预算 --"</option>
                            { categories.iter().map(|c| {
                                let id_str = c.id.to_string();
                                let name = c.name.clone();
                                view! { <option value=id_str>{ name }</option> }
                            }).collect_view() }
                        </select>
                    </div>
                    <div class="form-group">
                        <label>"周期"</label>
                        <select on:change=move |ev| period.set(event_target_value(&ev))>
                            <option value="month">"月度"</option>
                            <option value="week">"周度"</option>
                            <option value="year">"年度"</option>
                        </select>
                    </div>
                    <div class="form-group">
                        <label>"预算金额"</label>
                        <input type="number" step="0.01" placeholder="0.00"
                            prop:value=move || amount.get()
                            on:input=move |ev| amount.set(event_target_value(&ev)) />
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
