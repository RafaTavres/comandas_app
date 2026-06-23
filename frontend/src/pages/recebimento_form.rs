use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params_map};
use web_sys::window;

use crate::{
    components::common::PageLayout,
    context::auth::use_auth,
    services::caixa_service::{
        self, CaixaComandaResumo, CaixaResumoResponse, CaixaSelecaoComandasRequest, Recebimento,
        RecebimentoCreatePayload, RecebimentoUpdatePayload,
    },
    utils::{
        snackbar::show_snackbar,
        user_groups::{is_admin, is_admin_or_caixa},
    },
};

fn query_value(key: &str) -> Option<String> {
    let search = window()?.location().search().ok()?;
    let search = search.trim_start_matches('?');

    search.split('&').find_map(|part| {
        let (current_key, value) = part.split_once('=')?;

        (current_key == key).then(|| {
            value
                .replace("%2C", ",")
                .replace("%20", " ")
                .replace('+', " ")
        })
    })
}

fn parse_u32_list(value: &str) -> Vec<u32> {
    value
        .split(|char| matches!(char, ',' | ';' | '\n' | '\r' | '\t' | ' '))
        .filter_map(|item| item.trim().parse::<u32>().ok())
        .collect()
}

fn parse_string_list(value: &str) -> Vec<String> {
    value
        .split(|char| matches!(char, ',' | ';' | '\n' | '\r' | '\t' | ' '))
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn parse_decimal(value: &str) -> Option<f64> {
    value.trim().replace(',', ".").parse::<f64>().ok()
}

fn optional_u32(value: &str) -> Option<u32> {
    value.trim().parse::<u32>().ok()
}

fn optional_text(value: &str) -> Option<String> {
    let value = value.trim();

    (!value.is_empty()).then(|| value.to_string())
}

fn format_currency(value: f64) -> String {
    format!("R$ {:.2}", value)
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
        .map(|value| value.to_string())
        .or_else(|| {
            comanda
                .detalhe
                .as_ref()
                .and_then(|detalhe| detalhe.cliente_id)
                .map(|value| value.to_string())
        })
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

fn display_datetime(value: Option<String>) -> String {
    value
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "-".to_string())
}

fn final_value(total: f64, desconto: f64, acrescimo: f64) -> f64 {
    (total - desconto + acrescimo).max(0.0)
}

fn selection_payload(ids_text: &str, numeros_text: &str) -> CaixaSelecaoComandasRequest {
    let ids = parse_u32_list(ids_text);
    let numeros = parse_string_list(numeros_text);

    CaixaSelecaoComandasRequest {
        comanda_ids: (!ids.is_empty()).then_some(ids),
        numeros_comandas: (!numeros.is_empty()).then_some(numeros),
    }
}

#[component]
pub fn RecebimentoForm() -> impl IntoView {
    let params = use_params_map();
    let navigate = use_navigate();
    let auth = use_auth();

    let route_operation = params.with_untracked(|params| params.get("opr"));
    let route_id = params
        .with_untracked(|params| params.get("id"))
        .and_then(|id| id.parse::<u32>().ok());
    let is_view = route_operation.as_deref() == Some("view");
    let is_edit = route_operation.as_deref() == Some("edit");

    let (comanda_ids_text, set_comanda_ids_text) =
        signal(query_value("ids").unwrap_or_default());
    let (numeros_comandas_text, set_numeros_comandas_text) =
        signal(query_value("numeros").unwrap_or_default());
    let (cliente_id, set_cliente_id) = signal(String::new());
    let (desconto, set_desconto) = signal("0.00".to_string());
    let (acrescimo, set_acrescimo) = signal("0.00".to_string());
    let (observacao, set_observacao) = signal(String::new());
    let (recebimento, set_recebimento) = signal(None::<Recebimento>);
    let (selection_summary, set_selection_summary) = signal(None::<CaixaResumoResponse>);
    let (selection_refresh, set_selection_refresh) = signal(0usize);
    let (loading, set_loading) = signal(false);
    let (loading_data, set_loading_data) = signal(route_id.is_some());
    let (loading_selection, set_loading_selection) = signal(false);

    let nav_permission = navigate.clone();
    let auth_loading = auth.loading;
    let auth_user = auth.user;
    Effect::new(move |_| {
        if auth_loading.get() {
            return;
        }

        let user = auth_user.get();

        if is_edit && !is_admin(user.as_ref()) {
            show_snackbar(
                "Acesso negado: apenas administradores podem editar recebimentos.",
                "warning",
            );
            nav_permission("/recebimentos", Default::default());
            return;
        }

        if route_id.is_none() && !is_admin_or_caixa(user.as_ref()) {
            show_snackbar("Acesso negado para criar recebimentos.", "warning");
            nav_permission("/home", Default::default());
        }
    });

    Effect::new(move |_| {
        let Some(id) = route_id else {
            set_loading_data.set(false);
            return;
        };

        leptos::task::spawn_local(async move {
            set_loading_data.set(true);

            match caixa_service::get_recebimento(id).await {
                Ok(data) => {
                    set_cliente_id.set(
                        data.cliente_id
                            .map(|value| value.to_string())
                            .unwrap_or_default(),
                    );
                    set_desconto.set(format!("{:.2}", data.desconto));
                    set_acrescimo.set(format!("{:.2}", data.acrescimo));
                    set_observacao.set(data.observacao.clone().unwrap_or_default());
                    set_comanda_ids_text.set(
                        data.comandas
                            .iter()
                            .map(|comanda| comanda.id.to_string())
                            .collect::<Vec<_>>()
                            .join(", "),
                    );
                    set_numeros_comandas_text.set(
                        data.comandas
                            .iter()
                            .map(|comanda| comanda.comanda.clone())
                            .collect::<Vec<_>>()
                            .join(", "),
                    );
                    set_recebimento.set(Some(data));
                }
                Err(error) => {
                    show_snackbar(
                        &format!("Erro ao carregar recebimento: {}", error.message),
                        "error",
                    );
                }
            }

            set_loading_data.set(false);
        });
    });

    Effect::new(move |_| {
        let _ = selection_refresh.get();

        if route_id.is_some() {
            return;
        }

        let payload = selection_payload(
            &comanda_ids_text.get_untracked(),
            &numeros_comandas_text.get_untracked(),
        );

        if payload.comanda_ids.is_none() && payload.numeros_comandas.is_none() {
            set_selection_summary.set(None);
            return;
        }

        leptos::task::spawn_local(async move {
            set_loading_selection.set(true);

            match caixa_service::selecionar_comandas(&payload).await {
                Ok(summary) => set_selection_summary.set(Some(summary)),
                Err(error) => {
                    set_selection_summary.set(None);
                    show_snackbar(
                        &format!("Erro ao selecionar comandas: {}", error.message),
                        "error",
                    );
                }
            }

            set_loading_selection.set(false);
        });
    });

    let refresh_selection = move |_| {
        set_selection_refresh.update(|version| *version = version.wrapping_add(1));
    };

    let nav_cancel = navigate.clone();
    let on_cancel = move |_| {
        if route_id.is_some() {
            nav_cancel("/recebimentos", Default::default());
        } else {
            nav_cancel("/caixa", Default::default());
        }
    };

    let nav_after_submit = navigate.clone();
    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();

        if is_view {
            return;
        }

        let Some(desconto_value) = parse_decimal(&desconto.get()) else {
            show_snackbar("Desconto invalido", "warning");
            return;
        };

        let Some(acrescimo_value) = parse_decimal(&acrescimo.get()) else {
            show_snackbar("Acrescimo invalido", "warning");
            return;
        };

        if desconto_value < 0.0 || acrescimo_value < 0.0 {
            show_snackbar("Desconto e acrescimo nao podem ser negativos.", "warning");
            return;
        }

        let cliente_value = optional_u32(&cliente_id.get());
        let observacao_value = optional_text(&observacao.get());
        let navigate = nav_after_submit.clone();

        leptos::task::spawn_local(async move {
            set_loading.set(true);

            let result = if let Some(id) = route_id {
                let payload = RecebimentoUpdatePayload {
                    cliente_id: cliente_value,
                    desconto: Some(desconto_value),
                    acrescimo: Some(acrescimo_value),
                    observacao: observacao_value,
                };

                caixa_service::update_recebimento(id, &payload).await
            } else {
                let payload = RecebimentoCreatePayload {
                    comanda_ids: {
                        let ids = parse_u32_list(&comanda_ids_text.get_untracked());
                        (!ids.is_empty()).then_some(ids)
                    },
                    numeros_comandas: {
                        let numeros = parse_string_list(&numeros_comandas_text.get_untracked());
                        (!numeros.is_empty()).then_some(numeros)
                    },
                    cliente_id: cliente_value,
                    desconto: desconto_value,
                    acrescimo: acrescimo_value,
                    observacao: observacao_value,
                };

                if payload.comanda_ids.is_none() && payload.numeros_comandas.is_none() {
                    set_loading.set(false);
                    show_snackbar("Informe pelo menos uma comanda para receber.", "warning");
                    return;
                }

                caixa_service::create_recebimento(&payload).await
            };

            match result {
                Ok(saved) => {
                    show_snackbar("Recebimento salvo com sucesso!", "success");

                    if route_id.is_some() {
                        navigate("/recebimentos", Default::default());
                    } else {
                        navigate(
                            &format!("/recebimento/{}/comprovante", saved.id),
                            Default::default(),
                        );
                    }
                }
                Err(error) => {
                    show_snackbar(
                        &format!("Erro ao salvar recebimento: {}", error.message),
                        "error",
                    );
                }
            }

            set_loading.set(false);
        });
    };

    let field_disabled = move || loading.get() || loading_data.get() || is_view;
    let selection_disabled = move || field_disabled() || route_id.is_some();
    let total_base = move || {
        recebimento
            .get()
            .map(|data| data.valor_total)
            .or_else(|| {
                selection_summary
                    .get()
                    .map(|summary| {
                        if summary.valor_total > 0.0 {
                            summary.valor_total
                        } else {
                            summary.comandas.iter().map(comanda_total).sum()
                        }
                    })
            })
            .unwrap_or(0.0)
    };
    let page_title = move || {
        if is_view {
            "Visualizar Recebimento".to_string()
        } else if is_edit {
            "Editar Recebimento".to_string()
        } else {
            "Receber Comandas".to_string()
        }
    };
    let input_class = "w-full min-w-0 rounded-xl border border-slate-200 px-4 py-3 focus:border-amber-500 focus:outline-none focus:ring-2 focus:ring-amber-200 disabled:cursor-not-allowed disabled:bg-slate-100 disabled:text-slate-500";

    view! {
        <PageLayout title=page_title() max_width="7xl".to_string()>
            <form on:submit=on_submit class="space-y-6">
                <section class="rounded-2xl border border-slate-200 bg-white p-4 shadow-sm sm:p-5">
                    <div class="grid gap-4 lg:grid-cols-2">
                        <div class="min-w-0 space-y-2">
                            <label class="block text-sm font-semibold text-slate-700">"IDs das comandas"</label>
                            <input
                                class=input_class
                                type="text"
                                placeholder="Ex.: 1, 2, 3"
                                prop:value=move || comanda_ids_text.get()
                                disabled=selection_disabled
                                on:input=move |event| set_comanda_ids_text.set(event_target_value(&event))
                            />
                        </div>
                        <div class="min-w-0 space-y-2">
                            <label class="block text-sm font-semibold text-slate-700">"Numeros das comandas"</label>
                            <input
                                class=input_class
                                type="text"
                                placeholder="Ex.: 101, 102"
                                prop:value=move || numeros_comandas_text.get()
                                disabled=selection_disabled
                                on:input=move |event| set_numeros_comandas_text.set(event_target_value(&event))
                            />
                        </div>
                    </div>

                    <Show when=move || route_id.is_none()>
                        <div class="mt-4 flex justify-end">
                            <button
                                type="button"
                                class="rounded-xl border border-slate-200 bg-white px-5 py-2.5 text-sm font-semibold text-slate-700 transition hover:border-slate-300 disabled:cursor-not-allowed disabled:opacity-60"
                                disabled=move || loading_selection.get() || loading.get()
                                on:click=refresh_selection
                            >
                                {move || if loading_selection.get() { "Validando..." } else { "Atualizar resumo" }}
                            </button>
                        </div>
                    </Show>
                </section>

                <section class="rounded-2xl border border-slate-200 bg-slate-50 p-4 shadow-sm sm:p-5">
                    <div class="grid gap-4 md:grid-cols-4">
                        <div>
                            <p class="text-sm font-semibold uppercase tracking-wide text-slate-500">"Valor total"</p>
                            <p class="mt-2 text-2xl font-semibold text-slate-900">{move || format_currency(total_base())}</p>
                        </div>
                        <div>
                            <p class="text-sm font-semibold uppercase tracking-wide text-slate-500">"Desconto"</p>
                            <p class="mt-2 text-2xl font-semibold text-slate-900">{move || format_currency(parse_decimal(&desconto.get()).unwrap_or(0.0))}</p>
                        </div>
                        <div>
                            <p class="text-sm font-semibold uppercase tracking-wide text-slate-500">"Acrescimo"</p>
                            <p class="mt-2 text-2xl font-semibold text-slate-900">{move || format_currency(parse_decimal(&acrescimo.get()).unwrap_or(0.0))}</p>
                        </div>
                        <div>
                            <p class="text-sm font-semibold uppercase tracking-wide text-slate-500">"Valor final"</p>
                            <p class="mt-2 text-2xl font-semibold text-slate-900">
                                {move || {
                                    format_currency(final_value(
                                        total_base(),
                                        parse_decimal(&desconto.get()).unwrap_or(0.0),
                                        parse_decimal(&acrescimo.get()).unwrap_or(0.0),
                                    ))
                                }}
                            </p>
                        </div>
                    </div>
                </section>

                <section class="rounded-2xl border border-slate-200 bg-white p-4 shadow-sm sm:p-5">
                    <div class="grid gap-4 md:grid-cols-3">
                        <div class="min-w-0 space-y-2">
                            <label class="block text-sm font-semibold text-slate-700">"Cliente"</label>
                            <input
                                class=input_class
                                type="number"
                                placeholder="ID do cliente"
                                prop:value=move || cliente_id.get()
                                disabled=field_disabled
                                on:input=move |event| set_cliente_id.set(event_target_value(&event))
                            />
                        </div>
                        <div class="min-w-0 space-y-2">
                            <label class="block text-sm font-semibold text-slate-700">"Desconto"</label>
                            <input
                                class=input_class
                                type="number"
                                min="0"
                                step="0.01"
                                prop:value=move || desconto.get()
                                disabled=field_disabled
                                on:input=move |event| set_desconto.set(event_target_value(&event))
                            />
                        </div>
                        <div class="min-w-0 space-y-2">
                            <label class="block text-sm font-semibold text-slate-700">"Acrescimo"</label>
                            <input
                                class=input_class
                                type="number"
                                min="0"
                                step="0.01"
                                prop:value=move || acrescimo.get()
                                disabled=field_disabled
                                on:input=move |event| set_acrescimo.set(event_target_value(&event))
                            />
                        </div>
                    </div>

                    <div class="mt-4 space-y-2">
                        <label class="block text-sm font-semibold text-slate-700">"Observacao"</label>
                        <textarea
                            class="min-h-[96px] w-full rounded-xl border border-slate-200 px-4 py-3 focus:border-amber-500 focus:outline-none focus:ring-2 focus:ring-amber-200 disabled:cursor-not-allowed disabled:bg-slate-100 disabled:text-slate-500"
                            placeholder="Observacoes do recebimento"
                            prop:value=move || observacao.get()
                            disabled=field_disabled
                            on:input=move |event| set_observacao.set(event_target_value(&event))
                        />
                    </div>
                </section>

                <section class="overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-sm">
                    <div class="border-b border-slate-200 bg-slate-50 px-4 py-3 sm:px-5">
                        <h2 class="text-base font-semibold text-slate-900">"Comandas selecionadas"</h2>
                    </div>
                    <div class="divide-y divide-slate-200">
                        {move || {
                            if loading_data.get() || loading_selection.get() {
                                return vec![view! {
                                    <div class="px-5 py-8 text-center text-sm text-slate-500">
                                        "Carregando resumo..."
                                    </div>
                                }.into_any()];
                            }

                            let comandas = recebimento
                                .get()
                                .map(|data| data.comandas)
                                .or_else(|| selection_summary.get().map(|summary| summary.comandas))
                                .unwrap_or_default();

                            if comandas.is_empty() {
                                return vec![view! {
                                    <div class="px-5 py-8 text-center text-sm text-slate-500">
                                        "Nenhuma comanda selecionada."
                                    </div>
                                }.into_any()];
                            }

                            comandas.into_iter().map(|comanda| view! {
                                <div class="grid gap-3 px-5 py-4 text-sm md:grid-cols-[1fr_1fr_auto] md:items-center">
                                    <div class="min-w-0">
                                        <p class="break-words font-semibold text-slate-900">{"Comanda "}{comanda.comanda.clone()}</p>
                                        <p class="text-slate-500">{"ID: "}{comanda.id}</p>
                                    </div>
                                    <p class="text-slate-600">{"Cliente: "}{display_cliente(&comanda)}</p>
                                    <p class="font-semibold text-slate-900 md:text-right">{format_currency(comanda_total(&comanda))}</p>
                                </div>
                            }.into_any()).collect::<Vec<_>>()
                        }}
                    </div>
                </section>

                <Show when=move || recebimento.get().is_some()>
                    <section class="rounded-2xl border border-slate-200 bg-slate-50 p-4 text-sm text-slate-600 shadow-sm sm:p-5">
                        {move || recebimento.get().map(|data| view! {
                            <div class="grid gap-3 md:grid-cols-3">
                                <p><span class="font-semibold text-slate-700">"Recebimento: "</span>{data.id}</p>
                                <p><span class="font-semibold text-slate-700">"Data: "</span>{display_datetime(data.data_hora)}</p>
                                <p><span class="font-semibold text-slate-700">"Funcionario: "</span>{if data.funcionario_nome.trim().is_empty() { data.funcionario_id.map(|id| id.to_string()).unwrap_or_else(|| "-".to_string()) } else { data.funcionario_nome }}</p>
                            </div>
                        })}
                    </section>
                </Show>

                <div class="flex flex-col gap-3 sm:flex-row sm:justify-end">
                    <button
                        type="button"
                        on:click=on_cancel
                        class="rounded-xl border border-slate-200 bg-white px-6 py-3 font-semibold text-slate-700 transition hover:border-slate-300"
                    >
                        {move || if is_view { "Voltar" } else { "Cancelar" }}
                    </button>

                    <Show when=move || !is_view>
                        <button
                            type="submit"
                            disabled=move || loading.get() || loading_data.get() || loading_selection.get()
                            class="rounded-xl bg-amber-500 px-6 py-3 font-semibold text-white transition hover:bg-amber-600 disabled:cursor-not-allowed disabled:opacity-60"
                        >
                            {move || if loading.get() { "Salvando..." } else if route_id.is_some() { "Atualizar" } else { "Receber" }}
                        </button>
                    </Show>
                </div>
            </form>
        </PageLayout>
    }
}
