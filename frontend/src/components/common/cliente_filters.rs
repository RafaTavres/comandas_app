use std::sync::Arc;

use leptos::prelude::*;

use crate::hooks::masks::{apply_cpf_mask, apply_phone_mask, only_digits};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClienteFiltersState {
    pub id: Option<u32>,
    pub nome: Option<String>,
    pub cpf: Option<String>,
    pub telefone: Option<String>,
}

fn clean_text(value: String) -> Option<String> {
    let value = value.trim().to_string();

    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

fn clean_digits(value: String) -> Option<String> {
    let value = only_digits(&value);

    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

fn parse_u32(value: &str) -> Option<u32> {
    let value = value.trim();

    if value.is_empty() {
        None
    } else {
        value.parse::<u32>().ok()
    }
}

#[component]
pub fn ClienteFilters(
    on_filter: Arc<dyn Fn(ClienteFiltersState) + Send + Sync>,
    on_clear: Arc<dyn Fn() + Send + Sync>,
    #[prop(optional)] loading: Option<Arc<dyn Fn() -> bool + Send + Sync>>,
) -> impl IntoView {
    let loading = loading.unwrap_or_else(|| Arc::new(|| false));

    let (expanded, set_expanded) = signal(false);
    let (id, set_id) = signal(String::new());
    let (nome, set_nome) = signal(String::new());
    let (cpf, set_cpf) = signal(String::new());
    let (telefone, set_telefone) = signal(String::new());

    let has_active_filters = move || {
        !id.get().trim().is_empty()
            || !nome.get().trim().is_empty()
            || !cpf.get().trim().is_empty()
            || !telefone.get().trim().is_empty()
    };

    let handle_filter = {
        let on_filter = on_filter.clone();

        move |_| {
            on_filter(ClienteFiltersState {
                id: parse_u32(&id.get()),
                nome: clean_text(nome.get()),
                cpf: clean_digits(cpf.get()),
                telefone: clean_digits(telefone.get()),
            });
        }
    };

    let handle_clear = {
        let on_clear = on_clear.clone();

        move |_| {
            set_id.set(String::new());
            set_nome.set(String::new());
            set_cpf.set(String::new());
            set_telefone.set(String::new());
            on_clear();
        }
    };

    let loading_id = loading.clone();
    let loading_nome = loading.clone();
    let loading_cpf = loading.clone();
    let loading_telefone = loading.clone();
    let loading_clear = loading.clone();
    let loading_filter = loading.clone();

    let input_class = "w-full rounded-xl border border-slate-200 bg-white px-4 py-3 text-sm text-slate-700 focus:border-amber-500 focus:outline-none focus:ring-2 focus:ring-amber-200 disabled:cursor-not-allowed disabled:opacity-50";
    let label_class = "block text-sm font-semibold text-slate-700";

    view! {
        <section class="overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-sm">
            <button
                type="button"
                class="flex w-full items-center justify-between gap-4 bg-slate-50 px-4 py-4 text-left transition hover:bg-slate-100 sm:px-6"
                on:click=move |_| set_expanded.update(|value| *value = !*value)
                aria-expanded=move || expanded.get().to_string()
            >
                <div class="min-w-0">
                    <p class="text-base font-semibold text-slate-900">"Filtros"</p>
                    <p class="mt-1 text-sm text-slate-500">
                        "Opcoes de filtros"
                        <Show when=has_active_filters>
                            " (ativos)"
                        </Show>
                    </p>
                </div>
                <span class="shrink-0 rounded-lg border border-slate-200 bg-white px-3 py-1 text-sm font-semibold text-slate-600">
                    {move || if expanded.get() { "Recolher" } else { "Expandir" }}
                </span>
            </button>

            <div
                class="border-t border-slate-200 p-4 sm:p-6"
                class=("hidden", move || !expanded.get())
            >
                <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
                    <div class="space-y-2">
                        <label class=label_class>"ID"</label>
                        <input
                            class=input_class
                            type="number"
                            min="1"
                            placeholder="Buscar por ID..."
                            prop:value=move || id.get()
                            disabled=move || loading_id()
                            on:input=move |event| set_id.set(event_target_value(&event))
                        />
                    </div>

                    <div class="space-y-2">
                        <label class=label_class>"Nome"</label>
                        <input
                            class=input_class
                            type="text"
                            placeholder="Buscar por nome..."
                            prop:value=move || nome.get()
                            disabled=move || loading_nome()
                            on:input=move |event| set_nome.set(event_target_value(&event))
                        />
                    </div>

                    <div class="space-y-2">
                        <label class=label_class>"CPF"</label>
                        <input
                            class=input_class
                            type="text"
                            placeholder="Buscar por CPF..."
                            prop:value=move || cpf.get()
                            disabled=move || loading_cpf()
                            on:input=move |event| set_cpf.set(apply_cpf_mask(&event_target_value(&event)))
                        />
                    </div>

                    <div class="space-y-2">
                        <label class=label_class>"Telefone"</label>
                        <input
                            class=input_class
                            type="text"
                            placeholder="Buscar por telefone..."
                            prop:value=move || telefone.get()
                            disabled=move || loading_telefone()
                            on:input=move |event| set_telefone.set(apply_phone_mask(&event_target_value(&event)))
                        />
                    </div>
                </div>

                <div class="mt-5 flex flex-col gap-3 sm:flex-row sm:justify-end">
                    <button
                        type="button"
                        class="rounded-xl border border-slate-200 bg-white px-5 py-2.5 text-sm font-semibold text-slate-700 transition hover:border-slate-300 hover:bg-slate-50 disabled:cursor-not-allowed disabled:opacity-50"
                        disabled=move || loading_clear() || !has_active_filters()
                        on:click=handle_clear
                    >
                        "Limpar"
                    </button>
                    <button
                        type="button"
                        class="rounded-xl bg-amber-500 px-5 py-2.5 text-sm font-semibold text-white transition hover:bg-amber-600 disabled:cursor-not-allowed disabled:opacity-50"
                        disabled=move || loading_filter()
                        on:click=handle_filter
                    >
                        "Filtrar"
                    </button>
                </div>
            </div>
        </section>
    }
}
