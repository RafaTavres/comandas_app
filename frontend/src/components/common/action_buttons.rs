use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn ActionButtons<T>(
    item: T,
    on_view: std::sync::Arc<dyn Fn(T) + Send + Sync>,
    on_edit: std::sync::Arc<dyn Fn(T) + Send + Sync>,
    on_delete: std::sync::Arc<dyn Fn(T) + Send + Sync>,
) -> impl IntoView
where
    T: Clone + 'static,
{
    let view_item = item.clone();
    let edit_item = item.clone();
    let delete_item = item;

    view! {
        <div class="flex items-center gap-2">
            <button
                type="button"
                class="inline-flex items-center justify-center w-10 h-10 rounded-lg bg-slate-100 text-slate-700 hover:bg-slate-200 transition"
                title="Visualizar"
                on:click=move |_| (on_view)(view_item.clone())
            >
                <Icon icon=icondata::FaEyeSolid width="1.1em" height="1.1em" />
            </button>

            <button
                type="button"
                class="inline-flex items-center justify-center w-10 h-10 rounded-lg bg-slate-100 text-amber-600 hover:bg-amber-100 transition"
                title="Editar"
                on:click=move |_| (on_edit)(edit_item.clone())
            >
                <Icon icon=icondata::FaPencilSolid width="1.1em" height="1.1em" />
            </button>

            <button
                type="button"
                class="inline-flex items-center justify-center w-10 h-10 rounded-lg bg-slate-100 text-red-700 hover:bg-red-100 hover:text-red-700 transition"
                title="Excluir"
                on:click=move |_| (on_delete)(delete_item.clone())
            >
                <Icon icon=icondata::FaTrashSolid width="1.1em" height="1.1em" />
            </button>
        </div>
    }
}
