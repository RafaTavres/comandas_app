use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CustomEvent};

use crate::{
    components::common::{
        ActionButtons, FuncionarioFilters, FuncionarioFiltersState, PageLayout, Pagination,
    },
    hooks::masks::{apply_cpf_mask, apply_phone_mask},
    services::funcionario_service::{self, Funcionario, FuncionarioListParams},
    utils::snackbar::{show_confirm_snackbar, show_snackbar},
};

const MAX_API_LIMIT: usize = 1000;
const DELETE_FUNCIONARIO_ACTION: &str = "delete-funcionario";

fn list_params(page: usize, limit: usize, filters: FuncionarioFiltersState) -> FuncionarioListParams {
    let limit = limit.max(1);

    FuncionarioListParams {
        skip: page.saturating_sub(1) * limit,
        limit: limit.saturating_add(1).min(MAX_API_LIMIT),
        id: filters.id,
        nome: filters.nome,
        matricula: filters.matricula,
        cpf: filters.cpf,
        grupo: filters.grupo,
        telefone: filters.telefone,
    }
}

fn grupo_label(grupo: i32) -> &'static str {
    match grupo {
        1 => "Admin",
        2 => "Balcao",
        3 => "Caixa",
        _ => "Outro",
    }
}

#[component]
pub fn FuncionarioList() -> impl IntoView {
    let navigate = use_navigate();
    let (funcionarios, set_funcionarios) = signal(Vec::<Funcionario>::new());
    let (loading, set_loading) = signal(false);
    let (filters, set_filters) = signal(FuncionarioFiltersState::default());
    let (current_page, set_current_page) = signal(1usize);
    let (items_per_page, set_items_per_page) = signal(3usize);
    let (has_next_page, set_has_next_page) = signal(false);
    let (refresh_version, set_refresh_version) = signal(0usize);
    let (request_version, set_request_version) = signal(0usize);
    let (pending_delete, set_pending_delete) = signal(None::<Funcionario>);

    Effect::new(move |_| {
        let page = current_page.get();
        let per_page = items_per_page.get().max(1);
        let params = list_params(page, per_page, filters.get());
        let _ = refresh_version.get();
        let next_request_version = request_version.get_untracked().wrapping_add(1);

        set_request_version.set(next_request_version);
        set_loading.set(true);

        leptos::task::spawn_local(async move {
            match funcionario_service::list(params).await {
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
                    set_funcionarios.set(items);
                }
                Err(error) => {
                    if request_version.get_untracked() != next_request_version {
                        return;
                    }

                    set_has_next_page.set(false);
                    set_funcionarios.set(Vec::new());
                    show_snackbar(
                        &format!("Erro ao carregar funcionarios: {}", error.message),
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

            if custom.detail().as_string().as_deref() != Some(DELETE_FUNCIONARIO_ACTION) {
                return;
            }

            let Some(funcionario) = pending_delete.get_untracked() else {
                return;
            };

            set_pending_delete.set(None);

            leptos::task::spawn_local(async move {
                match funcionario_service::delete(funcionario.id).await {
                    Ok(()) => {
                        show_snackbar("Funcionario excluido com sucesso!", "success");

                        if funcionarios.get_untracked().len() <= 1
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
                            &format!("Erro ao excluir funcionario: {}", error.message),
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

    let on_filter = std::sync::Arc::new(move |next_filters| {
        set_filters.set(next_filters);
        set_current_page.set(1);
    });

    let on_clear_filters = std::sync::Arc::new(move || {
        set_filters.set(FuncionarioFiltersState::default());
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
    let on_view = std::sync::Arc::new(move |funcionario: Funcionario| {
        nav_view(
            &format!("/funcionario/view/{}", funcionario.id),
            Default::default(),
        );
    });

    let nav_edit = navigate.clone();
    let on_edit = std::sync::Arc::new(move |funcionario: Funcionario| {
        nav_edit(
            &format!("/funcionario/edit/{}", funcionario.id),
            Default::default(),
        );
    });

    let on_delete = std::sync::Arc::new(move |funcionario: Funcionario| {
        set_pending_delete.set(Some(funcionario.clone()));
        show_confirm_snackbar(
            &format!(
                "Tem certeza que deseja excluir o funcionario \"{}\"?",
                funcionario.nome
            ),
            "warning",
            "Excluir",
            "Cancelar",
            DELETE_FUNCIONARIO_ACTION,
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
        <PageLayout title="Funcionarios".to_string() max_width="7xl".to_string()>
            <div class="mb-6 flex">
                <div class="flex w-full justify-stretch sm:justify-end">
                    <a
                        href="/funcionario"
                        class="w-full rounded-xl bg-amber-500 px-6 py-3 text-center font-semibold text-white hover:bg-amber-600 transition sm:w-auto"
                    >
                        "+ Novo Funcionario"
                    </a>
                </div>
            </div>

            <div class="space-y-6">
                <FuncionarioFilters
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
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Matricula"</th>
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"CPF"</th>
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Telefone"</th>
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Grupo"</th>
                                <th class="px-6 py-4 text-right text-xs font-semibold uppercase tracking-wider text-slate-500">"Acoes"</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-slate-200 bg-white">
                            {move || {
                                if loading.get() || funcionarios.get().is_empty() {
                                    return vec![view! {
                                        <tr>
                                            <td colspan="7" class="px-6 py-8 text-center text-sm text-slate-500">
                                                {move || if loading.get() { "Carregando funcionarios..." } else { "Nenhum funcionario encontrado." }}
                                            </td>
                                        </tr>
                                    }.into_any()];
                                }

                                let on_view = on_view_table.clone();
                                let on_edit = on_edit_table.clone();
                                let on_delete = on_delete_table.clone();

                                funcionarios.get().into_iter().map(move |funcionario| {
                                    let cpf = apply_cpf_mask(&funcionario.cpf);
                                    let telefone = apply_phone_mask(&funcionario.telefone);
                                    let grupo = grupo_label(funcionario.grupo);

                                    view! {
                                        <tr>
                                            <td class="px-6 py-4 text-slate-700">{funcionario.id}</td>
                                            <td class="px-6 py-4 text-slate-700 font-semibold">{funcionario.nome.clone()}</td>
                                            <td class="px-6 py-4 text-slate-700">{funcionario.matricula.clone()}</td>
                                            <td class="px-6 py-4 text-slate-700">{cpf}</td>
                                            <td class="px-6 py-4 text-slate-700">{telefone}</td>
                                            <td class="px-6 py-4 text-slate-700">{grupo}</td>
                                            <td class="px-6 py-4 text-right">
                                                <ActionButtons
                                                    item=funcionario
                                                    on_view=on_view.clone()
                                                    on_edit=on_edit.clone()
                                                    on_delete=on_delete.clone()
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
                        if loading.get() || funcionarios.get().is_empty() {
                            return vec![view! {
                                <div class="rounded-2xl border border-slate-200 bg-white p-6 text-center text-sm text-slate-500 shadow-sm">
                                    {move || if loading.get() { "Carregando funcionarios..." } else { "Nenhum funcionario encontrado." }}
                                </div>
                            }.into_any()];
                        }

                        let on_view = on_view_mobile.clone();
                        let on_edit = on_edit_mobile.clone();
                        let on_delete = on_delete_mobile.clone();

                        funcionarios.get().into_iter().map(move |funcionario| {
                            let cpf = apply_cpf_mask(&funcionario.cpf);
                            let telefone = apply_phone_mask(&funcionario.telefone);
                            let grupo = grupo_label(funcionario.grupo);

                            view! {
                                <div class="rounded-2xl border border-slate-200 bg-white p-4 shadow-sm">
                                    <div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between sm:gap-4">
                                        <div class="min-w-0">
                                            <p class="break-words text-lg font-semibold text-slate-900">{funcionario.nome.clone()}</p>
                                            <p class="text-sm text-slate-500">{"ID: "}{funcionario.id}</p>
                                        </div>
                                        <div class="min-w-0 sm:text-right">
                                            <p class="break-words text-sm font-semibold text-success-600">{cpf}</p>
                                        </div>
                                    </div>
                                    <div class="mt-4 space-y-3">
                                        <p class="text-sm text-slate-600">{"Matricula: "}{funcionario.matricula.clone()}</p>
                                        <p class="text-sm text-slate-600">{"Telefone: "}{telefone}</p>
                                        <p class="text-sm text-slate-600">{"Grupo: "}{grupo}</p>
                                        <div class="flex justify-end">
                                            <ActionButtons
                                                item=funcionario
                                                on_view=on_view.clone()
                                                on_edit=on_edit.clone()
                                                on_delete=on_delete.clone()
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
