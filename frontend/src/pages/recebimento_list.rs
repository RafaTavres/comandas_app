use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::hooks::use_navigate;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CustomEvent};

use crate::{
    components::common::{ActionButtons, PageLayout, Pagination},
    context::auth::use_auth,
    services::caixa_service::{self, Recebimento, RecebimentoListParams},
    utils::{
        snackbar::{show_confirm_snackbar, show_snackbar},
        user_groups::is_admin,
    },
};

const MAX_API_LIMIT: usize = 1000;
const DELETE_RECEBIMENTO_ACTION: &str = "delete-recebimento";

#[derive(Debug, Clone, Default, PartialEq)]
struct RecebimentoFilters {
    id: Option<u32>,
    funcionario_id: Option<u32>,
    cliente_id: Option<u32>,
    data_inicio: Option<String>,
    data_fim: Option<String>,
}

fn list_params(page: usize, limit: usize, filters: RecebimentoFilters) -> RecebimentoListParams {
    let limit = limit.max(1);

    RecebimentoListParams {
        skip: page.saturating_sub(1) * limit,
        limit: limit.saturating_add(1).min(MAX_API_LIMIT),
        id: filters.id,
        funcionario_id: filters.funcionario_id,
        cliente_id: filters.cliente_id,
        data_inicio: filters.data_inicio,
        data_fim: filters.data_fim,
    }
}

fn parse_optional_u32(value: &str) -> Option<u32> {
    value.trim().parse::<u32>().ok()
}

fn optional_text(value: &str) -> Option<String> {
    let value = value.trim();

    (!value.is_empty()).then(|| value.to_string())
}

fn format_currency(value: f64) -> String {
    format!("R$ {:.2}", value)
}

fn display_datetime(value: Option<String>) -> String {
    value
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "-".to_string())
}

