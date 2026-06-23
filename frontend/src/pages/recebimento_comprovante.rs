use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::hooks::{use_navigate, use_params_map};

use crate::{
    components::common::PageLayout,
    services::caixa_service::{self, CaixaComandaResumo, ComprovantePagamento},
    utils::snackbar::show_snackbar,
};

fn format_currency(value: f64) -> String {
    format!("R$ {:.2}", value)
}

fn display_datetime(value: Option<String>) -> String {
    value
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "-".to_string())
}

fn display_cliente(comprovante: &ComprovantePagamento) -> String {
    if !comprovante.recebimento.cliente_nome.trim().is_empty() {
        return comprovante.recebimento.cliente_nome.clone();
    }

    comprovante
        .recebimento
        .cliente_id
        .map(|value| value.to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn display_funcionario(comprovante: &ComprovantePagamento) -> String {
    if !comprovante.recebimento.funcionario_nome.trim().is_empty() {
        return comprovante.recebimento.funcionario_nome.clone();
    }

    comprovante
        .recebimento
        .funcionario_id
        .map(|value| value.to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn comanda_total(comanda: &CaixaComandaResumo) -> f64 {
    if comanda.valor_total > 0.0 {
        return comanda.valor_total;
    }

    comanda
        .detalhe
        .as_ref()
        .map(|detalhe| detalhe.valor_total)
        .unwrap_or(0.0)
}

fn final_value(comprovante: &ComprovantePagamento) -> f64 {
    if comprovante.recebimento.valor_final > 0.0 {
        comprovante.recebimento.valor_final
    } else {
        comprovante.recebimento.valor_total - comprovante.recebimento.desconto
            + comprovante.recebimento.acrescimo
    }
}

#[component]
pub fn RecebimentoComprovante() -> impl IntoView {
    let params = use_params_map();
    let navigate = use_navigate();
    let route_id = params
        .with_untracked(|params| params.get("id"))
        .and_then(|id| id.parse::<u32>().ok());

    let (comprovante, set_comprovante) = signal(None::<ComprovantePagamento>);
    let (loading, set_loading) = signal(true);

    Effect::new(move |_| {
        let Some(id) = route_id else {
            set_loading.set(false);
            show_snackbar("Recebimento nao informado.", "error");
            return;
        };

        leptos::task::spawn_local(async move {
            set_loading.set(true);

            match caixa_service::comprovante(id).await {
                Ok(data) => set_comprovante.set(Some(data)),
                Err(error) => {
                    set_comprovante.set(None);
                    show_snackbar(
                        &format!("Erro ao carregar comprovante: {}", error.message),
                        "error",
                    );
                }
            }

            set_loading.set(false);
        });
    });

    let nav_caixa = navigate.clone();
    let nav_history = navigate.clone();

    view! {
        <PageLayout title="Comprovante".to_string() max_width="lg".to_string()>
            {move || {
                if loading.get() {
                    return view! {
                        <div class="rounded-2xl border border-slate-200 bg-white p-8 text-center text-sm text-slate-500 shadow-sm">
                            "Carregando comprovante..."
                        </div>
                    }.into_any();
                }

                let Some(data) = comprovante.get() else {
                    return view! {
                        <div class="rounded-2xl border border-slate-200 bg-white p-8 text-center text-sm text-slate-500 shadow-sm">
                            "Comprovante nao encontrado."
                        </div>
                    }.into_any();
                };

                let commandas = data.recebimento.comandas.clone();
                let mensagem = data.mensagem.clone();
                let mensagem_visible = mensagem.clone();
                let mensagem_text = mensagem.clone();
                let nav_caixa_button = nav_caixa.clone();
                let nav_history_button = nav_history.clone();

                view! {
                    <div class="space-y-6">
                        <section class="rounded-2xl border border-slate-200 bg-slate-50 p-5 shadow-sm">
                            <div class="flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
                                <div>
                                    <p class="text-sm font-semibold uppercase tracking-wide text-slate-500">"Recebimento"</p>
                                    <h2 class="mt-1 text-3xl font-semibold text-slate-900">{"#"}{data.recebimento.id}</h2>
                                </div>
                                <p class="text-2xl font-semibold text-emerald-700">{format_currency(final_value(&data))}</p>
                            </div>

                            <div class="mt-5 grid gap-3 text-sm text-slate-600 sm:grid-cols-2">
                                <p><span class="font-semibold text-slate-700">"Data: "</span>{display_datetime(data.recebimento.data_hora.clone())}</p>
                                <p><span class="font-semibold text-slate-700">"Cliente: "</span>{display_cliente(&data)}</p>
                                <p><span class="font-semibold text-slate-700">"Funcionario: "</span>{display_funcionario(&data)}</p>
                                <p><span class="font-semibold text-slate-700">"Comandas: "</span>{commandas.len()}</p>
                            </div>
                        </section>

                        <section class="rounded-2xl border border-slate-200 bg-white p-5 shadow-sm">
                            <h3 class="text-base font-semibold text-slate-900">"Resumo financeiro"</h3>
                            <div class="mt-4 grid gap-3 text-sm text-slate-600 sm:grid-cols-2">
                                <p class="flex justify-between gap-4"><span>"Valor total"</span><strong>{format_currency(data.recebimento.valor_total)}</strong></p>
                                <p class="flex justify-between gap-4"><span>"Desconto"</span><strong>{format_currency(data.recebimento.desconto)}</strong></p>
                                <p class="flex justify-between gap-4"><span>"Acrescimo"</span><strong>{format_currency(data.recebimento.acrescimo)}</strong></p>
                                <p class="flex justify-between gap-4 text-emerald-700"><span>"Valor final"</span><strong>{format_currency(final_value(&data))}</strong></p>
                            </div>
                            <Show when=move || !mensagem_visible.trim().is_empty()>
                                <p class="mt-4 rounded-xl bg-emerald-50 p-3 text-sm font-medium text-emerald-700">
                                    {mensagem_text.clone()}
                                </p>
                            </Show>
                        </section>

                        <section class="overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-sm">
                            <div class="border-b border-slate-200 bg-slate-50 px-5 py-3">
                                <h3 class="text-base font-semibold text-slate-900">"Comandas recebidas"</h3>
                            </div>
                            <div class="divide-y divide-slate-200">
                                {if commandas.is_empty() {
                                    vec![view! {
                                        <div class="px-5 py-8 text-center text-sm text-slate-500">
                                            "Nenhuma comanda vinculada ao comprovante."
                                        </div>
                                    }.into_any()]
                                } else {
                                    commandas.into_iter().map(|comanda| view! {
                                        <div class="grid gap-3 px-5 py-4 text-sm sm:grid-cols-[1fr_auto] sm:items-center">
                                            <div class="min-w-0">
                                                <p class="break-words font-semibold text-slate-900">{"Comanda "}{comanda.comanda.clone()}</p>
                                                <p class="text-slate-500">{"ID: "}{comanda.id}</p>
                                            </div>
                                            <p class="font-semibold text-slate-900 sm:text-right">{format_currency(comanda_total(&comanda))}</p>
                                        </div>
                                    }.into_any()).collect::<Vec<_>>()
                                }}
                            </div>
                        </section>

                        <div class="flex flex-col gap-3 sm:flex-row sm:justify-end">
                            <button
                                type="button"
                                on:click=move |_| nav_history_button("/recebimentos", Default::default())
                                class="inline-flex items-center justify-center gap-2 rounded-xl border border-slate-200 bg-white px-6 py-3 font-semibold text-slate-700 transition hover:border-slate-300"
                            >
                                <Icon icon=icondata::FaReceiptSolid width="1em" height="1em" />
                                "Historico"
                            </button>
                            <button
                                type="button"
                                on:click=move |_| nav_caixa_button("/caixa", Default::default())
                                class="inline-flex items-center justify-center gap-2 rounded-xl bg-amber-500 px-6 py-3 font-semibold text-white transition hover:bg-amber-600"
                            >
                                <Icon icon=icondata::FaCashRegisterSolid width="1em" height="1em" />
                                "Caixa"
                            </button>
                        </div>
                    </div>
                }.into_any()
            }}
        </PageLayout>
    }
}
