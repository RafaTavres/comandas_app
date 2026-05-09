
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use std::sync::Arc;
use web_sys::{window as web_window, Storage, Window as WebWindow};

#[derive(Clone)]
pub struct AuthContext {
    pub is_authenticated: ReadSignal<bool>,
    pub login: Arc<dyn Fn(String, String) + Send + Sync>,
    pub logout: Arc<dyn Fn() + Send + Sync>,
}

#[component]
pub fn AuthProvider(children: Children) -> impl IntoView {
    let navigate = use_navigate();
    let login_navigate = navigate.clone();
    let logout_navigate = navigate.clone();

    let initial_is_authenticated = match web_window().and_then(|w: WebWindow| -> Option<Storage> { w.session_storage().ok().flatten() }) {
        Some(storage) => storage.get_item("loginRealizado").ok().flatten().as_deref() == Some("true"),
        None => false,
    };

    let (is_authenticated, set_is_authenticated) = signal(initial_is_authenticated);

    let login = Arc::new(move |cpf: String, senha: String| {
        if cpf == "123" && senha == "123123" {
            set_is_authenticated.set(true);
            if let Some(storage) = web_window().and_then(|w: WebWindow| -> Option<Storage> { w.session_storage().ok().flatten() }) {
                _ = storage.set_item("loginRealizado", "true");
            }
            login_navigate("/home", Default::default());
        } else {
            window().alert_with_message("Usuário ou senha inválidos!").unwrap();
        }
    });

    let logout = Arc::new(move || {
        set_is_authenticated.set(false);
        if let Some(storage) = web_window().and_then(|w: WebWindow| -> Option<Storage> { w.session_storage().ok().flatten() }) {
            _ = storage.remove_item("loginRealizado");
        }
        logout_navigate("/login", Default::default());
    });

    let context = AuthContext {
        is_authenticated: is_authenticated.into(),
        login,
        logout,
    };

    provide_context(context);

    view! {
        {children()}
    }
}

pub fn use_auth() -> AuthContext {
    use_context::<AuthContext>()
        .expect("AuthProvider não encontrado. Envolva sua aplicação com <AuthProvider>.")
}