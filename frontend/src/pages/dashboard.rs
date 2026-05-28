use leptos::prelude::*;
use leptos_router::components::A;

use shared::dto::ledgers::LedgerResponse;

use crate::api;

#[component]
pub fn DashboardPage() -> impl IntoView {
    let ledgers = LocalResource::new(|| async move {
        api::get::<Vec<LedgerResponse>>("/ledgers").await
    });

    view! {
        <div>
            <div class="page-header">
                <h1 class="page-title">"总览"</h1>
            </div>

            <Suspense fallback=|| view! { <div class="loading">"加载中…"</div> }>
                { move || ledgers.get().map(|res| match res.take() {
                    Err(e) => view! { <div class="error-msg">{ e }</div> }.into_any(),
                    Ok(list) => {
                        let total_balance: rust_decimal::Decimal =
                            list.iter().map(|l| l.balance).sum();
                        let total_income: rust_decimal::Decimal =
                            list.iter().map(|l| l.this_month_income).sum();
                        let total_expense: rust_decimal::Decimal =
                            list.iter().map(|l| l.this_month_expense).sum();

                        view! {
                            <div class="grid-3" style="margin-bottom:24px">
                                <div class="card stat-card">
                                    <div class="label">"总资产（本月）"</div>
                                    <div class="value">{ format!("{:.2}", total_balance) }</div>
                                </div>
                                <div class="card stat-card">
                                    <div class="label">"本月收入"</div>
                                    <div class="value income">{ format!("+ {:.2}", total_income) }</div>
                                </div>
                                <div class="card stat-card">
                                    <div class="label">"本月支出"</div>
                                    <div class="value expense">{ format!("- {:.2}", total_expense) }</div>
                                </div>
                            </div>

                            <h2 style="font-size:16px;font-weight:600;margin-bottom:12px">"我的账本"</h2>
                            <div class="grid-3">
                                { list.iter().map(|l| {
                                    let id   = l.id;
                                    let name = l.name.clone();
                                    let icon = l.icon.clone();
                                    let bal  = l.balance;
                                    let curr = l.base_currency.clone();
                                    view! {
                                        <A href=format!("/ledgers/{}/transactions", id)>
                                            <div class="card" style="cursor:pointer">
                                                <div style="font-size:28px;margin-bottom:8px">{ icon }</div>
                                                <div style="font-weight:600">{ name }</div>
                                                <div style="color:var(--muted);font-size:13px">{ curr }</div>
                                                <div style="font-size:18px;font-weight:700;margin-top:8px">
                                                    { format!("{:.2}", bal) }
                                                </div>
                                            </div>
                                        </A>
                                    }
                                }).collect_view() }
                            </div>
                        }.into_any()
                    }
                })}
            </Suspense>
        </div>
    }
}
