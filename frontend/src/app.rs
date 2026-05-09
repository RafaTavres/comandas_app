
use leptos::*;
use leptos_router::{components::{Router, Routes, Route}, path};
use crate::components::forms::login_form::LoginForm;
use crate::components::routes::{PrivateRoute, RestrictedRoute};
use crate::context::auth::AuthProvider;
use crate::pages::{Dashboard, NotFound, ProdutoForm, ProdutoList};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <AuthProvider>
                <Routes fallback=|| view! { <NotFound/> }>
                    // A rota raiz exibe o formulário de login imediatamente
                    <Route path=path!("/") view=move || view! { <RestrictedRoute><LoginForm/></RestrictedRoute> } />
                    <Route path=path!("/login") view=move || view! { <RestrictedRoute><LoginForm/></RestrictedRoute> } />
                    <Route path=path!("/home") view=move || view! { <PrivateRoute><Dashboard/></PrivateRoute> } />
                    <Route path=path!("/dashboard") view=move || view! { <PrivateRoute><Dashboard/></PrivateRoute> } />
                    <Route path=path!("/produtos") view=move || view! { <PrivateRoute><ProdutoList/></PrivateRoute> } />
                    <Route path=path!("/produtos/novo") view=move || view! { <PrivateRoute><ProdutoForm/></PrivateRoute> } />
                </Routes>
            </AuthProvider>
        </Router>
    }
}