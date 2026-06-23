use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CustomEvent};

use crate::{
    components::common::{ActionButtons, ClienteFilters, ClienteFiltersState, PageLayout, Pagination},
    context::auth::use_auth,
    hooks::masks::{apply_cpf_mask, apply_phone_mask},
    services::cliente_service::{self, Cliente, ClienteListParams},
    utils::{
        snackbar::{show_confirm_snackbar, show_snackbar},
        user_groups::{is_admin, is_admin_or_caixa},
    },
};

const MAX_API_LIMIT: usize = 1000;
const DELETE_CLIENT_ACTION: &str = "delete-client";

fn list_params(page: usize, limit: usize, filters: ClienteFiltersState) -> ClienteListParams {
    let limit = limit.max(1);

    ClienteListParams {
        skip: page.saturating_sub(1) * limit,
        limit: limit.saturating_add(1).min(MAX_API_LIMIT),
        id: filters.id,
        nome: filters.nome,
        cpf: filters.cpf,
        telefone: filters.telefone,
    }
}

#[component]
pub fn ClienteList() -> impl IntoView {
    let navigate = use_navigate();
    let auth = use_auth();
    let user = auth.user;
    let (clientes, set_clientes) = signal(Vec::<Cliente>::new());
    let (loading, set_loading) = signal(false);
    let (filters, set_filters) = signal(ClienteFiltersState::default());
    let (current_page, set_current_page) = signal(1usize);
    let (items_per_page, set_items_per_page) = signal(3usize);
    let (has_next_page, set_has_next_page) = signal(false);
    let (refresh_version, set_refresh_version) = signal(0usize);
    let (request_version, set_request_version) = signal(0usize);
    let (pending_delete, set_pending_delete) = signal(None::<Cliente>);

    Effect::new(move |_| {
        let page = current_page.get();
        let per_page = items_per_page.get().max(1);
        let params = list_params(page, per_page, filters.get());
        let _ = refresh_version.get();
        let next_request_version = request_version.get_untracked().wrapping_add(1);

        set_request_version.set(next_request_version);
        set_loading.set(true);

        leptos::task::spawn_local(async move {
            match cliente_service::list(params).await {
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
                    set_clientes.set(items);
                }
                Err(error) => {
                    if request_version.get_untracked() != next_request_version {
                        return;
                    }

                    set_has_next_page.set(false);
                    set_clientes.set(Vec::new());
                    show_snackbar(&format!("Erro ao carregar clientes: {}", error.message), "error");
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

            if custom.detail().as_string().as_deref() != Some(DELETE_CLIENT_ACTION) {
                return;
            }

            let Some(cliente) = pending_delete.get_untracked() else {
                return;
            };

            set_pending_delete.set(None);

            leptos::task::spawn_local(async move {
                match cliente_service::delete(cliente.id).await {
                    Ok(()) => {
                        show_snackbar("Cliente excluido com sucesso!", "success");

                        if clientes.get_untracked().len() <= 1 && current_page.get_untracked() > 1 {
                            set_current_page.update(|page| {
                                *page = (*page).saturating_sub(1).max(1);
                            });
                        } else {
                            set_refresh_version.update(|version| *version = version.wrapping_add(1));
                        }
                    }
                    Err(error) => {
                        show_snackbar(&format!("Erro ao excluir cliente: {}", error.message), "error");
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

    let on_filter = std::sync::Arc::new(move |next_filters| {
        set_filters.set(next_filters);
        set_current_page.set(1);
    });

    let on_clear_filters = std::sync::Arc::new(move || {
        set_filters.set(ClienteFiltersState::default());
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
    let on_view = std::sync::Arc::new(move |cliente: Cliente| {
        nav_view(&format!("/cliente/view/{}", cliente.id), Default::default());
    });

    let nav_edit = navigate.clone();
    let on_edit = std::sync::Arc::new(move |cliente: Cliente| {
        nav_edit(&format!("/cliente/edit/{}", cliente.id), Default::default());
    });

    let on_delete = std::sync::Arc::new(move |cliente: Cliente| {
        set_pending_delete.set(Some(cliente.clone()));
        show_confirm_snackbar(
            &format!("Tem certeza que deseja excluir o cliente \"{}\"?", cliente.nome),
            "warning",
            "Excluir",
            "Cancelar",
            DELETE_CLIENT_ACTION,
        );
    });

    let on_view_table = on_view.clone();
    let on_edit_table = on_edit.clone();
    let on_delete_table = on_delete.clone();
    let on_view_mobile = on_view.clone();
    let on_edit_mobile = on_edit.clone();
    let on_delete_mobile = on_delete.clone();

    let loading_for_filters = std::sync::Arc::new(move || loading.get());
    let loading_for_pagination = loading_for_filters.clone();
    let has_next_page_for_pagination = std::sync::Arc::new(move || has_next_page.get());

    view! {
        <PageLayout title="Clientes".to_string() max_width="7xl".to_string()>
            <Show when=move || is_admin_or_caixa(user.get().as_ref())>
                <div class="mb-6 flex">
                    <div class="flex w-full justify-stretch sm:justify-end">
                        <a
                            href="/cliente"
                            class="w-full rounded-xl bg-amber-500 px-6 py-3 text-center font-semibold text-white hover:bg-amber-600 transition sm:w-auto"
                        >
                            "+ Novo Cliente"
                        </a>
                    </div>
                </div>
            </Show>

            <div class="space-y-6">
                <ClienteFilters
                    on_filter=on_filter
                    on_clear=on_clear_filters
                    loading=loading_for_filters
                />

                <div class="hidden md:block overflow-hidden rounded-3xl border border-slate-200 bg-white shadow-sm">
                    <table class="min-w-full divide-y divide-slate-200">
                        <thead class="bg-slate-50">
                            <tr>
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"ID"</th>
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Nome"</th>
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"CPF"</th>
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Telefone"</th>
                                <th class="px-6 py-4 text-right text-xs font-semibold uppercase tracking-wider text-slate-500">"Acoes"</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-slate-200 bg-white">
                            {move || {
                                if loading.get() || clientes.get().is_empty() {
                                    return vec![view! {
                                        <tr>
                                            <td colspan="5" class="px-6 py-8 text-center text-sm text-slate-500">
                                                {move || if loading.get() { "Carregando clientes..." } else { "Nenhum cliente encontrado." }}
                                            </td>
                                        </tr>
                                    }.into_any()];
                                }

                                let on_view = on_view_table.clone();
                                let on_edit = on_edit_table.clone();
                                let on_delete = on_delete_table.clone();
                                let can_edit = is_admin_or_caixa(user.get().as_ref());
                                let can_delete = is_admin(user.get().as_ref());

                                clientes.get().into_iter().map(move |cliente| {
                                    let cpf = apply_cpf_mask(&cliente.cpf);
                                    let telefone = apply_phone_mask(&cliente.telefone);

                                    view! {
                                        <tr>
                                            <td class="px-6 py-4 text-slate-700">{cliente.id}</td>
                                            <td class="px-6 py-4 text-slate-700 font-semibold">{cliente.nome.clone()}</td>
                                            <td class="px-6 py-4 text-slate-700">{cpf}</td>
                                            <td class="px-6 py-4 text-slate-700">{telefone}</td>
                                            <td class="px-6 py-4 text-right">
                                                <ActionButtons
                                                    item=cliente
                                                    on_view=on_view.clone()
                                                    on_edit=on_edit.clone()
                                                    on_delete=on_delete.clone()
                                                    show_edit=can_edit
                                                    show_delete=can_delete
                                                />
                                            </td>
                                        </tr>
                                    }.into_any()
                                }).collect::<Vec<_>>()
                            }}
                        </tbody>
                    </table>
                </div>

                <div class="md:hidden space-y-4">
                    {move || {
                        if loading.get() || clientes.get().is_empty() {
                            return vec![view! {
                                <div class="rounded-2xl border border-slate-200 bg-white p-6 text-center text-sm text-slate-500 shadow-sm">
                                    {move || if loading.get() { "Carregando clientes..." } else { "Nenhum cliente encontrado." }}
                                </div>
                            }.into_any()];
                        }

                        let on_view = on_view_mobile.clone();
                        let on_edit = on_edit_mobile.clone();
                        let on_delete = on_delete_mobile.clone();
                        let can_edit = is_admin_or_caixa(user.get().as_ref());
                        let can_delete = is_admin(user.get().as_ref());

                        clientes.get().into_iter().map(move |cliente| {
                            let cpf = apply_cpf_mask(&cliente.cpf);
                            let telefone = apply_phone_mask(&cliente.telefone);

                            view! {
                                <div class="rounded-2xl border border-slate-200 bg-white p-4 shadow-sm">
                                    <div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between sm:gap-4">
                                        <div class="min-w-0">
                                            <p class="break-words text-lg font-semibold text-slate-900">{cliente.nome.clone()}</p>
                                            <p class="text-sm text-slate-500">{"ID: "}{cliente.id}</p>
                                        </div>
                                        <div class="min-w-0 sm:text-right">
                                            <p class="break-words text-sm font-semibold text-success-600">{cpf}</p>
                                        </div>
                                    </div>
                                    <div class="mt-4 space-y-3">
                                        <p class="text-sm text-slate-600">{telefone}</p>
                                        <div class="flex justify-end">
                                            <ActionButtons
                                                item=cliente
                                                on_view=on_view.clone()
                                                on_edit=on_edit.clone()
                                                on_delete=on_delete.clone()
                                                show_edit=can_edit
                                                show_delete=can_delete
                                            />
                                        </div>
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
