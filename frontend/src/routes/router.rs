use leptos::prelude::*;
use leptos_router::{components::{Routes, Route}, path};
use leptos_router::hooks::use_navigate;
use crate::context::auth::use_auth;

use crate::components::forms::login_form::LoginForm;
use crate::pages::{ClienteForm, ClienteList, Dashboard, FuncionarioForm, FuncionarioList, NotFound, ProdutoForm, ProdutoList};
use super::{PrivateRoute, RestrictedRoute};

#[component]
pub fn AppRoutes() -> impl IntoView {

    let auth = use_auth();
    let navigate = use_navigate();
    let is_authenticated = move || auth.is_authenticated.get();

    Effect::new(move |_| {
        if is_authenticated() {
            navigate("/home", Default::default());
        }
    });

    view! {
      <Suspense fallback=|| view! { <div>"Carregando..."</div> }>
         <Routes fallback=|| view! { <NotFound/> }>
            <Route path=path!("/") view=move || view! { <RestrictedRoute><LoginForm/></RestrictedRoute> } />
            <Route path=path!("/login") view=move || view! { <RestrictedRoute><LoginForm/></RestrictedRoute> } />
            <Route path=path!("/home") view=move || view! { <PrivateRoute><Dashboard/></PrivateRoute> } />
            <Route path=path!("/dashboard") view=move || view! { <PrivateRoute><Dashboard/></PrivateRoute> } />
            <Route path=path!("/produtos") view=move || view! { <PrivateRoute><ProdutoList/></PrivateRoute> } />
            <Route path=path!("/produtos/novo") view=move || view! { <PrivateRoute><ProdutoForm/></PrivateRoute> } />
            <Route path=path!("/funcionarios") view=move || view! { <PrivateRoute><FuncionarioList/></PrivateRoute> } />
            <Route path=path!("/funcionarios/novo") view=move || view! { <PrivateRoute><FuncionarioForm/></PrivateRoute> } />
            <Route path=path!("/clientes") view=move || view! { <PrivateRoute><ClienteList/></PrivateRoute> } />
            <Route path=path!("/clientes/novo") view=move || view! { <PrivateRoute><ClienteForm/></PrivateRoute> } />
         </Routes>
      </Suspense>
    }
}