use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_params_map;
use uuid::Uuid;
use std::str::FromStr;

use shared::dto::categories::{CategoryResponse, CreateCategoryRequest};

use crate::api;

#[component]
pub fn CategoriesPage() -> impl IntoView {
    let params = use_params_map();
    let ledger_id = move || {
        params.with(|p| p.get("id").and_then(|id| Uuid::from_str(&id).ok()))
    };

    let show_form = create_rw_signal(false);
    let categories = LocalResource::new(move || {
        let lid = ledger_id();
        async move {
            match lid {
                None => Err("invalid ledger id".to_string()),
                Some(id) => api::get::<Vec<CategoryResponse>>(&format!("/ledgers/{id}/categories")).await,
            }
        }
    });

    view! {
        <div>
            <div class="page-header">
                <h1 class="page-title">"分类管理"</h1>
                <button class="btn-primary" on:click=move |_| show_form.set(true)>
                    "+ 新建分类"
                </button>
            </div>

            <Suspense fallback=|| view! { <div class="loading">"加载中…"</div> }>
                { move || categories.get().map(|res| match res.take() {
                    Err(e) => view! { <div class="error-msg">{ e }</div> }.into_any(),
                    Ok(list) => {
                        let expenses: Vec<_> = list.iter().filter(|c| c.category_type == "expense").collect();
                        let incomes:  Vec<_> = list.iter().filter(|c| c.category_type == "income").collect();
                        view! {
                            <div class="grid-2">
                                <div class="card">
                                    <h3 style="margin-bottom:12px;color:var(--danger)">"支出分类"</h3>
                                    { expenses.iter().map(|c| {
                                        let id      = c.id;
                                        let lid     = ledger_id();
                                        let name    = c.name.clone();
                                        let system  = c.is_system;
                                        let cats    = categories.clone();
                                        view! {
                                            <div style="display:flex;justify-content:space-between;padding:8px 0;border-bottom:1px solid var(--border)">
                                                <span>{ name }</span>
                                                { (!system).then(|| view! {
                                                    <button class="btn-danger" style="padding:4px 8px;font-size:12px"
                                                        on:click=move |_| {
                                                            if let Some(lid) = lid {
                                                                let c2 = cats.clone();
                                                                spawn_local(async move {
                                                                    let _ = api::delete::<serde_json::Value>(
                                                                        &format!("/ledgers/{lid}/categories/{id}")
                                                                    ).await;
                                                                    c2.refetch();
                                                                });
                                                            }
                                                        }>"删除"</button>
                                                })}
                                            </div>
                                        }
                                    }).collect_view() }
                                </div>
                                <div class="card">
                                    <h3 style="margin-bottom:12px;color:var(--success)">"收入分类"</h3>
                                    { incomes.iter().map(|c| {
                                        let id      = c.id;
                                        let lid     = ledger_id();
                                        let name    = c.name.clone();
                                        let system  = c.is_system;
                                        let cats    = categories.clone();
                                        view! {
                                            <div style="display:flex;justify-content:space-between;padding:8px 0;border-bottom:1px solid var(--border)">
                                                <span>{ name }</span>
                                                { (!system).then(|| view! {
                                                    <button class="btn-danger" style="padding:4px 8px;font-size:12px"
                                                        on:click=move |_| {
                                                            if let Some(lid) = lid {
                                                                let c2 = cats.clone();
                                                                spawn_local(async move {
                                                                    let _ = api::delete::<serde_json::Value>(
                                                                        &format!("/ledgers/{lid}/categories/{id}")
                                                                    ).await;
                                                                    c2.refetch();
                                                                });
                                                            }
                                                        }>"删除"</button>
                                                })}
                                            </div>
                                        }
                                    }).collect_view() }
                                </div>
                            </div>
                        }.into_any()
                    }
                })}
            </Suspense>

            { move || show_form.get().then(|| view! {
                <CreateCategoryModal
                    ledger_id=ledger_id().unwrap_or_default()
                    on_close=move || show_form.set(false)
                    on_saved=move || { show_form.set(false); categories.refetch(); }
                />
            })}
        </div>
    }
}

#[component]
fn CreateCategoryModal(
    ledger_id: Uuid,
    on_close: impl Fn() + 'static + Clone,
    on_saved: impl Fn() + 'static + Clone,
) -> impl IntoView {
    let name     = create_rw_signal(String::new());
    let cat_type = create_rw_signal("expense".to_string());
    let error    = create_rw_signal(Option::<String>::None);

    let save = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let req = CreateCategoryRequest {
            name: name.get(),
            category_type: cat_type.get(),
            parent_id: None,
            sort_order: None,
        };
        let on_s = on_saved.clone();
        spawn_local(async move {
            match api::post::<_, CategoryResponse>(
                &format!("/ledgers/{ledger_id}/categories"), &req
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
                <h2 class="modal-title">"新建分类"</h2>
                <form on:submit=save>
                    <div class="form-group">
                        <label>"名称"</label>
                        <input prop:value=move || name.get()
                            on:input=move |ev| name.set(event_target_value(&ev)) />
                    </div>
                    <div class="form-group">
                        <label>"类型"</label>
                        <select on:change=move |ev| cat_type.set(event_target_value(&ev))>
                            <option value="expense">"支出"</option>
                            <option value="income">"收入"</option>
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
