use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn ActionButtons<T>(
    item: T,
    on_view: std::sync::Arc<dyn Fn(T) + Send + Sync>,
    on_edit: std::sync::Arc<dyn Fn(T) + Send + Sync>,
    on_delete: std::sync::Arc<dyn Fn(T) + Send + Sync>,
    #[prop(optional, default = true)] show_view: bool,
    #[prop(optional, default = true)] show_edit: bool,
    #[prop(optional, default = true)] show_delete: bool,
    #[prop(optional, default = false)] disabled: bool,
    #[prop(optional, default = false)] view_disabled: bool,
    #[prop(optional, default = false)] edit_disabled: bool,
    #[prop(optional, default = false)] delete_disabled: bool,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView
where
    T: Clone + 'static,
{
    let view_item = item.clone();
    let edit_item = item.clone();
    let delete_item = item;
    let extra_actions = children.map(|children| children());
    let view_button = show_view.then(|| {
        let on_view = on_view.clone();

        view! {
            <button
                type="button"
                class="inline-flex items-center justify-center w-10 h-10 rounded-lg bg-slate-100 text-slate-700 hover:bg-slate-200 transition disabled:cursor-not-allowed disabled:opacity-50"
                title="Visualizar"
                disabled=move || disabled || view_disabled
                on:click=move |_| {
                    if !(disabled || view_disabled) {
                        (on_view)(view_item.clone());
                    }
                }
            >
                <Icon icon=icondata::FaEyeSolid width="1.1em" height="1.1em" />
            </button>
        }
    });
    let edit_button = show_edit.then(|| {
        let on_edit = on_edit.clone();

        view! {
            <button
                type="button"
                class="inline-flex items-center justify-center w-10 h-10 rounded-lg bg-slate-100 text-amber-600 hover:bg-amber-100 transition disabled:cursor-not-allowed disabled:opacity-50"
                title="Editar"
                disabled=move || disabled || edit_disabled
                on:click=move |_| {
                    if !(disabled || edit_disabled) {
                        (on_edit)(edit_item.clone());
                    }
                }
            >
                <Icon icon=icondata::FaPencilSolid width="1.1em" height="1.1em" />
            </button>
        }
    });
    let delete_button = show_delete.then(|| {
        let on_delete = on_delete.clone();

        view! {
            <button
                type="button"
                class="inline-flex items-center justify-center w-10 h-10 rounded-lg bg-slate-100 text-red-700 hover:bg-red-100 hover:text-red-700 transition disabled:cursor-not-allowed disabled:opacity-50"
                title="Excluir"
                disabled=move || disabled || delete_disabled
                on:click=move |_| {
                    if !(disabled || delete_disabled) {
                        (on_delete)(delete_item.clone());
                    }
                }
            >
                <Icon icon=icondata::FaTrashSolid width="1.1em" height="1.1em" />
            </button>
        }
    });

    view! {
        <div class="flex items-center gap-2">
            {view_button}
            {edit_button}
            {delete_button}
            {extra_actions}
        </div>
    }
}
