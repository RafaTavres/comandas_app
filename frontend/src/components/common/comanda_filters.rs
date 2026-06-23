use std::sync::Arc;

use leptos::prelude::*;

use crate::constants::comanda_status::STATUS_OPTIONS;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ComandaFiltersState {
    pub id: Option<u32>,
    pub comanda: Option<String>,
    pub status: Option<i32>,
    pub funcionario_id: Option<String>,
    pub cliente_id: Option<String>,
    pub data_inicio: Option<String>,
    pub data_fim: Option<String>,
}

fn clean_text(value: String) -> Option<String> {
    let value = value.trim().to_string();

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

fn parse_i32(value: &str) -> Option<i32> {
    let value = value.trim();

    if value.is_empty() {
        None
    } else {
        value.parse::<i32>().ok()
    }
}

#[component]
pub fn ComandaFilters(
    on_filter: Arc<dyn Fn(ComandaFiltersState) + Send + Sync>,
    on_clear: Arc<dyn Fn() + Send + Sync>,
    #[prop(optional)] loading: Option<Arc<dyn Fn() -> bool + Send + Sync>>,
) -> impl IntoView {
    let loading = loading.unwrap_or_else(|| Arc::new(|| false));

    let (expanded, set_expanded) = signal(false);
    let (id, set_id) = signal(String::new());
    let (comanda, set_comanda) = signal(String::new());
    let (status, set_status) = signal(String::new());
    let (funcionario_id, set_funcionario_id) = signal(String::new());
    let (cliente_id, set_cliente_id) = signal(String::new());
    let (data_inicio, set_data_inicio) = signal(String::new());
    let (data_fim, set_data_fim) = signal(String::new());

    let has_active_filters = move || {
        !id.get().trim().is_empty()
            || !comanda.get().trim().is_empty()
            || !status.get().trim().is_empty()
            || !funcionario_id.get().trim().is_empty()
            || !cliente_id.get().trim().is_empty()
            || !data_inicio.get().trim().is_empty()
            || !data_fim.get().trim().is_empty()
    };

    let handle_filter = {
        let on_filter = on_filter.clone();

        move |_| {
            on_filter(ComandaFiltersState {
                id: parse_u32(&id.get()),
                comanda: clean_text(comanda.get()),
                status: parse_i32(&status.get()),
                funcionario_id: clean_text(funcionario_id.get()),
                cliente_id: clean_text(cliente_id.get()),
                data_inicio: clean_text(data_inicio.get()),
                data_fim: clean_text(data_fim.get()),
            });
        }
    };

    let handle_clear = {
        let on_clear = on_clear.clone();

        move |_| {
            set_id.set(String::new());
            set_comanda.set(String::new());
            set_status.set(String::new());
            set_funcionario_id.set(String::new());
            set_cliente_id.set(String::new());
            set_data_inicio.set(String::new());
            set_data_fim.set(String::new());
            on_clear();
        }
    };

    let loading_id = loading.clone();
    let loading_comanda = loading.clone();
    let loading_status = loading.clone();
    let loading_funcionario = loading.clone();
    let loading_cliente = loading.clone();
    let loading_data_inicio = loading.clone();
    let loading_data_fim = loading.clone();
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
                        <label class=label_class>"Comanda"</label>
                        <input
                            class=input_class
                            type="text"
                            placeholder="Buscar por comanda..."
                            prop:value=move || comanda.get()
                            disabled=move || loading_comanda()
                            on:input=move |event| set_comanda.set(event_target_value(&event))
                        />
                    </div>

                    <div class="space-y-2">
                        <label class=label_class>"Status"</label>
                        <select
                            class=input_class
                            prop:value=move || status.get()
                            disabled=move || loading_status()
                            on:change=move |event| set_status.set(event_target_value(&event))
                        >
                            <option value="">"Todos"</option>
                            {STATUS_OPTIONS.iter().map(|option| view! {
                                <option value=option.value.to_string()>{option.label}</option>
                            }).collect::<Vec<_>>()}
                        </select>
                    </div>

                    <div class="space-y-2">
                        <label class=label_class>"Funcionario"</label>
                        <input
                            class=input_class
                            type="text"
                            placeholder="Buscar por funcionario..."
                            prop:value=move || funcionario_id.get()
                            disabled=move || loading_funcionario()
                            on:input=move |event| set_funcionario_id.set(event_target_value(&event))
                        />
                    </div>

                    <div class="space-y-2">
                        <label class=label_class>"Cliente"</label>
                        <input
                            class=input_class
                            type="text"
                            placeholder="Buscar por cliente..."
                            prop:value=move || cliente_id.get()
                            disabled=move || loading_cliente()
                            on:input=move |event| set_cliente_id.set(event_target_value(&event))
                        />
                    </div>

                    <div class="space-y-2">
                        <label class=label_class>"Data Inicial"</label>
                        <input
                            class=input_class
                            type="date"
                            prop:value=move || data_inicio.get()
                            disabled=move || loading_data_inicio()
                            on:input=move |event| set_data_inicio.set(event_target_value(&event))
                        />
                    </div>

                    <div class="space-y-2">
                        <label class=label_class>"Data Final"</label>
                        <input
                            class=input_class
                            type="date"
                            prop:value=move || data_fim.get()
                            disabled=move || loading_data_fim()
                            on:input=move |event| set_data_fim.set(event_target_value(&event))
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
