use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::hooks::use_navigate;
use wasm_bindgen::JsCast;

use crate::{
    components::common::{PageLayout, Pagination},
    services::caixa_service::{self, CaixaComandaResumo},
    utils::snackbar::show_snackbar,
};

const MAX_API_LIMIT: usize = 1000;

fn format_currency(value: f64) -> String {
    format!("R$ {:.2}", value)
}

fn parse_text_list(value: &str) -> Vec<String> {
    value
        .split(|char| matches!(char, ',' | ';' | '\n' | '\r' | '\t' | ' '))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn display_cliente(comanda: &CaixaComandaResumo) -> String {
    if !comanda.cliente_nome.trim().is_empty() {
        return comanda.cliente_nome.clone();
    }

    if let Some(detalhe) = &comanda.detalhe {
        if !detalhe.cliente_nome.trim().is_empty() {
            return detalhe.cliente_nome.clone();
        }
    }

    comanda
        .cliente_id
        .map(|id| id.to_string())
        .or_else(|| comanda.detalhe.as_ref().and_then(|detalhe| detalhe.cliente_id).map(|id| id.to_string()))
        .unwrap_or_else(|| "-".to_string())
}

fn display_total(comanda: &CaixaComandaResumo) -> f64 {
    if comanda.valor_total > 0.0 {
        return comanda.valor_total;
    }

    comanda
        .detalhe
        .as_ref()
        .map(|detalhe| detalhe.valor_total)
        .unwrap_or(0.0)
}

fn receive_query(selected_ids: &[u32], manual_comandas: &str) -> String {
    let mut query = Vec::new();

    if !selected_ids.is_empty() {
        let ids = selected_ids
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(",");
        query.push(format!("ids={ids}"));
    }

    let numeros = parse_text_list(manual_comandas);

    if !numeros.is_empty() {
        query.push(format!("numeros={}", numeros.join(",")));
    }

    if query.is_empty() {
        "/caixa/receber".to_string()
    } else {
        format!("/caixa/receber?{}", query.join("&"))
    }
}

#[component]
pub fn CaixaDashboard() -> impl IntoView {
    let navigate = use_navigate();
    let (comandas, set_comandas) = signal(Vec::<CaixaComandaResumo>::new());
    let (valor_total, set_valor_total) = signal(0.0);
    let (quantidade_comandas, set_quantidade_comandas) = signal(0u32);
    let (loading, set_loading) = signal(false);
    let (current_page, set_current_page) = signal(1usize);
    let (items_per_page, set_items_per_page) = signal(6usize);
    let (has_next_page, set_has_next_page) = signal(false);
    let (request_version, set_request_version) = signal(0usize);
    let (selected_ids, set_selected_ids) = signal(Vec::<u32>::new());
    let (manual_comandas, set_manual_comandas) = signal(String::new());

    Effect::new(move |_| {
        let page = current_page.get();
        let per_page = items_per_page.get().max(1);
        let next_request_version = request_version.get_untracked().wrapping_add(1);

        set_request_version.set(next_request_version);
        set_loading.set(true);

        leptos::task::spawn_local(async move {
            let skip = page.saturating_sub(1) * per_page;
            let limit = per_page.saturating_add(1).min(MAX_API_LIMIT);

            match caixa_service::dashboard(skip, limit).await {
                Ok(mut resumo) => {
                    if request_version.get_untracked() != next_request_version {
                        return;
                    }

                    let has_more_items = resumo.comandas.len() > per_page;
                    resumo.comandas.truncate(per_page);

                    if resumo.comandas.is_empty() && page > 1 {
                        set_has_next_page.set(false);
                        set_current_page.set(page - 1);
                        return;
                    }

                    let count = resumo.quantidade_comandas.max(resumo.comandas.len() as u32);
                    let total = if resumo.valor_total > 0.0 {
                        resumo.valor_total
                    } else {
                        resumo.comandas.iter().map(display_total).sum()
                    };

                    set_has_next_page.set(has_more_items);
                    set_quantidade_comandas.set(count);
                    set_valor_total.set(total);
                    set_comandas.set(resumo.comandas);
                }
                Err(error) => {
                    if request_version.get_untracked() != next_request_version {
                        return;
                    }

                    set_has_next_page.set(false);
                    set_quantidade_comandas.set(0);
                    set_valor_total.set(0.0);
                    set_comandas.set(Vec::new());
                    show_snackbar(
                        &format!("Erro ao carregar dashboard do caixa: {}", error.message),
                        "error",
                    );
                }
            }

            set_loading.set(false);
        });
    });

    let on_page_change = std::sync::Arc::new(move |page| {
        set_current_page.set(page);
    });

    let on_items_per_page_change = std::sync::Arc::new(move |value| {
        set_items_per_page.set(value);
        set_current_page.set(1);
    });

    let loading_for_pagination = std::sync::Arc::new(move || loading.get());
    let has_next_page_for_pagination = std::sync::Arc::new(move || has_next_page.get());

    let toggle_selection = move |id: u32, selected: bool| {
        set_selected_ids.update(|ids| {
            if selected {
                if !ids.contains(&id) {
                    ids.push(id);
                }
            } else {
                ids.retain(|current| *current != id);
            }
        });
    };

    let nav_receive = navigate.clone();
    let go_to_recebimento = move |_| {
        let ids = selected_ids.get();
        let manual = manual_comandas.get();

        if ids.is_empty() && parse_text_list(&manual).is_empty() {
            show_snackbar("Selecione ou informe pelo menos uma comanda.", "warning");
            return;
        }

        nav_receive(&receive_query(&ids, &manual), Default::default());
    };

    let nav_history = navigate.clone();
    let go_to_history = move |_| {
        nav_history("/recebimentos", Default::default());
    };

    view! {
        <PageLayout title="Caixa".to_string() max_width="7xl".to_string()>
            <div class="space-y-6">
                <div class="grid gap-4 md:grid-cols-3">
                    <section class="rounded-2xl border border-slate-200 bg-slate-50 p-4 shadow-sm">
                        <p class="text-sm font-semibold uppercase tracking-wide text-slate-500">"Comandas abertas"</p>
                        <p class="mt-2 text-3xl font-semibold text-slate-900">{move || quantidade_comandas.get()}</p>
                    </section>
                    <section class="rounded-2xl border border-slate-200 bg-slate-50 p-4 shadow-sm">
                        <p class="text-sm font-semibold uppercase tracking-wide text-slate-500">"Total em aberto"</p>
                        <p class="mt-2 text-3xl font-semibold text-slate-900">{move || format_currency(valor_total.get())}</p>
                    </section>
                    <section class="rounded-2xl border border-slate-200 bg-slate-50 p-4 shadow-sm">
                        <p class="text-sm font-semibold uppercase tracking-wide text-slate-500">"Selecionadas"</p>
                        <p class="mt-2 text-3xl font-semibold text-slate-900">{move || selected_ids.get().len()}</p>
                    </section>
                </div>

                <section class="rounded-2xl border border-slate-200 bg-white p-4 shadow-sm sm:p-5">
                    <div class="grid gap-4 lg:grid-cols-[minmax(0,1fr)_auto] lg:items-end">
                        <div class="min-w-0 space-y-2">
                            <label class="block text-sm font-semibold text-slate-700">"Buscar por numero da comanda"</label>
                            <input
                                type="text"
                                class="w-full rounded-xl border border-slate-200 px-4 py-3 focus:border-amber-500 focus:outline-none focus:ring-2 focus:ring-amber-200"
                                placeholder="Ex.: 101, 102, 103"
                                prop:value=move || manual_comandas.get()
                                on:input=move |event| set_manual_comandas.set(event_target_value(&event))
                            />
                        </div>
                        <div class="flex flex-col gap-3 sm:flex-row lg:justify-end">
                            <button
                                type="button"
                                class="inline-flex items-center justify-center gap-2 rounded-xl border border-slate-200 bg-white px-5 py-3 font-semibold text-slate-700 transition hover:border-slate-300"
                                on:click=go_to_history
                            >
                                <Icon icon=icondata::FaReceiptSolid width="1em" height="1em" />
                                "Historico"
                            </button>
                            <button
                                type="button"
                                class="inline-flex items-center justify-center gap-2 rounded-xl bg-amber-500 px-5 py-3 font-semibold text-white transition hover:bg-amber-600"
                                on:click=go_to_recebimento
                            >
                                <Icon icon=icondata::FaCashRegisterSolid width="1em" height="1em" />
                                "Receber"
                            </button>
                        </div>
                    </div>
                </section>

                <section class="hidden overflow-hidden rounded-3xl border border-slate-200 bg-white shadow-sm md:block">
                    <table class="min-w-full divide-y divide-slate-200">
                        <thead class="bg-slate-50">
                            <tr>
                                <th class="w-16 px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Sel."</th>
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Comanda"</th>
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Cliente"</th>
                                <th class="px-6 py-4 text-right text-xs font-semibold uppercase tracking-wider text-slate-500">"Valor"</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-slate-200 bg-white">
                            {move || {
                                if loading.get() || comandas.get().is_empty() {
                                    return vec![view! {
                                        <tr>
                                            <td colspan="4" class="px-6 py-8 text-center text-sm text-slate-500">
                                                {move || if loading.get() { "Carregando comandas..." } else { "Nenhuma comanda aberta encontrada." }}
                                            </td>
                                        </tr>
                                    }.into_any()];
                                }

                                comandas.get().into_iter().map(move |comanda| {
                                    let id = comanda.id;
                                    let selected = selected_ids.get().contains(&id);

                                    view! {
                                        <tr>
                                            <td class="px-6 py-4">
                                                <input
                                                    type="checkbox"
                                                    class="h-5 w-5 rounded border-slate-300 text-amber-500 focus:ring-amber-400"
                                                    prop:checked=selected
                                                    on:change=move |event| {
                                                        let checked = event
                                                            .target()
                                                            .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
                                                            .map(|input| input.checked())
                                                            .unwrap_or(false);

                                                        toggle_selection(id, checked);
                                                    }
                                                />
                                            </td>
                                            <td class="px-6 py-4 font-semibold text-slate-700">{comanda.comanda.clone()}</td>
                                            <td class="px-6 py-4 text-slate-700">{display_cliente(&comanda)}</td>
                                            <td class="px-6 py-4 text-right font-semibold text-slate-900">{format_currency(display_total(&comanda))}</td>
                                        </tr>
                                    }.into_any()
                                }).collect::<Vec<_>>()
                            }}
                        </tbody>
                    </table>
                </section>

                <div class="space-y-4 md:hidden">
                    {move || {
                        if loading.get() || comandas.get().is_empty() {
                            return vec![view! {
                                <div class="rounded-2xl border border-slate-200 bg-white p-6 text-center text-sm text-slate-500 shadow-sm">
                                    {move || if loading.get() { "Carregando comandas..." } else { "Nenhuma comanda aberta encontrada." }}
                                </div>
                            }.into_any()];
                        }

                        comandas.get().into_iter().map(move |comanda| {
                            let id = comanda.id;
                            let selected = selected_ids.get().contains(&id);

                            view! {
                                <div class="rounded-2xl border border-slate-200 bg-white p-4 shadow-sm">
                                    <div class="flex items-start justify-between gap-4">
                                        <div class="min-w-0">
                                            <p class="text-sm font-semibold uppercase tracking-wide text-slate-500">"Comanda"</p>
                                            <h2 class="mt-1 break-words text-xl font-semibold text-slate-900">{comanda.comanda.clone()}</h2>
                                        </div>
                                        <input
                                            type="checkbox"
                                            class="mt-1 h-5 w-5 rounded border-slate-300 text-amber-500 focus:ring-amber-400"
                                            prop:checked=selected
                                            on:change=move |event| {
                                                let checked = event
                                                    .target()
                                                    .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
                                                    .map(|input| input.checked())
                                                    .unwrap_or(false);

                                                toggle_selection(id, checked);
                                            }
                                        />
                                    </div>
                                    <div class="mt-4 space-y-2 text-sm text-slate-600">
                                        <p>{"Cliente: "}{display_cliente(&comanda)}</p>
                                        <p class="font-semibold text-slate-900">{"Valor: "}{format_currency(display_total(&comanda))}</p>
                                    </div>
                                </div>
                            }.into_any()
                        }).collect::<Vec<_>>()
                    }}
                </div>

                <Pagination
                    current_page=current_page
                    items_per_page=items_per_page
                    on_page_change=on_page_change
                    on_items_per_page_change=on_items_per_page_change
                    loading=loading_for_pagination
                    has_next_page=has_next_page_for_pagination
                />
            </div>
        </PageLayout>
    }
}
