use leptos::prelude::*;
use leptos::children::ChildrenFn;
use leptos_router::hooks::use_navigate;
use crate::context::auth::use_auth;

#[component]
pub fn RestrictedRoute(children: ChildrenFn) -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();
    let is_authenticated = move || auth.is_authenticated.get();

    Effect::new(move |_| {
        if is_authenticated() {
            navigate("/home", Default::default());
        }
    });

    view! {
        <Show when=move || !is_authenticated() fallback=move || view! { <div/> }>
            {children()}
        </Show>
    }
}