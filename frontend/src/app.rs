
use leptos::*;
use leptos_router::{components::{Router}};
use crate::components::common::SnackbarGlobal;
use crate::context::auth::AuthProvider;
use crate::routes::AppRoutes;


#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <AuthProvider>
                <SnackbarGlobal />
                <AppRoutes/>
            </AuthProvider>
        </Router>
    }
}