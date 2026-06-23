use std::collections::HashMap;

use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::hooks::use_navigate;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CustomEvent};

use crate::{
    components::common::{
        ActionButtons, ComandaFilters, ComandaFiltersState, PageLayout, Pagination,
    },
    constants::comanda_status::{status_config, ComandaStatus},
    context::auth::use_auth,
    services::comanda_service::{self, Comanda, ComandaItemListParams, ComandaListParams},
    utils::{
        snackbar::{show_confirm_snackbar, show_snackbar},
        user_groups::is_admin,
    },
};

const MAX_API_LIMIT: usize = 1000;
const CANCEL_COMANDA_ACTION: &str = "cancel-comanda";
const DELETE_COMANDA_ACTION: &str = "delete-comanda";

fn list_params(page: usize, limit: usize, filters: ComandaFiltersState) -> ComandaListParams {
    let limit = limit.max(1);

    ComandaListParams {
        skip: page.saturating_sub(1) * limit,
        limit: limit.saturating_add(1).min(MAX_API_LIMIT),
        id: filters.id,
        comanda: filters.comanda,
        status: filters.status,
        funcionario_id: filters.funcionario_id,
        cliente_id: filters.cliente_id,
        data_inicio: filters.data_inicio,
        data_fim: filters.data_fim,
    }
}

fn display_datetime(value: Option<String>) -> String {
    value
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "-".to_string())
}

