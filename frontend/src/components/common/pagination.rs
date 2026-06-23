use std::sync::Arc;

use leptos::prelude::*;
use leptos_icons::Icon;

const MIN_ITEMS_PER_PAGE: usize = 1;
const MAX_ITEMS_PER_PAGE: usize = 1000;

#[component]
pub fn Pagination(
    current_page: ReadSignal<usize>,
    items_per_page: ReadSignal<usize>,
    on_page_change: Arc<dyn Fn(usize) + Send + Sync>,
    on_items_per_page_change: Arc<dyn Fn(usize) + Send + Sync>,
    #[prop(optional)] loading: Option<Arc<dyn Fn() -> bool + Send + Sync>>,
    #[prop(optional)] has_next_page: Option<Arc<dyn Fn() -> bool + Send + Sync>>,
) -> impl IntoView {
    let loading = loading.unwrap_or_else(|| Arc::new(|| false));
    let has_next_page = has_next_page.unwrap_or_else(|| Arc::new(|| true));

    let handle_previous = {
        let on_page_change = on_page_change.clone();
        let loading = loading.clone();

        move |_| {
            if current_page.get() > 1 && !loading() {
                on_page_change(current_page.get() - 1);
            }
        }
    };

    let handle_next = {
        let on_page_change = on_page_change.clone();
        let loading = loading.clone();
        let has_next_page = has_next_page.clone();

        move |_| {
            if !loading() && has_next_page() {
                on_page_change(current_page.get() + 1);
            }
        }
    };

    let handle_items_per_page_change = {
        let on_items_per_page_change = on_items_per_page_change.clone();

        move |event| {
            let value = event_target_value(&event);

            if let Ok(parsed) = value.parse::<usize>() {
                let value = parsed.clamp(MIN_ITEMS_PER_PAGE, MAX_ITEMS_PER_PAGE);
                on_items_per_page_change(value);
            }
        }
    };

    let previous_disabled = {
        let loading = loading.clone();
        move || current_page.get() == 1 || loading()
    };

    let next_disabled = {
        let loading = loading.clone();
        let has_next_page = has_next_page.clone();
        move || loading() || !has_next_page()
    };

    view! {
        <div class="flex flex-col gap-4 rounded-xl border border-slate-200 bg-slate-50 p-3 sm:flex-row sm:flex-wrap sm:items-center sm:justify-between">
            <div class="flex items-center justify-center gap-2">
                <button
                    type="button"
                    class="inline-flex h-9 items-center gap-2 rounded-lg border border-slate-200 bg-white px-3 text-sm font-semibold text-slate-700 transition hover:border-slate-300 hover:bg-slate-100 disabled:cursor-not-allowed disabled:opacity-50"
                    disabled=previous_disabled
                    on:click=handle_previous
                    title="Página anterior"
                    aria-label="Página anterior"
                >
                    <Icon icon=icondata::FaAngleLeftSolid width="1em" height="1em" />
                    "Anterior"
                </button>

                <span class="min-w-20 text-center text-sm font-semibold text-slate-700">
                    "Página " {move || current_page.get()}
                </span>

                <button
                    type="button"
                    class="inline-flex h-9 items-center gap-2 rounded-lg border border-slate-200 bg-white px-3 text-sm font-semibold text-slate-700 transition hover:border-slate-300 hover:bg-slate-100 disabled:cursor-not-allowed disabled:opacity-50"
                    disabled=next_disabled
                    on:click=handle_next
                    title="Próxima página"
                    aria-label="Próxima página"
                >
                    "Próxima"
                    <Icon icon=icondata::FaAngleRightSolid width="1em" height="1em" />
                </button>
            </div>

            <label class="flex items-center justify-center gap-2 text-sm text-slate-600">
                <span class="font-medium">"Itens por página"</span>
                <input
                    type="number"
                    min=MIN_ITEMS_PER_PAGE
                    max=MAX_ITEMS_PER_PAGE
                    class="h-9 w-24 rounded-lg border border-slate-200 bg-white px-3 text-center text-sm font-semibold text-slate-700 focus:border-amber-500 focus:outline-none focus:ring-2 focus:ring-amber-200 disabled:cursor-not-allowed disabled:opacity-50"
                    prop:value=move || items_per_page.get().to_string()
                    disabled=move || loading()
                    on:input=handle_items_per_page_change
                />
            </label>
        </div>
    }
}
