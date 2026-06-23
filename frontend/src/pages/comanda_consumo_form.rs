use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params_map};
use serde::Serialize;
use serde_json::Value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CustomEvent};

use crate::{
    components::common::{ActionButtons, PageLayout},
    constants::comanda_status::status_config,
    context::auth::use_auth,
    services::{
        api::ApiError,
        comanda_service::{self, Comanda, ComandaItem, ComandaItemListParams},
        produto_service::{self, Produto, ProdutoListParams},
    },
    utils::{
        snackbar::{show_confirm_snackbar, show_snackbar},
        user_groups::is_admin,
    },
};

const DELETE_ITEM_ACTION: &str = "delete-comanda-item";

#[derive(Debug, Serialize)]
struct ComandaItemCreatePayload {
    produto_id: u32,
    quantidade: u32,
    funcionario_id: u32,
    valor_unitario: f64,
}

#[derive(Debug, Serialize)]
struct ComandaItemUpdatePayload {
    quantidade: u32,
    valor_unitario: f64,
}

fn parse_quantity(value: &str) -> Option<u32> {
    value.trim().parse::<u32>().ok()
}

fn format_quantity(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        format!("{value:.2}")
    }
}

fn format_currency(value: f64) -> String {
    format!("R$ {:.2}", value)
}

fn display_optional_id(value: Option<u32>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn display_datetime(value: Option<String>) -> String {
    value
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "-".to_string())
}

fn value_u32(value: &Value, key: &str) -> Option<u32> {
    value.get(key).and_then(|field| {
        field
            .as_u64()
            .and_then(|value| u32::try_from(value).ok())
            .or_else(|| field.as_str()?.parse::<u32>().ok())
    })
}

fn value_string(value: &Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(|field| field.as_str())
        .filter(|value| !value.trim().is_empty())
        .map(ToString::to_string)
}

fn user_id(user: Option<Value>) -> Option<u32> {
    let user = user?;

    value_u32(&user, "id")
        .or_else(|| value_u32(&user, "funcionario_id"))
        .or_else(|| value_u32(&user, "sub"))
}

fn user_name(user: Option<Value>) -> Option<String> {
    let user = user?;

    value_string(&user, "nome")
        .or_else(|| value_string(&user, "name"))
        .or_else(|| value_string(&user, "usuario"))
}

fn find_product_name(produtos: &[Produto], item: &ComandaItem) -> String {
    if !item.produto_nome.trim().is_empty() {
        return item.produto_nome.clone();
    }

    item.produto_id
        .and_then(|id| produtos.iter().find(|produto| produto.id == id))
        .map(|produto| produto.nome.clone())
        .unwrap_or_else(|| "Produto nao encontrado".to_string())
}

fn display_funcionario(item: &ComandaItem) -> String {
    if !item.funcionario_nome.trim().is_empty() {
        return item.funcionario_nome.clone();
    }

    display_optional_id(item.funcionario_id)
}

fn display_cliente(comanda: &Comanda) -> String {
    if !comanda.cliente_nome.trim().is_empty() {
        return comanda.cliente_nome.clone();
    }

    display_optional_id(comanda.cliente_id)
}

fn display_comanda_funcionario(comanda: &Comanda) -> String {
    if !comanda.funcionario_nome.trim().is_empty() {
        return comanda.funcionario_nome.clone();
    }

    display_optional_id(comanda.funcionario_id)
}

async fn load_items(comanda_id: u32) -> Result<Vec<ComandaItem>, ApiError> {
    comanda_service::list_items(
        comanda_id,
        ComandaItemListParams {
            skip: 0,
            limit: 1000,
        },
    )
    .await
}

