use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params_map};
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;

use crate::{
    components::common::{ComandaValidator, PageLayout},
    constants::comanda_status::ComandaStatus,
    context::auth::use_auth,
    services::comanda_service::{self, Comanda},
    utils::{
        snackbar::show_snackbar,
        user_groups::is_admin,
    },
};

#[derive(Debug, Serialize)]
struct ComandaPayload {
    comanda: String,
    cliente_id: Option<String>,
    funcionario_id: Option<u32>,
    status: i32,
}

fn only_digits(value: &str) -> String {
    value.chars().filter(|value| value.is_ascii_digit()).collect()
}

fn now_datetime_value() -> String {
    js_sys::Date::new_0()
        .to_iso_string()
        .as_string()
        .unwrap_or_default()
        .chars()
        .take(16)
        .collect()
}

fn datetime_input_value(value: Option<String>) -> String {
    value
        .unwrap_or_default()
        .chars()
        .take(16)
        .collect::<String>()
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

#[component]
pub fn ComandaForm() -> impl IntoView {
    let params = use_params_map();
    let navigate = use_navigate();
    let auth = use_auth();

    let route_operation = params.with_untracked(|params| params.get("opr"));
    let route_id = params
        .with_untracked(|params| params.get("id"))
        .and_then(|id| id.parse::<u32>().ok());
    let is_view = route_operation.as_deref() == Some("view");
    let is_edit = route_operation.as_deref() == Some("edit");

    let is_view_mode = move || is_view;
    let page_title = move || {
        if is_view {
            "Visualizar Comanda".to_string()
        } else if is_edit {
            "Editar Comanda".to_string()
        } else {
            "Abertura de Comanda".to_string()
        }
    };

    let (comanda, set_comanda) = signal(String::new());
    let (original_comanda, set_original_comanda) = signal(String::new());
    let (data_hora, set_data_hora) = signal(now_datetime_value());
    let (cliente_id, set_cliente_id) = signal(String::new());
    let (funcionario_id, set_funcionario_id) = signal(String::new());
    let (status, set_status) = signal(ComandaStatus::Aberta as i32);
    let (loading, set_loading) = signal(false);
    let (loading_data, set_loading_data) = signal(route_id.is_some());
    let (validator_open, set_validator_open) = signal(false);
    let (existing_record, set_existing_record) = signal(None::<Comanda>);

    let nav_permission = navigate.clone();
    let auth_loading = auth.loading;
    let auth_user = auth.user;
    Effect::new(move |_| {
        if auth_loading.get() {
            return;
        }

        if is_edit && !is_admin(auth_user.get().as_ref()) {
            show_snackbar(
                "Acesso negado: apenas administradores podem editar comandas.",
                "warning",
            );
            nav_permission("/comandas", Default::default());
        }
    });

    Effect::new(move |_| {
        if route_id.is_some() {
            return;
        }

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
            return;
        };

        leptos::task::spawn_local(async move {
            set_loading_data.set(true);

            match comanda_service::get_by_id(id).await {
                Ok(data) => {
                    set_comanda.set(data.comanda.clone());
                    set_original_comanda.set(data.comanda);
                    set_data_hora.set(datetime_input_value(data.data_hora));
                    set_cliente_id.set(
                        data.cliente_id
                            .map(|value| value.to_string())
                            .unwrap_or_default(),
                    );
                    set_funcionario_id.set(
                        data.funcionario_id
                            .map(|value| value.to_string())
                            .unwrap_or_default(),
                    );
                    set_status.set(data.status);
                }
                Err(error) => {
                    show_snackbar(
                        &format!("Erro ao carregar comanda: {}", error.message),
                        "error",
                    );
                }
            }

            set_loading_data.set(false);
        });
    });

    let nav_cancel = navigate.clone();
    let on_cancel = move |_| {
        nav_cancel("/comandas", Default::default());
    };

    let close_validator = Arc::new(move || {
        set_validator_open.set(false);
    });

    let clear_comanda_field = Arc::new(move || {
        set_comanda.set(String::new());
        set_existing_record.set(None);
    });

    let nav_validator_view = navigate.clone();
    let validator_view = Arc::new(move |record: Comanda| {
        nav_validator_view(&format!("/comanda/view/{}", record.id), Default::default());
    });

    let nav_validator_edit = navigate.clone();
    let validator_edit = Arc::new(move |record: Comanda| {
        nav_validator_edit(&format!("/comanda/edit/{}", record.id), Default::default());
    });

    let validate_current_comanda = move || {
        if is_view_mode() {
            return;
        }

        let value = only_digits(&comanda.get());

        if value.is_empty() {
            return;
        }

        if route_id.is_some() && value == original_comanda.get_untracked() {
            return;
        }

        leptos::task::spawn_local(async move {
            match comanda_service::check_in_use(value, route_id).await {
                Ok(Some(record)) => {
                    set_existing_record.set(Some(record));
                    set_validator_open.set(true);
                }
                Ok(None) => {
                    set_existing_record.set(None);
                    set_validator_open.set(false);
                }
                Err(error) => {
                    show_snackbar(
                        &format!("Erro ao validar comanda: {}", error.message),
                        "error",
                    );
                }
            }
        });
    };

    let nav_after_submit = navigate.clone();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        if is_view_mode() {
            return;
        }

        let comanda_value = only_digits(&comanda.get());
        let cliente_value = cliente_id.get().trim().to_string();
        let funcionario_value = funcionario_id
            .get()
            .trim()
            .parse::<u32>()
            .ok()
            .or_else(|| user_id(auth.user.get_untracked()));
        let status_value = status.get();

        if comanda_value.is_empty() {
            show_snackbar("Numero da comanda e obrigatorio", "warning");
            return;
        }

        if funcionario_value.is_none() {
            show_snackbar("Funcionario responsavel nao encontrado", "warning");
            return;
        }

        let id = route_id;
        let navigate = nav_after_submit.clone();

        leptos::task::spawn_local(async move {
            set_loading.set(true);

            match comanda_service::check_in_use(comanda_value.clone(), id).await {
                Ok(Some(record)) => {
                    set_existing_record.set(Some(record));
                    set_validator_open.set(true);
                    set_loading.set(false);
                    return;
                }
                Ok(None) => {}
                Err(error) => {
                    show_snackbar(
                        &format!("Erro ao validar comanda: {}", error.message),
                        "error",
                    );
                    set_loading.set(false);
                    return;
                }
            }

            let payload = ComandaPayload {
                comanda: comanda_value,
                cliente_id: if cliente_value.is_empty() {
                    None
                } else {
                    Some(cliente_value)
                },
                funcionario_id: funcionario_value,
                status: status_value,
            };

            let result = match id {
                Some(id) => comanda_service::update(id, &payload).await,
                None => comanda_service::create(&payload).await,
            };

            match result {
                Ok(_) => {
                    show_snackbar("Comanda salva com sucesso!", "success");
                    navigate("/comandas", Default::default());
                }
                Err(error) => {
                    show_snackbar(
                        &format!("Erro ao salvar comanda: {}", error.message),
                        "error",
                    );
                }
            }

            set_loading.set(false);
        });
    };

    let field_disabled = move || loading.get() || loading_data.get() || is_view_mode();
    let save_disabled = move || loading.get() || loading_data.get();
    let funcionario_label = move || {
        let id = funcionario_id.get();
        let name = user_name(auth.user.get()).unwrap_or_else(|| "Funcionario".to_string());

        if id.trim().is_empty() {
            name
        } else {
            format!("ID: {id} - {name}")
        }
    };
    let input_class = "w-full min-w-0 rounded-xl border border-slate-200 px-4 py-3 focus:border-amber-500 focus:outline-none focus:ring-2 focus:ring-amber-200 disabled:cursor-not-allowed disabled:bg-slate-100 disabled:text-slate-500";

    view! {
        <PageLayout title=page_title() max_width="lg".to_string()>
            <form on:submit=on_submit class="space-y-8">
                <div class="grid gap-6 sm:grid-cols-2">
                    <div class="min-w-0 space-y-2">
                        <label class="block text-sm font-semibold text-slate-700">"Numero da comanda"</label>
                        <input
                            class=input_class
                            type="text"
                            inputmode="numeric"
                            prop:value=move || comanda.get()
                            placeholder="Informe o numero da comanda"
                            disabled=field_disabled
                            on:input=move |event| set_comanda.set(only_digits(&event_target_value(&event)))
                            on:blur=move |_| validate_current_comanda()
                        />
                    </div>

                    <div class="min-w-0 space-y-2">
                        <label class="block text-sm font-semibold text-slate-700">"Data e hora da abertura"</label>
                        <input
                            class=input_class
                            type="datetime-local"
                            prop:value=move || data_hora.get()
                            disabled=move || true
                            on:input=move |event| set_data_hora.set(event_target_value(&event))
                        />
                    </div>

                    <div class="min-w-0 space-y-2">
                        <label class="block text-sm font-semibold text-slate-700">"Funcionario responsavel"</label>
                        <input
                            class=input_class
                            type="text"
                            prop:value=funcionario_label
                            disabled=move || true
                        />
                    </div>

                    <div class="min-w-0 space-y-2">
                        <label class="block text-sm font-semibold text-slate-700">"Identificacao do cliente"</label>
                        <input
                            class=input_class
                            type="text"
                            prop:value=move || cliente_id.get()
                            placeholder="Nome, codigo ou observacao"
                            disabled=field_disabled
                            on:input=move |event| set_cliente_id.set(event_target_value(&event))
                        />
                    </div>
                </div>

                <div class="flex flex-col gap-3 sm:flex-row sm:justify-end">
                    <button
                        type="button"
                        on:click=on_cancel
                        class="rounded-xl border border-slate-200 bg-white px-6 py-3 font-semibold text-slate-700 transition hover:border-slate-300"
                    >
                        {move || if is_view_mode() { "Voltar" } else { "Cancelar" }}
                    </button>

                    <Show when=move || !is_view_mode()>
                        <button
                            type="submit"
                            disabled=save_disabled
                            class="rounded-xl bg-amber-500 px-6 py-3 font-semibold text-white transition hover:bg-amber-600 disabled:cursor-not-allowed disabled:opacity-60"
                        >
                            {move || if loading.get() { "Salvando..." } else if route_id.is_some() { "Atualizar" } else { "Abrir Comanda" }}
                        </button>
                    </Show>
                </div>
            </form>

            <ComandaValidator
                open=validator_open
                existing_record=existing_record
                on_close=close_validator
                on_clear_field=clear_comanda_field
                record_type="comanda".to_string()
                on_view=validator_view
                on_edit=validator_edit
            />
        </PageLayout>
    }
}
