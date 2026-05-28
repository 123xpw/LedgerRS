use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_params_map;
use uuid::Uuid;
use std::str::FromStr;

use shared::dto::pagination::PaginatedResponse;
use shared::dto::transactions::{CreateTransactionRequest, TransactionResponse};
use shared::dto::categories::CategoryResponse;
use shared::types::TransactionType;

use crate::api;

#[component]
pub fn TransactionsPage() -> impl IntoView {
    let params = use_params_map();
    let ledger_id = move || {
        params.with(|p| p.get("id").and_then(|id| Uuid::from_str(&id).ok()))
    };

    let page = create_rw_signal(1i64);
    let show_form = create_rw_signal(false);

    let transactions = LocalResource::new(move || {
        let (lid, pg) = (ledger_id(), page.get());
        async move {
            match lid {
                None => Err("invalid ledger id".to_string()),
                Some(id) => {
                    api::get::<PaginatedResponse<TransactionResponse>>(
                        &format!("/ledgers/{id}/transactions?page={pg}&per_page=20")
                    ).await
                }
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
                <h1 class="page-title">"账目记录"</h1>
                <button class="btn-primary" on:click=move |_| show_form.set(true)>
                    "+ 新增账目"
                </button>
            </div>

            <Suspense fallback=|| view! { <div class="loading">"加载中…"</div> }>
                { move || transactions.get().map(|res| match res.take() {
                    Err(e) => view! { <div class="error-msg">{ e }</div> }.into_any(),
                    Ok(paged) => {
                        let items = paged.items.clone();
                        view! {
                            <div class="card">
                                <table>
                                    <thead>
                                        <tr>
                                            <th>"时间"</th>
                                            <th>"类型"</th>
                                            <th>"金额"</th>
                                            <th>"币种"</th>
                                            <th>"分类"</th>
                                            <th>"备注"</th>
                                            <th>"操作"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        { items.iter().map(|tx| {
                                            let id = tx.id;
                                            let lid = ledger_id();
                                            let t = transactions.clone();
                                            let type_class = match tx.transaction_type.as_str() {
                                                "income"   => "badge-income",
                                                "expense"  => "badge-expense",
                                                _          => "badge-transfer",
                                            };
                                            let type_label = match tx.transaction_type.as_str() {
                                                "income"   => "收入",
                                                "expense"  => "支出",
                                                _          => "转账",
                                            };
                                            let occurred = tx.occurred_at.format("%Y-%m-%d").to_string();
                                            let amount  = format!("{:.2}", tx.amount);
                                            let currency = tx.currency.clone();
                                            let cat_name = tx.category_name.clone().unwrap_or_default();
                                            let note = tx.note.clone().unwrap_or_default();
                                            view! {
                                                <tr>
                                                    <td>{ occurred }</td>
                                                    <td><span class=format!("badge {type_class}")>{ type_label }</span></td>
                                                    <td>{ amount }</td>
                                                    <td>{ currency }</td>
                                                    <td>{ cat_name }</td>
                                                    <td>{ note }</td>
                                                    <td>
                                                        <button class="btn-danger"
                                                            on:click=move |_| {
                                                                if let Some(lid) = lid {
                                                                    let t2 = t.clone();
                                                                    spawn_local(async move {
                                                                        let _ = api::delete::<serde_json::Value>(
                                                                            &format!("/ledgers/{lid}/transactions/{id}")
                                                                        ).await;
                                                                        t2.refetch();
                                                                    });
                                                                }
                                                            }>
                                                            "删除"
                                                        </button>
                                                    </td>
                                                </tr>
                                            }
                                        }).collect_view() }
                                    </tbody>
                                </table>
                                <div style="display:flex;justify-content:space-between;align-items:center;margin-top:12px">
                                    <span style="color:var(--muted);font-size:13px">
                                        { format!("共 {} 条", paged.meta.total) }
                                    </span>
                                    <div style="display:flex;gap:8px">
                                        <button class="btn-ghost"
                                            disabled=move || page.get() <= 1
                                            on:click=move |_| page.update(|p| *p -= 1)>
                                            "上一页"
                                        </button>
                                        <span style="padding:8px">
                                            { move || format!("{} / {}", page.get(), paged.meta.total_pages) }
                                        </span>
                                        <button class="btn-ghost"
                                            disabled=move || page.get() >= paged.meta.total_pages
                                            on:click=move |_| page.update(|p| *p += 1)>
                                            "下一页"
                                        </button>
                                    </div>
                                </div>
                            </div>
                        }.into_any()
                    }
                })}
            </Suspense>

            { move || show_form.get().then(|| {
                let cats = categories.get().map(|sw| sw.take()).unwrap_or(Ok(vec![])).unwrap_or_default();
                view! {
                    <CreateTransactionModal
                        ledger_id=ledger_id().unwrap_or_default()
                        categories=cats
                        on_close=move || show_form.set(false)
                        on_saved=move || { show_form.set(false); transactions.refetch(); }
                    />
                }
            })}
        </div>
    }
}

#[component]
fn CreateTransactionModal(
    ledger_id: Uuid,
    categories: Vec<CategoryResponse>,
    on_close: impl Fn() + 'static + Clone,
    on_saved: impl Fn() + 'static + Clone,
) -> impl IntoView {
    let tx_type  = create_rw_signal("expense".to_string());
    let amount   = create_rw_signal(String::new());
    let currency = create_rw_signal("CNY".to_string());
    let cat_id   = create_rw_signal(Option::<Uuid>::None);
    let note     = create_rw_signal(String::new());
    let error    = create_rw_signal(Option::<String>::None);

    let save = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let amt: rust_decimal::Decimal = match amount.get().parse() {
            Ok(v) => v,
            Err(_) => { error.set(Some("金额格式错误".into())); return; }
        };
        let tt = match tx_type.get().as_str() {
            "income"  => TransactionType::Income,
            "expense" => TransactionType::Expense,
            _         => TransactionType::Transfer,
        };
        let req = CreateTransactionRequest {
            transaction_type: tt,
            amount: amt,
            currency: currency.get(),
            category_id: cat_id.get(),
            tag_ids: vec![],
            note: if note.get().is_empty() { None } else { Some(note.get()) },
            occurred_at: chrono::Utc::now(),
            custom_rate: None,
            target_ledger_id: None,
            target_currency: None,
            target_amount: None,
        };
        let on_s = on_saved.clone();
        spawn_local(async move {
            match api::post::<_, TransactionResponse>(
                &format!("/ledgers/{ledger_id}/transactions"), &req
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
                <h2 class="modal-title">"新增账目"</h2>
                <form on:submit=save>
                    <div class="form-group">
                        <label>"类型"</label>
                        <select on:change=move |ev| tx_type.set(event_target_value(&ev))>
                            <option value="expense">"支出"</option>
                            <option value="income">"收入"</option>
                            <option value="transfer">"转账"</option>
                        </select>
                    </div>
                    <div class="form-group">
                        <label>"金额"</label>
                        <input type="number" step="0.01" placeholder="0.00"
                            prop:value=move || amount.get()
                            on:input=move |ev| amount.set(event_target_value(&ev)) />
                    </div>
                    <div class="form-group">
                        <label>"币种"</label>
                        <select on:change=move |ev| currency.set(event_target_value(&ev))>
                            <option value="CNY">"CNY"</option>
                            <option value="USD">"USD"</option>
                            <option value="EUR">"EUR"</option>
                            <option value="JPY">"JPY"</option>
                        </select>
                    </div>
                    <div class="form-group">
                        <label>"分类"</label>
                        <select on:change=move |ev| {
                            let v = event_target_value(&ev);
                            cat_id.set(if v.is_empty() { None } else { Uuid::from_str(&v).ok() });
                        }>
                            <option value="">"-- 不选分类 --"</option>
                            { categories.iter().map(|c| {
                                let id_str = c.id.to_string();
                                let name = c.name.clone();
                                view! { <option value=id_str>{ name }</option> }
                            }).collect_view() }
                        </select>
                    </div>
                    <div class="form-group">
                        <label>"备注"</label>
                        <input prop:value=move || note.get()
                            on:input=move |ev| note.set(event_target_value(&ev)) />
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