#[component]
pub fn ComandaConsumoForm() -> impl IntoView {
    let params = use_params_map();
    let navigate = use_navigate();
    let auth = use_auth();
    let user = auth.user;

    let route_id = params
        .with_untracked(|params| params.get("id"))
        .and_then(|id| id.parse::<u32>().ok());

    let (comanda_data, set_comanda_data) = signal(None::<Comanda>);
    let (items, set_items) = signal(Vec::<ComandaItem>::new());
    let (produtos, set_produtos) = signal(Vec::<Produto>::new());
    let (produto_id, set_produto_id) = signal(String::new());
    let (quantidade, set_quantidade) = signal("1".to_string());
    let (funcionario_id, set_funcionario_id) = signal(String::new());
    let (loading, set_loading) = signal(false);
    let (loading_data, set_loading_data) = signal(true);
    let (loading_produtos, set_loading_produtos) = signal(false);
    let (editing_item, set_editing_item) = signal(None::<ComandaItem>);
    let (pending_delete, set_pending_delete) = signal(None::<ComandaItem>);

    Effect::new(move |_| {
        if !funcionario_id.get_untracked().trim().is_empty() {
            return;
        }

        if let Some(id) = user_id(auth.user.get()) {
            set_funcionario_id.set(id.to_string());
        }
    });

    Effect::new(move |_| {
        let Some(id) = route_id else {
            set_loading_data.set(false);
            show_snackbar("Comanda nao informada", "error");
            return;
        };

        leptos::task::spawn_local(async move {
            set_loading_data.set(true);
            set_loading_produtos.set(true);

            let comanda_result = comanda_service::get_by_id(id).await;
            let items_result = load_items(id).await;
            let produtos_result = produto_service::list(ProdutoListParams {
                skip: 0,
                limit: 1000,
                ..ProdutoListParams::default()
            })
            .await;

            match comanda_result {
                Ok(comanda) => set_comanda_data.set(Some(comanda)),
                Err(error) => show_snackbar(
                    &format!("Erro ao carregar comanda: {}", error.message),
                    "error",
                ),
            }

            match items_result {
                Ok(next_items) => set_items.set(next_items),
                Err(error) => show_snackbar(
                    &format!("Erro ao carregar itens: {}", error.message),
                    "error",
                ),
            }

            match produtos_result {
                Ok(next_produtos) => set_produtos.set(next_produtos),
                Err(error) => show_snackbar(
                    &format!("Erro ao carregar produtos: {}", error.message),
                    "error",
                ),
            }

            set_loading_produtos.set(false);
            set_loading_data.set(false);
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

            if custom.detail().as_string().as_deref() != Some(DELETE_ITEM_ACTION) {
                return;
            }

            let Some(comanda_id) = route_id else {
                return;
            };

            let Some(item) = pending_delete.get_untracked() else {
                return;
            };

            let item_id = item.id;

            set_pending_delete.set(None);

            leptos::task::spawn_local(async move {
                set_loading.set(true);

                match comanda_service::remove_item(item_id).await {
                    Ok(()) => {
                        show_snackbar("Item removido com sucesso!", "success");

                        match load_items(comanda_id).await {
                            Ok(next_items) => set_items.set(next_items),
                            Err(error) => show_snackbar(
                                &format!("Erro ao recarregar itens: {}", error.message),
                                "error",
                            ),
                        }

                        if let Ok(next_comanda) = comanda_service::get_by_id(comanda_id).await {
                            set_comanda_data.set(Some(next_comanda));
                        }
                    }
                    Err(error) => show_snackbar(
                        &format!("Erro ao remover item: {}", error.message),
                        "error",
                    ),
                }

                set_loading.set(false);
            });
        }) as Box<dyn FnMut(_)>);

        let _ = win.add_event_listener_with_callback(
            "snackbarConfirmed",
            listener.as_ref().unchecked_ref(),
        );

        listener.forget();
    });

    let reset_form = move || {
        set_produto_id.set(String::new());
        set_quantidade.set("1".to_string());
        set_editing_item.set(None);

        if let Some(id) = user_id(auth.user.get_untracked()) {
            set_funcionario_id.set(id.to_string());
        }
    };

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();

        let Some(comanda_id) = route_id else {
            show_snackbar("Comanda nao informada", "error");
            return;
        };

        let Some(produto_value) = produto_id.get().parse::<u32>().ok() else {
            show_snackbar("Selecione um produto", "warning");
            return;
        };

        let Some(quantidade_value) = parse_quantity(&quantidade.get()) else {
            show_snackbar("Quantidade invalida", "warning");
            return;
        };

        if quantidade_value == 0 {
            show_snackbar("Quantidade deve ser maior que zero", "warning");
            return;
        }

        let funcionario_value = funcionario_id
            .get()
            .parse::<u32>()
            .ok()
            .or_else(|| user_id(auth.user.get_untracked()));

        let Some(funcionario_value) = funcionario_value else {
            show_snackbar("Funcionario responsavel nao encontrado", "warning");
            return;
        };

        let valor_unitario = produtos
            .get_untracked()
            .into_iter()
            .find(|produto| produto.id == produto_value)
            .map(|produto| produto.valor_unitario)
            .unwrap_or(0.0);

        let editing = editing_item.get_untracked();

        leptos::task::spawn_local(async move {
            set_loading.set(true);

            let result = if let Some(item) = editing {
                let payload = ComandaItemUpdatePayload {
                    quantidade: quantidade_value,
                    valor_unitario,
                };

                comanda_service::update_item(item.id, &payload).await
            } else {
                let payload = ComandaItemCreatePayload {
                    produto_id: produto_value,
                    quantidade: quantidade_value,
                    funcionario_id: funcionario_value,
                    valor_unitario,
                };

                comanda_service::add_item(comanda_id, &payload).await
            };

            match result {
                Ok(_) => {
                    show_snackbar("Item salvo com sucesso!", "success");
                    reset_form();

                    match load_items(comanda_id).await {
                        Ok(next_items) => set_items.set(next_items),
                        Err(error) => show_snackbar(
                            &format!("Erro ao recarregar itens: {}", error.message),
                            "error",
                        ),
                    }

                    if let Ok(next_comanda) = comanda_service::get_by_id(comanda_id).await {
                        set_comanda_data.set(Some(next_comanda));
                    }
                }
                Err(error) => show_snackbar(
                    &format!("Erro ao salvar item: {}", error.message),
                    "error",
                ),
            }

            set_loading.set(false);
        });
    };

    let nav_back = navigate.clone();
    let on_cancel = move |_| {
        nav_back("/comandas", Default::default());
    };

    let on_edit_item = std::sync::Arc::new(move |item: ComandaItem| {
        set_produto_id.set(
            item.produto_id
                .map(|value| value.to_string())
                .unwrap_or_default(),
        );
        set_quantidade.set(format_quantity(item.quantidade));
        set_funcionario_id.set(
            item.funcionario_id
                .or_else(|| user_id(auth.user.get_untracked()))
                .map(|value| value.to_string())
                .unwrap_or_default(),
        );
        set_editing_item.set(Some(item));
    });

    let on_delete_item = std::sync::Arc::new(move |item: ComandaItem| {
        set_pending_delete.set(Some(item.clone()));

        show_confirm_snackbar(
            &format!(
                "Tem certeza que deseja remover \"{}\" com quantidade {}?",
                find_product_name(&produtos.get_untracked(), &item),
                format_quantity(item.quantidade)
            ),
            "warning",
            "Remover",
            "Cancelar",
            DELETE_ITEM_ACTION,
        );
    });

    let on_view_item = std::sync::Arc::new(move |_item: ComandaItem| {});
    let cancel_edit = move |_| reset_form();

    let on_edit_table = on_edit_item.clone();
    let on_delete_table = on_delete_item.clone();
    let on_view_table = on_view_item.clone();
    let on_edit_mobile = on_edit_item.clone();
    let on_delete_mobile = on_delete_item.clone();
    let on_view_mobile = on_view_item.clone();

    let form_disabled = move || loading.get() || loading_data.get() || loading_produtos.get();
    let page_title = move || {
        comanda_data
            .get()
            .map(|comanda| format!("Consumo - Comanda {}", comanda.comanda))
            .unwrap_or_else(|| "Consumo da Comanda".to_string())
    };
    let input_class = "w-full min-w-0 rounded-xl border border-slate-200 px-4 py-3 focus:border-amber-500 focus:outline-none focus:ring-2 focus:ring-amber-200 disabled:cursor-not-allowed disabled:bg-slate-100 disabled:text-slate-500";

    view! {
        <PageLayout title=page_title() max_width="7xl".to_string()>
            <div class="space-y-6">
                {move || {
                    if loading_data.get() {
                        return view! {
                            <div class="rounded-2xl border border-slate-200 bg-white p-8 text-center text-sm text-slate-500 shadow-sm">
                                "Carregando consumo..."
                            </div>
                        }.into_any();
                    }

                    let Some(comanda) = comanda_data.get() else {
                        return view! {
                            <div class="rounded-2xl border border-slate-200 bg-white p-8 text-center text-sm text-slate-500 shadow-sm">
                                "Comanda nao encontrada."
                            </div>
                        }.into_any();
                    };

                    let status = status_config(comanda.status);
                    let cliente = display_cliente(&comanda);
                    let funcionario = display_comanda_funcionario(&comanda);
                    let total = comanda_service::items_total(&items.get());

                    view! {
                        <section class="rounded-2xl border border-slate-200 bg-slate-50 p-4 shadow-sm sm:p-5">
                            <div class="flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
                                <div class="min-w-0">
                                    <p class="text-sm font-semibold uppercase tracking-wide text-slate-500">"Comanda"</p>
                                    <h2 class="mt-1 break-words text-2xl font-semibold text-slate-900">
                                        {comanda.comanda.clone()}
                                    </h2>
                                </div>
                                <span class=format!("inline-flex w-fit rounded-full border px-3 py-1 text-sm font-semibold {}", status.badge_class)>
                                    {status.label}
                                </span>
                            </div>

                            <div class="mt-4 grid gap-4 text-sm text-slate-600 sm:grid-cols-2 lg:grid-cols-4">
                                <p><span class="font-semibold text-slate-700">"Abertura: "</span>{display_datetime(comanda.data_hora.clone())}</p>
                                <p><span class="font-semibold text-slate-700">"Cliente: "</span>{cliente}</p>
                                <p><span class="font-semibold text-slate-700">"Funcionario: "</span>{funcionario}</p>
                                <p><span class="font-semibold text-slate-700">"Total: "</span>{format_currency(total)}</p>
                            </div>
                        </section>
                    }.into_any()
                }}

                <section class="rounded-2xl border border-slate-200 bg-white p-4 shadow-sm sm:p-5">
                    <form on:submit=on_submit class="space-y-5">
                        <div class="flex flex-col gap-1">
                            <h2 class="text-lg font-semibold text-slate-900">
                                {move || if editing_item.get().is_some() { "Editar Item de Consumo" } else { "Adicionar Item de Consumo" }}
                            </h2>
                            <p class="text-sm text-slate-500">
                                {move || if loading_produtos.get() { "Carregando produtos..." } else { "Selecione o produto e informe a quantidade." }}
                            </p>
                        </div>

                        <div class="grid gap-4 md:grid-cols-[minmax(0,1fr)_180px_minmax(180px,240px)]">
                            <div class="min-w-0 space-y-2">
                                <label class="block text-sm font-semibold text-slate-700">"Produto"</label>
                                <select
                                    class=input_class
                                    prop:value=move || produto_id.get()
                                    disabled=move || form_disabled() || editing_item.get().is_some()
                                    on:change=move |event| set_produto_id.set(event_target_value(&event))
                                >
                                    <option value="">"Selecione um produto"</option>
                                    {move || produtos.get().into_iter().map(|produto| view! {
                                        <option value=produto.id.to_string()>
                                            {format!("{} - {}", produto.nome, format_currency(produto.valor_unitario))}
                                        </option>
                                    }).collect::<Vec<_>>()}
                                </select>
                            </div>

                            <div class="min-w-0 space-y-2">
                                <label class="block text-sm font-semibold text-slate-700">"Quantidade"</label>
                                <input
                                    class=input_class
                                    type="number"
                                    min="1"
                                    step="1"
                                    prop:value=move || quantidade.get()
                                    disabled=form_disabled
                                    on:input=move |event| set_quantidade.set(event_target_value(&event))
                                />
                            </div>

                            <div class="min-w-0 space-y-2">
                                <label class="block text-sm font-semibold text-slate-700">"Funcionario"</label>
                                <input
                                    class=input_class
                                    type="text"
                                    prop:value=move || {
                                        let id = funcionario_id.get();
                                        let name = user_name(auth.user.get()).unwrap_or_else(|| "Funcionario".to_string());

                                        if id.trim().is_empty() {
                                            name
                                        } else {
                                            format!("ID: {id} - {name}")
                                        }
                                    }
                                    disabled=move || true
                                />
                            </div>
                        </div>

                        <div class="flex flex-col gap-3 sm:flex-row sm:justify-end">
                            <Show when=move || editing_item.get().is_some()>
                                <button
                                    type="button"
                                    on:click=cancel_edit
                                    disabled=form_disabled
                                    class="rounded-xl border border-slate-200 bg-white px-5 py-2.5 text-sm font-semibold text-slate-700 transition hover:border-slate-300 disabled:cursor-not-allowed disabled:opacity-60"
                                >
                                    "Cancelar Edicao"
                                </button>
                            </Show>
                            <button
                                type="submit"
                                disabled=form_disabled
                                class="rounded-xl bg-amber-500 px-5 py-2.5 text-sm font-semibold text-white transition hover:bg-amber-600 disabled:cursor-not-allowed disabled:opacity-60"
                            >
                                {move || {
                                    if loading.get() {
                                        "Salvando..."
                                    } else if editing_item.get().is_some() {
                                        "Atualizar Item"
                                    } else {
                                        "Adicionar Item"
                                    }
                                }}
                            </button>
                        </div>
                    </form>
                </section>

                <section class="space-y-4">
                    <h2 class="text-lg font-semibold text-slate-900">"Itens de consumo"</h2>

                    <div class="hidden overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-sm md:block">
                        <table class="min-w-full divide-y divide-slate-200">
                            <thead class="bg-slate-50">
                                <tr>
                                    <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Produto"</th>
                                    <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Quantidade"</th>
                                    <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Valor Unitario"</th>
                                    <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Valor Total"</th>
                                    <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Funcionario"</th>
                                    <th class="px-6 py-4 text-right text-xs font-semibold uppercase tracking-wider text-slate-500">"Acoes"</th>
                                </tr>
                            </thead>
                            <tbody class="divide-y divide-slate-200 bg-white">
                                {move || {
                                    if items.get().is_empty() {
                                        return vec![view! {
                                            <tr>
                                                <td colspan="6" class="px-6 py-8 text-center text-sm text-slate-500">
                                                    "Nenhum item de consumo registrado."
                                                </td>
                                            </tr>
                                        }.into_any()];
                                    }

                                    let produtos_snapshot = produtos.get();
                                    let on_view = on_view_table.clone();
                                    let on_edit = on_edit_table.clone();
                                    let on_delete = on_delete_table.clone();
                                    let can_manage_items = is_admin(user.get().as_ref());

                                    items.get().into_iter().map(move |item| {
                                        let produto_nome = find_product_name(&produtos_snapshot, &item);
                                        let valor_total = comanda_service::item_total(&item);

                                        view! {
                                            <tr>
                                                <td class="px-6 py-4 font-semibold text-slate-700">{produto_nome}</td>
                                                <td class="px-6 py-4 text-slate-700">{format_quantity(item.quantidade)}</td>
                                                <td class="px-6 py-4 text-slate-700">{format_currency(item.valor_unitario)}</td>
                                                <td class="px-6 py-4 text-slate-700">{format_currency(valor_total)}</td>
                                                <td class="px-6 py-4 text-slate-700">{display_funcionario(&item)}</td>
                                                <td class="px-6 py-4 text-right">
                                                    <ActionButtons
                                                        item=item
                                                        on_view=on_view.clone()
                                                        on_edit=on_edit.clone()
                                                        on_delete=on_delete.clone()
                                                        show_view=false
                                                        show_edit=can_manage_items
                                                        show_delete=can_manage_items
                                                    />
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
                            if items.get().is_empty() {
                                return vec![view! {
                                    <div class="rounded-2xl border border-slate-200 bg-white p-6 text-center text-sm text-slate-500 shadow-sm">
                                        "Nenhum item de consumo registrado."
                                    </div>
                                }.into_any()];
                            }

                            let produtos_snapshot = produtos.get();
                            let on_view = on_view_mobile.clone();
                            let on_edit = on_edit_mobile.clone();
                            let on_delete = on_delete_mobile.clone();
                            let can_manage_items = is_admin(user.get().as_ref());

                            items.get().into_iter().map(move |item| {
                                let produto_nome = find_product_name(&produtos_snapshot, &item);
                                let valor_total = comanda_service::item_total(&item);

                                view! {
                                    <div class="rounded-2xl border border-slate-200 bg-white p-4 shadow-sm">
                                        <div class="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
                                            <div class="min-w-0">
                                                <p class="break-words text-lg font-semibold text-slate-900">{produto_nome}</p>
                                                <p class="text-sm text-slate-500">{"Quantidade: "}{format_quantity(item.quantidade)}</p>
                                            </div>
                                            <p class="text-base font-semibold text-slate-900">{format_currency(valor_total)}</p>
                                        </div>

                                        <div class="mt-4 space-y-2 text-sm text-slate-600">
                                            <p>{"Valor unitario: "}{format_currency(item.valor_unitario)}</p>
                                            <p>{"Funcionario: "}{display_funcionario(&item)}</p>
                                        </div>

                                        <div class="mt-4 flex justify-end">
                                            <ActionButtons
                                                item=item
                                                on_view=on_view.clone()
                                                on_edit=on_edit.clone()
                                                on_delete=on_delete.clone()
                                                show_view=false
                                                show_edit=can_manage_items
                                                show_delete=can_manage_items
                                            />
                                        </div>
                                    </div>
                                }.into_any()
                            }).collect::<Vec<_>>()
                        }}
                    </div>
                </section>

                <div class="flex justify-end">
                    <button
                        type="button"
                        on:click=on_cancel
                        disabled=move || loading.get()
                        class="rounded-xl border border-slate-200 bg-white px-6 py-3 font-semibold text-slate-700 transition hover:border-slate-300 disabled:cursor-not-allowed disabled:opacity-60"
                    >
                        "Voltar"
                    </button>
                </div>
            </div>
        </PageLayout>
    }
}