fn display_cliente(recebimento: &Recebimento) -> String {
    if !recebimento.cliente_nome.trim().is_empty() {
        return recebimento.cliente_nome.clone();
    }

    recebimento
        .cliente_id
        .map(|value| value.to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn display_funcionario(recebimento: &Recebimento) -> String {
    if !recebimento.funcionario_nome.trim().is_empty() {
        return recebimento.funcionario_nome.clone();
    }

    recebimento
        .funcionario_id
        .map(|value| value.to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn final_value(recebimento: &Recebimento) -> f64 {
    if recebimento.valor_final > 0.0 {
        recebimento.valor_final
    } else {
        recebimento.valor_total - recebimento.desconto + recebimento.acrescimo
    }
}

#[component]
pub fn RecebimentoList() -> impl IntoView {
    let navigate = use_navigate();
    let auth = use_auth();
    let user = auth.user;
    let (recebimentos, set_recebimentos) = signal(Vec::<Recebimento>::new());
    let (loading, set_loading) = signal(false);
    let (filters, set_filters) = signal(RecebimentoFilters::default());
    let (id_filter, set_id_filter) = signal(String::new());
    let (funcionario_filter, set_funcionario_filter) = signal(String::new());
    let (cliente_filter, set_cliente_filter) = signal(String::new());
    let (data_inicio_filter, set_data_inicio_filter) = signal(String::new());
    let (data_fim_filter, set_data_fim_filter) = signal(String::new());
    let (current_page, set_current_page) = signal(1usize);
    let (items_per_page, set_items_per_page) = signal(6usize);
    let (has_next_page, set_has_next_page) = signal(false);
    let (refresh_version, set_refresh_version) = signal(0usize);
    let (request_version, set_request_version) = signal(0usize);
    let (pending_delete, set_pending_delete) = signal(None::<Recebimento>);

    Effect::new(move |_| {
        let page = current_page.get();
        let per_page = items_per_page.get().max(1);
        let params = list_params(page, per_page, filters.get());
        let _ = refresh_version.get();
        let next_request_version = request_version.get_untracked().wrapping_add(1);

        set_request_version.set(next_request_version);
        set_loading.set(true);

        leptos::task::spawn_local(async move {
            match caixa_service::list_recebimentos(params).await {
                Ok(mut items) => {
                    if request_version.get_untracked() != next_request_version {
                        return;
                    }

                    let has_more_items = items.len() > per_page;
                    items.truncate(per_page);

                    if items.is_empty() && page > 1 {
                        set_has_next_page.set(false);
                        set_current_page.set(page - 1);
                        return;
                    }

                    set_has_next_page.set(has_more_items);
                    set_recebimentos.set(items);
                }
                Err(error) => {
                    if request_version.get_untracked() != next_request_version {
                        return;
                    }

                    set_has_next_page.set(false);
                    set_recebimentos.set(Vec::new());
                    show_snackbar(
                        &format!("Erro ao carregar recebimentos: {}", error.message),
                        "error",
                    );
                }
            }

            set_loading.set(false);
        });
    });

    Effect::new(move |_| {
        let Some(win) = window() else {
            return;
        };

        let listener = Closure::wrap(Box::new(move |event: web_sys::Event| {
            let Some(custom) = event.dyn_ref::<CustomEvent>() else {
                return;
            };

            if custom.detail().as_string().as_deref() != Some(DELETE_RECEBIMENTO_ACTION) {
                return;
            }

            let Some(recebimento) = pending_delete.get_untracked() else {
                return;
            };

            set_pending_delete.set(None);

            leptos::task::spawn_local(async move {
                match caixa_service::delete_recebimento(recebimento.id).await {
                    Ok(()) => {
                        show_snackbar("Recebimento excluido com sucesso!", "success");

                        if recebimentos.get_untracked().len() <= 1
                            && current_page.get_untracked() > 1
                        {
                            set_current_page.update(|page| {
                                *page = (*page).saturating_sub(1).max(1);
                            });
                        } else {
                            set_refresh_version.update(|version| {
                                *version = version.wrapping_add(1);
                            });
                        }
                    }
                    Err(error) => {
                        show_snackbar(
                            &format!("Erro ao excluir recebimento: {}", error.message),
                            "error",
                        );
                    }
                }
            });
        }) as Box<dyn FnMut(_)>);

        let _ = win.add_event_listener_with_callback(
            "snackbarConfirmed",
            listener.as_ref().unchecked_ref(),
        );

        listener.forget();
    });

    let apply_filters = move |_| {
        set_filters.set(RecebimentoFilters {
            id: parse_optional_u32(&id_filter.get()),
            funcionario_id: parse_optional_u32(&funcionario_filter.get()),
            cliente_id: parse_optional_u32(&cliente_filter.get()),
            data_inicio: optional_text(&data_inicio_filter.get()),
            data_fim: optional_text(&data_fim_filter.get()),
        });
        set_current_page.set(1);
    };

    let clear_filters = move |_| {
        set_id_filter.set(String::new());
        set_funcionario_filter.set(String::new());
        set_cliente_filter.set(String::new());
        set_data_inicio_filter.set(String::new());
        set_data_fim_filter.set(String::new());
        set_filters.set(RecebimentoFilters::default());
        set_current_page.set(1);
    };

    let on_page_change = std::sync::Arc::new(move |page| {
        set_current_page.set(page);
    });

    let on_items_per_page_change = std::sync::Arc::new(move |value| {
        set_items_per_page.set(value);
        set_current_page.set(1);
    });

    let nav_view = navigate.clone();
    let on_view = std::sync::Arc::new(move |recebimento: Recebimento| {
        nav_view(
            &format!("/recebimento/view/{}", recebimento.id),
            Default::default(),
        );
    });

    let nav_edit = navigate.clone();
    let on_edit = std::sync::Arc::new(move |recebimento: Recebimento| {
        nav_edit(
            &format!("/recebimento/edit/{}", recebimento.id),
            Default::default(),
        );
    });

    let on_delete = std::sync::Arc::new(move |recebimento: Recebimento| {
        set_pending_delete.set(Some(recebimento.clone()));
        show_confirm_snackbar(
            &format!("Tem certeza que deseja excluir o recebimento #{}?", recebimento.id),
            "warning",
            "Excluir",
            "Cancelar",
            DELETE_RECEBIMENTO_ACTION,
        );
    });

    let nav_receipt = navigate.clone();
    let go_to_receipt = std::sync::Arc::new(move |recebimento: Recebimento| {
        nav_receipt(
            &format!("/recebimento/{}/comprovante", recebimento.id),
            Default::default(),
        );
    });

    let nav_caixa = navigate.clone();
    let go_to_caixa = move |_| {
        nav_caixa("/caixa", Default::default());
    };

    let loading_for_pagination = std::sync::Arc::new(move || loading.get());
    let has_next_page_for_pagination = std::sync::Arc::new(move || has_next_page.get());

    let on_view_table = on_view.clone();
    let on_edit_table = on_edit.clone();
    let on_delete_table = on_delete.clone();
    let go_to_receipt_table = go_to_receipt.clone();
    let on_view_mobile = on_view.clone();
    let on_edit_mobile = on_edit.clone();
    let on_delete_mobile = on_delete.clone();
    let go_to_receipt_mobile = go_to_receipt.clone();

    view! {
        <PageLayout title="Recebimentos".to_string() max_width="7xl".to_string()>
            <div class="space-y-6">
                <section class="rounded-2xl border border-slate-200 bg-white p-4 shadow-sm sm:p-5">
                    <div class="grid gap-4 md:grid-cols-3 xl:grid-cols-5">
                        <input
                            type="number"
                            class="rounded-xl border border-slate-200 px-4 py-3 focus:border-amber-500 focus:outline-none focus:ring-2 focus:ring-amber-200"
                            placeholder="ID"
                            prop:value=move || id_filter.get()
                            on:input=move |event| set_id_filter.set(event_target_value(&event))
                        />
                        <input
                            type="number"
                            class="rounded-xl border border-slate-200 px-4 py-3 focus:border-amber-500 focus:outline-none focus:ring-2 focus:ring-amber-200"
                            placeholder="Funcionario"
                            prop:value=move || funcionario_filter.get()
                            on:input=move |event| set_funcionario_filter.set(event_target_value(&event))
                        />
                        <input
                            type="number"
                            class="rounded-xl border border-slate-200 px-4 py-3 focus:border-amber-500 focus:outline-none focus:ring-2 focus:ring-amber-200"
                            placeholder="Cliente"
                            prop:value=move || cliente_filter.get()
                            on:input=move |event| set_cliente_filter.set(event_target_value(&event))
                        />
                        <input
                            type="date"
                            class="rounded-xl border border-slate-200 px-4 py-3 focus:border-amber-500 focus:outline-none focus:ring-2 focus:ring-amber-200"
                            prop:value=move || data_inicio_filter.get()
                            on:input=move |event| set_data_inicio_filter.set(event_target_value(&event))
                        />
                        <input
                            type="date"
                            class="rounded-xl border border-slate-200 px-4 py-3 focus:border-amber-500 focus:outline-none focus:ring-2 focus:ring-amber-200"
                            prop:value=move || data_fim_filter.get()
                            on:input=move |event| set_data_fim_filter.set(event_target_value(&event))
                        />
                    </div>
                    <div class="mt-4 flex flex-col gap-3 sm:flex-row sm:justify-end">
                        <button
                            type="button"
                            class="rounded-xl border border-slate-200 bg-white px-5 py-2.5 text-sm font-semibold text-slate-700 transition hover:border-slate-300"
                            on:click=clear_filters
                        >
                            "Limpar"
                        </button>
                        <button
                            type="button"
                            class="rounded-xl bg-amber-500 px-5 py-2.5 text-sm font-semibold text-white transition hover:bg-amber-600"
                            on:click=apply_filters
                        >
                            "Filtrar"
                        </button>
                        <button
                            type="button"
                            class="rounded-xl bg-slate-900 px-5 py-2.5 text-sm font-semibold text-white transition hover:bg-slate-800"
                            on:click=go_to_caixa
                        >
                            "Voltar ao Caixa"
                        </button>
                    </div>
                </section>

                <div class="hidden overflow-hidden rounded-3xl border border-slate-200 bg-white shadow-sm md:block">
                    <table class="min-w-full divide-y divide-slate-200">
                        <thead class="bg-slate-50">
                            <tr>
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"ID"</th>
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Data"</th>
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Cliente"</th>
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Funcionario"</th>
                                <th class="px-6 py-4 text-right text-xs font-semibold uppercase tracking-wider text-slate-500">"Valor Final"</th>
                                <th class="px-6 py-4 text-right text-xs font-semibold uppercase tracking-wider text-slate-500">"Acoes"</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-slate-200 bg-white">
                            {move || {
                                if loading.get() || recebimentos.get().is_empty() {
                                    return vec![view! {
                                        <tr>
                                            <td colspan="6" class="px-6 py-8 text-center text-sm text-slate-500">
                                                {move || if loading.get() { "Carregando recebimentos..." } else { "Nenhum recebimento encontrado." }}
                                            </td>
                                        </tr>
                                    }.into_any()];
                                }

                                let can_manage = is_admin(user.get().as_ref());
                                let on_view = on_view_table.clone();
                                let on_edit = on_edit_table.clone();
                                let on_delete = on_delete_table.clone();
                                let go_receipt = go_to_receipt_table.clone();

                                recebimentos.get().into_iter().map(move |recebimento| {
                                    let receipt_item = recebimento.clone();
                                    let go_receipt_button = go_receipt.clone();

                                    view! {
                                        <tr>
                                            <td class="px-6 py-4 text-slate-700">{recebimento.id}</td>
                                            <td class="px-6 py-4 text-slate-700">{display_datetime(recebimento.data_hora.clone())}</td>
                                            <td class="px-6 py-4 text-slate-700">{display_cliente(&recebimento)}</td>
                                            <td class="px-6 py-4 text-slate-700">{display_funcionario(&recebimento)}</td>
                                            <td class="px-6 py-4 text-right font-semibold text-slate-900">{format_currency(final_value(&recebimento))}</td>
                                            <td class="px-6 py-4 text-right">
                                                <ActionButtons
                                                    item=recebimento
                                                    on_view=on_view.clone()
                                                    on_edit=on_edit.clone()
                                                    on_delete=on_delete.clone()
                                                    show_edit=can_manage
                                                    show_delete=can_manage
                                                >
                                                    <button
                                                        type="button"
                                                        class="inline-flex h-10 w-10 items-center justify-center rounded-lg bg-slate-100 text-emerald-700 transition hover:bg-emerald-100"
                                                        title="Comprovante"
                                                        on:click=move |_| go_receipt_button(receipt_item.clone())
                                                    >
                                                        <Icon icon=icondata::FaReceiptSolid width="1em" height="1em" />
                                                    </button>
                                                </ActionButtons>
                                            </td>
                                        </tr>
                                    }.into_any()
                                }).collect::<Vec<_>>()
                            }}
                        </tbody>
                    </table>
                </div>

                <div class="space-y-4 md:hidden">
                    {move || {
                        if loading.get() || recebimentos.get().is_empty() {
                            return vec![view! {
                                <div class="rounded-2xl border border-slate-200 bg-white p-6 text-center text-sm text-slate-500 shadow-sm">
                                    {move || if loading.get() { "Carregando recebimentos..." } else { "Nenhum recebimento encontrado." }}
                                </div>
                            }.into_any()];
                        }

                        let can_manage = is_admin(user.get().as_ref());
                        let on_view = on_view_mobile.clone();
                        let on_edit = on_edit_mobile.clone();
                        let on_delete = on_delete_mobile.clone();
                        let go_receipt = go_to_receipt_mobile.clone();

                        recebimentos.get().into_iter().map(move |recebimento| {
                            let receipt_item = recebimento.clone();
                            let go_receipt_button = go_receipt.clone();

                            view! {
                                <div class="rounded-2xl border border-slate-200 bg-white p-4 shadow-sm">
                                    <div class="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
                                        <div class="min-w-0">
                                            <p class="text-sm text-slate-500">{"Recebimento #"}{recebimento.id}</p>
                                            <p class="mt-1 break-words text-lg font-semibold text-slate-900">{display_cliente(&recebimento)}</p>
                                        </div>
                                        <p class="text-base font-semibold text-slate-900">{format_currency(final_value(&recebimento))}</p>
                                    </div>

                                    <div class="mt-4 space-y-2 text-sm text-slate-600">
                                        <p>{"Data: "}{display_datetime(recebimento.data_hora.clone())}</p>
                                        <p>{"Funcionario: "}{display_funcionario(&recebimento)}</p>
                                    </div>

                                    <div class="mt-4 flex justify-end">
                                        <ActionButtons
                                            item=recebimento
                                            on_view=on_view.clone()
                                            on_edit=on_edit.clone()
                                            on_delete=on_delete.clone()
                                            show_edit=can_manage
                                            show_delete=can_manage
                                        >
                                            <button
                                                type="button"
                                                class="inline-flex h-10 w-10 items-center justify-center rounded-lg bg-slate-100 text-emerald-700 transition hover:bg-emerald-100"
                                                title="Comprovante"
                                                on:click=move |_| go_receipt_button(receipt_item.clone())
                                            >
                                                <Icon icon=icondata::FaReceiptSolid width="1em" height="1em" />
                                            </button>
                                        </ActionButtons>
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
