use std::sync::Arc;

use leptos::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProdutoFiltersState {
    pub id: Option<u32>,
    pub nome: Option<String>,
    pub descricao: Option<String>,
    pub valor: Option<f64>,
    pub valor_min: Option<f64>,
    pub valor_max: Option<f64>,
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

fn parse_f64(value: &str) -> Option<f64> {
    let value = value.trim().replace(',', ".");

    if value.is_empty() {
        None
    } else {
        value.parse::<f64>().ok()
    }
}

#[component]
pub fn ProdutoFilters(
    on_filter: Arc<dyn Fn(ProdutoFiltersState) + Send + Sync>,
    on_clear: Arc<dyn Fn() + Send + Sync>,
    #[prop(optional)] loading: Option<Arc<dyn Fn() -> bool + Send + Sync>>,
) -> impl IntoView {
    let loading = loading.unwrap_or_else(|| Arc::new(|| false));

    let (expanded, set_expanded) = signal(false);
    let (id, set_id) = signal(String::new());
    let (nome, set_nome) = signal(String::new());
    let (descricao, set_descricao) = signal(String::new());
    let (valor, set_valor) = signal(String::new());
    let (valor_min, set_valor_min) = signal(String::new());
    let (valor_max, set_valor_max) = signal(String::new());

    let has_active_filters = move || {
        !id.get().trim().is_empty()
            || !nome.get().trim().is_empty()
            || !descricao.get().trim().is_empty()
            || !valor.get().trim().is_empty()
            || !valor_min.get().trim().is_empty()
            || !valor_max.get().trim().is_empty()
    };

    let handle_filter = {
        let on_filter = on_filter.clone();

        move |_| {
            on_filter(ProdutoFiltersState {
                id: parse_u32(&id.get()),
                nome: clean_text(nome.get()),
                descricao: clean_text(descricao.get()),
                valor: parse_f64(&valor.get()),
                valor_min: parse_f64(&valor_min.get()),
                valor_max: parse_f64(&valor_max.get()),
            });
        }
    };

    let handle_clear = {
        let on_clear = on_clear.clone();

        move |_| {
            set_id.set(String::new());
            set_nome.set(String::new());
            set_descricao.set(String::new());
            set_valor.set(String::new());
            set_valor_min.set(String::new());
            set_valor_max.set(String::new());
            on_clear();
        }
    };

    let loading_id = loading.clone();
    let loading_nome = loading.clone();
    let loading_descricao = loading.clone();
    let loading_valor = loading.clone();
    let loading_valor_min = loading.clone();
    let loading_valor_max = loading.clone();
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
                        "Opções de filtros"
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
                    <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
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
                            <label class=label_class>"Descrição"</label>
                            <input
                                class=input_class
                                type="text"
                                placeholder="Buscar por descrição..."
                                prop:value=move || descricao.get()
                                disabled=move || loading_descricao()
                                on:input=move |event| set_descricao.set(event_target_value(&event))
                            />
                        </div>

                        <div class="space-y-2">
                            <label class=label_class>"Valor"</label>
                            <input
                                class=input_class
                                type="number"
                                step="0.01"
                                placeholder="0,00"
                                prop:value=move || valor.get()
                                disabled=move || loading_valor()
                                on:input=move |event| set_valor.set(event_target_value(&event))
                            />
                        </div>

                        <div class="space-y-2">
                            <label class=label_class>"Valor Mínimo"</label>
                            <input
                                class=input_class
                                type="number"
                                step="0.01"
                                placeholder="0,00"
                                prop:value=move || valor_min.get()
                                disabled=move || loading_valor_min()
                                on:input=move |event| set_valor_min.set(event_target_value(&event))
                            />
                        </div>

                        <div class="space-y-2">
                            <label class=label_class>"Valor Máximo"</label>
                            <input
                                class=input_class
                                type="number"
                                step="0.01"
                                placeholder="999,99"
                                prop:value=move || valor_max.get()
                                disabled=move || loading_valor_max()
                                on:input=move |event| set_valor_max.set(event_target_value(&event))
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
