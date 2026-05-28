use leptos::prelude::*;

use shared::dto::ledgers::LedgerResponse;
use shared::dto::reports::{OverviewResponse, TrendResponse};

use crate::api;

#[component]
pub fn ReportsPage() -> impl IntoView {
    let ledgers = LocalResource::new(|| async move {
        api::get::<Vec<LedgerResponse>>("/ledgers").await
    });

    let selected_id = create_rw_signal(Option::<uuid::Uuid>::None);

    let overview = LocalResource::new(move || {
        let lid = selected_id.get();
        async move {
            match lid {
                None => Ok(None),
                Some(id) => {
                    let now = chrono::Utc::now();
                    let start = now - chrono::Duration::days(30);
                    api::get::<OverviewResponse>(
                        &format!("/ledgers/{id}/reports/overview?start={}&end={}",
                            start.to_rfc3339(), now.to_rfc3339())
                    ).await.map(Some)
                }
            }
        }
    });

    let trend = LocalResource::new(move || {
        let lid = selected_id.get();
        async move {
            match lid {
                None => Ok(None),
                Some(id) => {
                    api::get::<TrendResponse>(
                        &format!("/ledgers/{id}/reports/trend?months=6")
                    ).await.map(Some)
                }
            }
        }
    });

    view! {
        <div>
            <div class="page-header">
                <h1 class="page-title">"报表分析"</h1>
            </div>

            <div class="card" style="margin-bottom:16px">
                <label style="font-weight:500;margin-right:12px">"选择账本:"</label>
                <Suspense fallback=|| view! {}>
                    { move || ledgers.get().map(|res| {
                        let list = res.take().unwrap_or_default();
                        if selected_id.get().is_none() {
                            if let Some(first) = list.first() {
                                selected_id.set(Some(first.id));
                            }
                        }
                        view! {
                            <select on:change=move |ev| {
                                let v = event_target_value(&ev);
                                selected_id.set(uuid::Uuid::parse_str(&v).ok());
                            }>
                                { list.iter().map(|l| {
                                    let id_str = l.id.to_string();
                                    let name = l.name.clone();
                                    view! { <option value=id_str>{ name }</option> }
                                }).collect_view() }
                            </select>
                        }
                    })}
                </Suspense>
            </div>

            <Suspense fallback=|| view! { <div class="loading">"加载中…"</div> }>
                { move || overview.get().map(|res| {
                    let ov = res.take().ok().flatten();
                    ov.map(|o| view! {
                        <div class="grid-4" style="margin-bottom:24px">
                            <div class="card stat-card">
                                <div class="label">"收入"</div>
                                <div class="value income">{ format!("{:.2}", o.total_income) }</div>
                            </div>
                            <div class="card stat-card">
                                <div class="label">"支出"</div>
                                <div class="value expense">{ format!("{:.2}", o.total_expense) }</div>
                            </div>
                            <div class="card stat-card">
                                <div class="label">"净额"</div>
                                <div class="value">{ format!("{:.2}", o.net) }</div>
                            </div>
                            <div class="card stat-card">
                                <div class="label">"笔数"</div>
                                <div class="value">{ o.transaction_count }</div>
                            </div>
                        </div>
                    })
                })}
            </Suspense>

            <Suspense fallback=|| view! {}>
                { move || trend.get().map(|res| {
                    let tr = res.take().ok().flatten();
                    tr.map(|t| view! {
                        <div class="card">
                            <h2 style="font-size:16px;font-weight:600;margin-bottom:12px">"近6个月收支趋势"</h2>
                            <table>
                                <thead>
                                    <tr>
                                        <th>"月份"</th>
                                        <th>"收入"</th>
                                        <th>"支出"</th>
                                        <th>"净额"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    { t.months.iter().map(|m| view! {
                                        <tr>
                                            <td>{ m.year_month.clone() }</td>
                                            <td style="color:var(--success)">{ format!("{:.2}", m.income) }</td>
                                            <td style="color:var(--danger)">{ format!("{:.2}", m.expense) }</td>
                                            <td>{ format!("{:.2}", m.net) }</td>
                                        </tr>
                                    }).collect_view() }
                                </tbody>
                            </table>
                        </div>
                    })
                })}
            </Suspense>
        </div>
    }
}
