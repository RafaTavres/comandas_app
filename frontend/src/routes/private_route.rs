use leptos::children::ChildrenFn;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::{
    context::auth::use_auth,
    utils::{
        snackbar::show_snackbar,
        user_groups::has_group,
    },
};

#[component]
pub fn PrivateRoute(
    #[prop(optional)] allowed_groups: Option<Vec<i32>>,
    children: ChildrenFn,
) -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();
    let allowed_groups_for_effect = allowed_groups.clone();
    let allowed_groups_for_view = allowed_groups.clone();

    Effect::new(move |_| {
        if auth.loading.get() {
            return;
        }

        if !auth.is_authenticated.get() {
            navigate("/login", Default::default());
            return;
        }

        if let Some(groups) = &allowed_groups_for_effect
            && !has_group(auth.user.get().as_ref(), groups)
        {
            show_snackbar("Acesso negado para o seu grupo de usuario.", "warning");
            navigate("/home", Default::default());
        }
    });

    let is_authorized = move || {
        if auth.loading.get() || !auth.is_authenticated.get() {
            return false;
        }

        allowed_groups_for_view
            .as_ref()
            .map(|groups| has_group(auth.user.get().as_ref(), groups))
            .unwrap_or(true)
    };

    view! {
        <Show when=is_authorized fallback=move || view! { <div/> }>
            {children()}
        </Show>
    }
}
