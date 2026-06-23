use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params_map};

use crate::{
    components::common::PageLayout,
    context::auth::use_auth,
    hooks::masks::{apply_cpf_mask, apply_phone_mask, only_digits},
    services::cliente_service::{self, ClientePayload},
    utils::{
        snackbar::show_snackbar,
        user_groups::is_admin_or_caixa,
    },
};

#[component]
pub fn ClienteForm() -> impl IntoView {
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
            "Visualizar Cliente".to_string()
        } else if is_edit {
            "Editar Cliente".to_string()
        } else {
            "Cadastro de Cliente".to_string()
        }
    };

    let (nome, set_nome) = signal(String::new());
    let (cpf, set_cpf) = signal(String::new());
    let (telefone, set_telefone) = signal(String::new());
    let (loading, set_loading) = signal(false);

    let nav_permission = navigate.clone();
    let auth_loading = auth.loading;
    let auth_user = auth.user;
    Effect::new(move |_| {
        if auth_loading.get() {
            return;
        }

        if !is_view && !is_admin_or_caixa(auth_user.get().as_ref()) {
            show_snackbar(
                "Acesso negado: apenas administradores e caixas podem cadastrar ou editar clientes.",
                "warning",
            );
            nav_permission("/clientes", Default::default());
        }
    });

    Effect::new(move |_| {
        let Some(id) = route_id else {
            return;
        };

        leptos::task::spawn_local(async move {
            set_loading.set(true);

            match cliente_service::get_by_id(id).await {
                Ok(cliente) => {
                    set_nome.set(cliente.nome);
                    set_cpf.set(apply_cpf_mask(&cliente.cpf));
                    set_telefone.set(apply_phone_mask(&cliente.telefone));
                }
                Err(error) => {
                    show_snackbar(&format!("Erro ao carregar cliente: {}", error.message), "error");
                }
            }

            set_loading.set(false);
        });
    });

    let nav_cancel = navigate.clone();
    let on_cancel = move |_| {
        nav_cancel("/clientes", Default::default());
    };

    let nav_after_submit = navigate.clone();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        if is_view_mode() {
            return;
        }

        let nome_value = nome.get().trim().to_string();
        let cpf_value = only_digits(&cpf.get());
        let telefone_value = only_digits(&telefone.get());

        if nome_value.is_empty() {
            show_snackbar("Nome do cliente e obrigatorio", "warning");
            return;
        }

        if cpf_value.is_empty() {
            show_snackbar("CPF do cliente e obrigatorio", "warning");
            return;
        }

        if cpf_value.len() != 11 {
            show_snackbar("CPF deve ter 11 digitos", "warning");
            return;
        }

        if telefone_value.is_empty() {
            show_snackbar("Telefone do cliente e obrigatorio", "warning");
            return;
        }

        if !(10..=11).contains(&telefone_value.len()) {
            show_snackbar("Telefone deve ter 10 ou 11 digitos", "warning");
            return;
        }

        let payload = ClientePayload {
            nome: nome_value,
            cpf: cpf_value,
            telefone: telefone_value,
        };
        let id = route_id;
        let navigate = nav_after_submit.clone();

        leptos::task::spawn_local(async move {
            set_loading.set(true);

            let result = match id {
                Some(id) => cliente_service::update(id, &payload).await,
                None => cliente_service::create(&payload).await,
            };

            match result {
                Ok(_) => {
                    show_snackbar("Cliente salvo com sucesso!", "success");
                    navigate("/clientes", Default::default());
                }
                Err(error) => {
                    show_snackbar(&format!("Erro ao salvar cliente: {}", error.message), "error");
                }
            }

            set_loading.set(false);
        });
    };

    let field_disabled = move || loading.get() || is_view_mode();
    let input_class = "w-full min-w-0 rounded-xl border border-slate-200 px-4 py-3 focus:border-amber-500 focus:outline-none focus:ring-2 focus:ring-amber-200 disabled:cursor-not-allowed disabled:bg-slate-100 disabled:text-slate-500";

    view! {
        <PageLayout title=page_title() max_width="lg".to_string()>
            <form on:submit=on_submit class="space-y-8">
                <div class="grid gap-6 sm:grid-cols-2">
                    <div class="min-w-0 space-y-2">
                        <label class="block text-sm font-semibold text-slate-700">"Nome do cliente"</label>
                        <input
                            class=input_class
                            type="text"
                            prop:value=move || nome.get()
                            placeholder="Digite o nome do cliente"
                            disabled=field_disabled
                            on:input=move |event| set_nome.set(event_target_value(&event))
                        />
                    </div>

                    <div class="min-w-0 space-y-2">
                        <label class="block text-sm font-semibold text-slate-700">"CPF"</label>
                        <input
                            class=input_class
                            type="text"
                            inputmode="numeric"
                            prop:value=move || cpf.get()
                            placeholder="000.000.000-00"
                            disabled=field_disabled
                            on:input=move |event| set_cpf.set(apply_cpf_mask(&event_target_value(&event)))
                        />
                    </div>

                    <div class="min-w-0 space-y-2">
                        <label class="block text-sm font-semibold text-slate-700">"Telefone"</label>
                        <input
                            class=input_class
                            type="tel"
                            inputmode="numeric"
                            prop:value=move || telefone.get()
                            placeholder="(00) 00000-0000"
                            disabled=field_disabled
                            on:input=move |event| set_telefone.set(apply_phone_mask(&event_target_value(&event)))
                        />
                    </div>
                </div>

                <div class="flex flex-col gap-3 sm:flex-row sm:justify-end">
                    <button
                        type="button"
                        on:click=on_cancel
                        class="rounded-xl border border-slate-200 bg-white px-6 py-3 font-semibold text-slate-700 hover:border-slate-300 transition"
                    >
                        {move || if is_view_mode() { "Voltar" } else { "Cancelar" }}
                    </button>

                    <Show when=move || !is_view_mode()>
                        <button
                            type="submit"
                            disabled=move || loading.get()
                            class="rounded-xl bg-amber-500 px-6 py-3 font-semibold text-white hover:bg-amber-600 transition disabled:cursor-not-allowed disabled:opacity-60"
                        >
                            {move || if loading.get() { "Salvando..." } else { "Salvar" }}
                        </button>
                    </Show>
                </div>
            </form>
        </PageLayout>
    }
}
