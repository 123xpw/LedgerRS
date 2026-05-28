mod api;
mod components;
mod pages;
mod state;

use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_navigate;
use leptos_router::path;
use wasm_bindgen::prelude::wasm_bindgen;

use pages::{
    auth::{LoginPage, RegisterPage},
    budgets::BudgetsPage,
    categories::CategoriesPage,
    dashboard::DashboardPage,
    ledgers::LedgersPage,
    reports::ReportsPage,
    settings::SettingsPage,
    transactions::TransactionsPage,
};
use state::AuthContext;

#[wasm_bindgen(start)]
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let auth = RwSignal::new(state::AuthState::load_from_storage());
    provide_context(AuthContext(auth));

    view! {
        <Router>
            <Routes fallback=|| view! { <p>"页面未找到"</p> }>
                <Route path=path!("/login")    view=LoginPage />
                <Route path=path!("/register") view=RegisterPage />
                <ParentRoute path=path!("/") view=ProtectedLayout>
                    <Route path=path!("")                          view=DashboardPage />
                    <Route path=path!("ledgers")                   view=LedgersPage />
                    <Route path=path!("ledgers/:id/transactions")  view=TransactionsPage />
                    <Route path=path!("ledgers/:id/categories")    view=CategoriesPage />
                    <Route path=path!("ledgers/:id/budgets")       view=BudgetsPage />
                    <Route path=path!("reports")                   view=ReportsPage />
                    <Route path=path!("settings")                  view=SettingsPage />
                </ParentRoute>
            </Routes>
        </Router>
    }
}

#[component]
fn ProtectedLayout() -> impl IntoView {
    let auth_ctx = use_context::<AuthContext>().expect("AuthContext missing");
    let navigate = use_navigate();

    Effect::new(move |_| {
        if auth_ctx.0.get().token.is_none() {
            navigate("/login", Default::default());
        }
    });

    view! {
        <div class="layout">
            <components::Sidebar />
            <main class="main-content">
                <Outlet />
            </main>
        </div>
    }
}