fn display_optional_id(value: Option<u32>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn display_cliente(comanda: &Comanda) -> String {
    if !comanda.cliente_nome.trim().is_empty() {
        return comanda.cliente_nome.clone();
    }

    display_optional_id(comanda.cliente_id)
}

fn format_currency(value: f64) -> String {
    format!("R$ {:.2}", value)
}

fn display_total(comanda: &Comanda, totals: &HashMap<u32, f64>) -> String {
    if let Some(total) = totals.get(&comanda.id) {
        return format_currency(*total);
    }

    comanda
        .total
        .map(format_currency)
        .unwrap_or_else(|| "-".to_string())
}

#[component]
pub fn ComandaList() -> impl IntoView {
    let navigate = use_navigate();
    let auth = use_auth();
    let user = auth.user;
    let (comandas, set_comandas) = signal(Vec::<Comanda>::new());
    let (loading, set_loading) = signal(false);
    let (filters, set_filters) = signal(ComandaFiltersState::default());
    let (current_page, set_current_page) = signal(1usize);
    let (items_per_page, set_items_per_page) = signal(3usize);
    let (has_next_page, set_has_next_page) = signal(false);
    let (refresh_version, set_refresh_version) = signal(0usize);
    let (request_version, set_request_version) = signal(0usize);
    let (comanda_totals, set_comanda_totals) = signal(HashMap::<u32, f64>::new());
    let (pending_cancel, set_pending_cancel) = signal(None::<Comanda>);
    let (pending_delete, set_pending_delete) = signal(None::<Comanda>);

    Effect::new(move |_| {
        let page = current_page.get();
        let per_page = items_per_page.get().max(1);
        let params = list_params(page, per_page, filters.get());
        let _ = refresh_version.get();
        let next_request_version = request_version.get_untracked().wrapping_add(1);

        set_request_version.set(next_request_version);
        set_loading.set(true);

        leptos::task::spawn_local(async move {
            match comanda_service::list(params).await {
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

                    let mut totals = HashMap::new();

                    for comanda in &items {
                        if let Ok(consumos) = comanda_service::list_items(
                            comanda.id,
                            ComandaItemListParams {
                                skip: 0,
                                limit: MAX_API_LIMIT,
                            },
                        )
                        .await
                        {
                            totals.insert(comanda.id, comanda_service::items_total(&consumos));
                        } else if let Some(total) = comanda.total {
                            totals.insert(comanda.id, total);
                        }
                    }

                    if request_version.get_untracked() != next_request_version {
                        return;
                    }

                    set_has_next_page.set(has_more_items);
                    set_comanda_totals.set(totals);
                    set_comandas.set(items);
                }
                Err(error) => {
                    if request_version.get_untracked() != next_request_version {
                        return;
                    }

                    set_has_next_page.set(false);
                    set_comanda_totals.set(HashMap::new());
                    set_comandas.set(Vec::new());
                    show_snackbar(
                        &format!("Erro ao carregar comandas: {}", error.message),
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
        let listener_active = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
        let listener_active_for_callback = listener_active.clone();

        let listener = Closure::wrap(Box::new(move |event: web_sys::Event| {
            if !listener_active_for_callback.load(std::sync::atomic::Ordering::Relaxed) {
                return;
            }

            let Some(custom) = event.dyn_ref::<CustomEvent>() else {
                return;
            };

            let Some(action) = custom.detail().as_string() else {
                return;
            };

            match action.as_str() {
                CANCEL_COMANDA_ACTION => {
                    let Some(comanda) = pending_cancel.get_untracked() else {
                        return;
                    };

                    set_pending_cancel.set(None);

                    leptos::task::spawn_local(async move {
                        match comanda_service::cancel(comanda.id).await {
                            Ok(()) => {
                                show_snackbar("Comanda cancelada com sucesso!", "success");
                                set_refresh_version
                                    .update(|version| *version = version.wrapping_add(1));
                            }
                            Err(error) => {
                                show_snackbar(
                                    &format!("Erro ao cancelar comanda: {}", error.message),
                                    "error",
                                );
                            }
                        }
                    });
                }
                DELETE_COMANDA_ACTION => {
                    let Some(comanda) = pending_delete.get_untracked() else {
                        return;
                    };

                    set_pending_delete.set(None);

                    leptos::task::spawn_local(async move {
                        match comanda_service::delete(comanda.id).await {
                            Ok(()) => {
                                show_snackbar("Comanda excluida com sucesso!", "success");

                                if comandas.get_untracked().len() <= 1
                                    && current_page.get_untracked() > 1
                                {
                                    set_current_page.update(|page| {
                                        *page = (*page).saturating_sub(1).max(1);
                                    });
                                } else {
                                    set_refresh_version
                                        .update(|version| *version = version.wrapping_add(1));
                                }
                            }
                            Err(error) => {
                                show_snackbar(
                                    &format!("Erro ao excluir comanda: {}", error.message),
                                    "error",
                                );
                            }
                        }
                    });
                }
                _ => {}
            }
        }) as Box<dyn FnMut(_)>);

        let _ = win.add_event_listener_with_callback(
            "snackbarConfirmed",
            listener.as_ref().unchecked_ref(),
        );

        on_cleanup(move || {
            listener_active.store(false, std::sync::atomic::Ordering::Relaxed);
        });
        listener.forget();
    });

    let on_filter = std::sync::Arc::new(move |next_filters| {
        set_filters.set(next_filters);
        set_current_page.set(1);
    });

    let on_clear_filters = std::sync::Arc::new(move || {
        set_filters.set(ComandaFiltersState::default());
        set_current_page.set(1);
    });

    let on_page_change = std::sync::Arc::new(move |page| {
        set_current_page.set(page);
    });

    let on_items_per_page_change = std::sync::Arc::new(move |value| {
        set_items_per_page.set(value);
        set_current_page.set(1);
    });

    let nav_view = navigate.clone();
    let on_view = std::sync::Arc::new(move |comanda: Comanda| {
        nav_view(&format!("/comanda/view/{}", comanda.id), Default::default());
    });

    let nav_edit = navigate.clone();
    let on_edit = std::sync::Arc::new(move |comanda: Comanda| {
        nav_edit(&format!("/comanda/edit/{}", comanda.id), Default::default());
    });

    let on_delete = std::sync::Arc::new(move |comanda: Comanda| {
        set_pending_delete.set(Some(comanda.clone()));
        show_confirm_snackbar(
            &format!(
                "Tem certeza que deseja excluir a comanda \"{}\"?",
                comanda.comanda
            ),
            "warning",
            "Excluir",
            "Cancelar",
            DELETE_COMANDA_ACTION,
        );
    });

    let nav_consumo = navigate.clone();
    let on_consumo = std::sync::Arc::new(move |comanda: Comanda| {
        nav_consumo(
            &format!("/comanda/consumo/{}", comanda.id),
            Default::default(),
        );
    });

    let on_cancel = std::sync::Arc::new(move |comanda: Comanda| {
        set_pending_cancel.set(Some(comanda.clone()));
        show_confirm_snackbar(
            &format!(
                "Tem certeza que deseja cancelar a comanda \"{}\"?",
                comanda.comanda
            ),
            "warning",
            "Cancelar Comanda",
            "Voltar",
            CANCEL_COMANDA_ACTION,
        );
    });

    let on_view_table = on_view.clone();
    let on_edit_table = on_edit.clone();
    let on_delete_table = on_delete.clone();
    let on_consumo_table = on_consumo.clone();
    let on_cancel_table = on_cancel.clone();
    let on_view_mobile = on_view.clone();
    let on_edit_mobile = on_edit.clone();
    let on_delete_mobile = on_delete.clone();
    let on_consumo_mobile = on_consumo.clone();
    let on_cancel_mobile = on_cancel.clone();

    let loading_for_filters = std::sync::Arc::new(move || loading.get());
    let loading_for_pagination = loading_for_filters.clone();
    let has_next_page_for_pagination = std::sync::Arc::new(move || has_next_page.get());

    view! {
        <PageLayout title="Comandas".to_string() max_width="7xl".to_string()>
            <div class="mb-6 flex">
                <div class="flex w-full justify-stretch sm:justify-end">
                    <a
                        href="/comanda"
                        class="w-full rounded-xl bg-amber-500 px-6 py-3 text-center font-semibold text-white transition hover:bg-amber-600 sm:w-auto"
                    >
                        "+ Abrir Comanda"
                    </a>
                </div>
            </div>

            <div class="space-y-6">
                <ComandaFilters
                    on_filter=on_filter
                    on_clear=on_clear_filters
                    loading=loading_for_filters
                />

                <div class="hidden overflow-hidden rounded-3xl border border-slate-200 bg-white shadow-sm md:block">
                    <table class="min-w-full divide-y divide-slate-200">
                        <thead class="bg-slate-50">
                            <tr>
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"ID"</th>
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Comanda"</th>
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Abertura"</th>
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Cliente"</th>
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Total"</th>
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Status"</th>
                                <th class="px-6 py-4 text-right text-xs font-semibold uppercase tracking-wider text-slate-500">"Acoes"</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-slate-200 bg-white">
                            {move || {
                                if loading.get() || comandas.get().is_empty() {
                                    return vec![view! {
                                        <tr>
                                            <td colspan="7" class="px-6 py-8 text-center text-sm text-slate-500">
                                                {move || if loading.get() { "Carregando comandas..." } else { "Nenhuma comanda encontrada." }}
                                            </td>
                                        </tr>
                                    }.into_any()];
                                }

                                let on_view = on_view_table.clone();
                                let on_edit = on_edit_table.clone();
                                let on_delete = on_delete_table.clone();
                                let on_consumo = on_consumo_table.clone();
                                let on_cancel = on_cancel_table.clone();
                                let totals = comanda_totals.get();
                                let can_manage = is_admin(user.get().as_ref());

                                comandas.get().into_iter().map(move |comanda| {
                                    let status = status_config(comanda.status);
                                    let cliente = display_cliente(&comanda);
                                    let total = display_total(&comanda, &totals);
                                    let is_open = comanda.status == ComandaStatus::Aberta as i32;
                                    let consumo_item = comanda.clone();
                                    let cancel_item = comanda.clone();
                                    let on_consumo_button = on_consumo.clone();
                                    let on_cancel_button = on_cancel.clone();

                                    view! {
                                        <tr>
                                            <td class="px-6 py-4 text-slate-700">{comanda.id}</td>
                                            <td class="px-6 py-4 font-semibold text-slate-700">{comanda.comanda.clone()}</td>
                                            <td class="px-6 py-4 text-slate-700">{display_datetime(comanda.data_hora.clone())}</td>
                                            <td class="px-6 py-4 text-slate-700">{cliente}</td>
                                            <td class="px-6 py-4 text-slate-700">{total}</td>
                                            <td class="px-6 py-4">
                                                <span class=format!("inline-flex rounded-full border px-2 py-1 text-xs font-semibold {}", status.badge_class)>
                                                    {status.label}
                                                </span>
                                            </td>
                                            <td class="px-6 py-4 text-right">
                                                <ActionButtons
                                                    item=comanda
                                                    on_view=on_view.clone()
                                                    on_edit=on_edit.clone()
                                                    on_delete=on_delete.clone()
                                                    show_edit=can_manage
                                                    show_delete=can_manage
                                                    edit_disabled=!is_open
                                                    delete_disabled=!is_open
                                                >
                                                    <button
                                                        type="button"
                                                        class="inline-flex h-10 w-10 items-center justify-center rounded-lg bg-slate-100 text-emerald-700 transition hover:bg-emerald-100 disabled:cursor-not-allowed disabled:opacity-50"
                                                        title="Adicionar consumo"
                                                        disabled=!is_open
                                                        on:click=move |_| {
                                                            if is_open {
                                                                on_consumo_button(consumo_item.clone());
                                                            }
                                                        }
                                                    >
                                                        <Icon icon=icondata::FaPlusSolid width="1em" height="1em" />
                                                    </button>
                                                    {can_manage.then(|| view! {
                                                        <button
                                                            type="button"
                                                            class="inline-flex h-10 w-10 items-center justify-center rounded-lg bg-slate-100 text-orange-700 transition hover:bg-orange-100 disabled:cursor-not-allowed disabled:opacity-50"
                                                            title="Cancelar comanda"
                                                            disabled=!is_open
                                                            on:click=move |_| {
                                                                if is_open {
                                                                    on_cancel_button(cancel_item.clone());
                                                                }
                                                            }
                                                        >
                                                            <Icon icon=icondata::FaBanSolid width="1em" height="1em" />
                                                        </button>
                                                    })}
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
                        if loading.get() || comandas.get().is_empty() {
                            return vec![view! {
                                <div class="rounded-2xl border border-slate-200 bg-white p-6 text-center text-sm text-slate-500 shadow-sm">
                                    {move || if loading.get() { "Carregando comandas..." } else { "Nenhuma comanda encontrada." }}
                                </div>
                            }.into_any()];
                        }

                        let on_view = on_view_mobile.clone();
                        let on_edit = on_edit_mobile.clone();
                        let on_delete = on_delete_mobile.clone();
                        let on_consumo = on_consumo_mobile.clone();
                        let on_cancel = on_cancel_mobile.clone();
                        let totals = comanda_totals.get();
                        let can_manage = is_admin(user.get().as_ref());

                        comandas.get().into_iter().map(move |comanda| {
                            let status = status_config(comanda.status);
                            let cliente = display_cliente(&comanda);
                            let total = display_total(&comanda, &totals);
                            let is_open = comanda.status == ComandaStatus::Aberta as i32;
                            let consumo_item = comanda.clone();
                            let cancel_item = comanda.clone();
                            let on_consumo_button = on_consumo.clone();
                            let on_cancel_button = on_cancel.clone();

                            view! {
                                <div class="rounded-2xl border border-slate-200 bg-white p-4 shadow-sm">
                                    <div class="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
                                        <div class="min-w-0">
                                            <p class="break-words text-lg font-semibold text-slate-900">
                                                {"Comanda "}{comanda.comanda.clone()}
                                            </p>
                                            <p class="text-sm text-slate-500">{"ID: "}{comanda.id}</p>
                                        </div>
                                        <span class=format!("inline-flex w-fit rounded-full border px-2 py-1 text-xs font-semibold {}", status.badge_class)>
                                            {status.label}
                                        </span>
                                    </div>

                                    <div class="mt-4 space-y-2 text-sm text-slate-600">
                                        <p>{"Abertura: "}{display_datetime(comanda.data_hora.clone())}</p>
                                        <p>{"Cliente: "}{cliente}</p>
                                        <p class="font-semibold text-slate-800">{"Total: "}{total}</p>
                                    </div>

                                    <div class="mt-4 flex justify-end">
                                        <ActionButtons
                                            item=comanda
                                            on_view=on_view.clone()
                                            on_edit=on_edit.clone()
                                            on_delete=on_delete.clone()
                                            show_edit=can_manage
                                            show_delete=can_manage
                                            edit_disabled=!is_open
                                            delete_disabled=!is_open
                                        >
                                            <button
                                                type="button"
                                                class="inline-flex h-10 w-10 items-center justify-center rounded-lg bg-slate-100 text-emerald-700 transition hover:bg-emerald-100 disabled:cursor-not-allowed disabled:opacity-50"
                                                title="Adicionar consumo"
                                                disabled=!is_open
                                                on:click=move |_| {
                                                    if is_open {
                                                        on_consumo_button(consumo_item.clone());
                                                    }
                                                }
                                            >
                                                <Icon icon=icondata::FaPlusSolid width="1em" height="1em" />
                                            </button>
                                            {can_manage.then(|| view! {
                                                <button
                                                    type="button"
                                                    class="inline-flex h-10 w-10 items-center justify-center rounded-lg bg-slate-100 text-orange-700 transition hover:bg-orange-100 disabled:cursor-not-allowed disabled:opacity-50"
                                                    title="Cancelar comanda"
                                                    disabled=!is_open
                                                    on:click=move |_| {
                                                        if is_open {
                                                            on_cancel_button(cancel_item.clone());
                                                        }
                                                    }
                                                >
                                                    <Icon icon=icondata::FaBanSolid width="1em" height="1em" />
                                                </button>
                                            })}
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
