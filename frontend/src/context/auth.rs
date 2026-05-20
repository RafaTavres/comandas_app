use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use serde_json::Value;
use std::sync::Arc;

use crate::services::auth_service;
use crate::utils::snackbar::show_snackbar;

#[derive(Clone)]
pub struct AuthContext {
    pub is_authenticated: ReadSignal<bool>,
    pub user: ReadSignal<Option<Value>>,
    pub loading: ReadSignal<bool>,
    pub login: Arc<dyn Fn(String, String) + Send + Sync>,
    pub logout: Arc<dyn Fn() + Send + Sync>,
    pub is_token_expiring_soon: Arc<dyn Fn() -> bool + Send + Sync>,
}

#[component]
pub fn AuthProvider(children: Children) -> impl IntoView {
    let navigate = use_navigate();
    let login_navigate = navigate.clone();

    let (is_authenticated, set_is_authenticated) = signal(auth_service::is_authenticated());
    let (user, set_user) = signal(None::<Value>);
    let (loading, set_loading) = signal(true);

    leptos::task::spawn_local(async move {
        if auth_service::is_authenticated() {
            set_is_authenticated.set(true);

            if let Some(user_data) = auth_service::get_user_data::<Value>().await {
                set_user.set(Some(user_data));
            }
        } else {
            set_is_authenticated.set(false);
            set_user.set(None);
        }

        set_loading.set(false);
    });

    let login = Arc::new(move |cpf: String, senha: String| {
        let login_navigate = login_navigate.clone();

        leptos::task::spawn_local(async move {
            set_loading.set(true);
            let result = auth_service::login(&cpf, &senha).await;

            if result.success {
                set_is_authenticated.set(true);
                set_user.set(auth_service::get_user_data::<Value>().await);
                set_loading.set(false);
                login_navigate("/home", Default::default());
            } else {
                let message = result
                    .error
                    .unwrap_or_else(|| "Usuario ou senha invalidos!".to_string());

                set_loading.set(false);
                show_snackbar(&message, "error");
            }
        });
    });

    let logout = Arc::new(move || {
        set_is_authenticated.set(false);
        set_user.set(None);
        set_loading.set(false);
        auth_service::logout();
    });

    let is_token_expiring_soon = Arc::new(auth_service::is_token_expiring_soon);

    provide_context(AuthContext {
        is_authenticated: is_authenticated.into(),
        user: user.into(),
        loading: loading.into(),
        login,
        logout,
        is_token_expiring_soon,
    });

    view! {
        {children()}
    }
}

pub fn use_auth() -> AuthContext {
    use_context::<AuthContext>()
        .expect("AuthProvider nao encontrado. Envolva sua aplicacao com <AuthProvider>.")
}
