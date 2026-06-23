use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params_map};

use crate::{
    components::common::PageLayout,
    context::auth::use_auth,
    hooks::masks::{apply_cpf_mask, apply_phone_mask, only_digits},
    services::funcionario_service::{
        self, FuncionarioCreatePayload, FuncionarioUpdatePayload,
    },
    utils::{
        snackbar::show_snackbar,
        user_groups::is_admin,
    },
};

const MAX_MATRICULA_CHARS: usize = 10;

fn limit_matricula(value: &str) -> String {
    value.chars().take(MAX_MATRICULA_CHARS).collect()
}

fn parse_grupo(value: &str) -> Option<i32> {
    match value.trim().parse::<i32>().ok()? {
        grupo @ 1..=3 => Some(grupo),
        _ => None,
    }
}

#[component]
pub fn FuncionarioForm() -> impl IntoView {
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
    let is_create_mode = move || route_id.is_none();
    let page_title = move || {
        if is_view {
            "Visualizar Funcionario".to_string()
        } else if is_edit {
            "Editar Funcionario".to_string()
        } else {
            "Cadastro de Funcionario".to_string()
        }
    };

    let (nome, set_nome) = signal(String::new());
    let (matricula, set_matricula) = signal(String::new());
    let (cpf, set_cpf) = signal(String::new());
    let (telefone, set_telefone) = signal(String::new());
    let (grupo, set_grupo) = signal(String::new());
    let (senha, set_senha) = signal(String::new());
    let (loading, set_loading) = signal(false);

    let nav_permission = navigate.clone();
    let auth_loading = auth.loading;
    let auth_user = auth.user;
    Effect::new(move |_| {
        if auth_loading.get() {
            return;
        }

        if !is_view && !is_admin(auth_user.get().as_ref()) {
            show_snackbar(
                "Acesso negado: apenas administradores podem cadastrar ou editar funcionarios.",
                "warning",
            );
            nav_permission("/home", Default::default());
        }
    });

    Effect::new(move |_| {
        let Some(id) = route_id else {
            return;
        };

        leptos::task::spawn_local(async move {
            set_loading.set(true);

            match funcionario_service::get_by_id(id).await {
                Ok(funcionario) => {
                    set_nome.set(funcionario.nome);
                    set_matricula.set(funcionario.matricula);
                    set_cpf.set(apply_cpf_mask(&funcionario.cpf));
                    set_telefone.set(apply_phone_mask(&funcionario.telefone));
                    set_grupo.set(funcionario.grupo.to_string());
                    set_senha.set(String::new());
                }
                Err(error) => {
                    show_snackbar(
                        &format!("Erro ao carregar funcionario: {}", error.message),
                        "error",
                    );
                }
            }

            set_loading.set(false);
        });
    });

    let nav_cancel = navigate.clone();
    let on_cancel = move |_| {
        nav_cancel("/funcionarios", Default::default());
    };

    let nav_after_submit = navigate.clone();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        if is_view_mode() {
            return;
        }

        let nome_value = nome.get().trim().to_string();
        let matricula_value = matricula.get().trim().to_string();
        let cpf_value = only_digits(&cpf.get());
        let telefone_value = only_digits(&telefone.get());
        let Some(grupo_value) = parse_grupo(&grupo.get()) else {
            show_snackbar("Grupo deve ser 1, 2 ou 3", "warning");
            return;
        };
        let senha_value = senha.get().trim().to_string();

        if nome_value.is_empty() {
            show_snackbar("Nome do funcionario e obrigatorio", "warning");
            return;
        }

        if matricula_value.is_empty() {
            show_snackbar("Matricula do funcionario e obrigatoria", "warning");
            return;
        }

        if matricula_value.chars().count() > MAX_MATRICULA_CHARS {
            show_snackbar("Matricula deve ter no maximo 10 caracteres", "warning");
            return;
        }

        if cpf_value.is_empty() {
            show_snackbar("CPF do funcionario e obrigatorio", "warning");
            return;
        }

        if cpf_value.len() != 11 {
            show_snackbar("CPF deve ter 11 digitos", "warning");
            return;
        }

        if telefone_value.is_empty() {
            show_snackbar("Telefone do funcionario e obrigatorio", "warning");
            return;
        }

        if !(10..=11).contains(&telefone_value.len()) {
            show_snackbar("Telefone deve ter 10 ou 11 digitos", "warning");
            return;
        }

        if route_id.is_none() && senha_value.is_empty() {
            show_snackbar("Senha do funcionario e obrigatoria", "warning");
            return;
        }

        let id = route_id;
        let navigate = nav_after_submit.clone();

        leptos::task::spawn_local(async move {
            set_loading.set(true);

            let result = match id {
                Some(id) => {
                    let payload = FuncionarioUpdatePayload {
                        nome: Some(nome_value),
                        matricula: Some(matricula_value),
                        cpf: Some(cpf_value),
                        telefone: Some(telefone_value),
                        grupo: Some(grupo_value),
                        senha: if senha_value.is_empty() {
                            None
                        } else {
                            Some(senha_value)
                        },
                    };

                    funcionario_service::update(id, &payload).await
                }
                None => {
                    let payload = FuncionarioCreatePayload {
                        nome: nome_value,
                        matricula: matricula_value,
                        cpf: cpf_value,
                        telefone: telefone_value,
                        grupo: grupo_value,
                        senha: senha_value,
                    };

                    funcionario_service::create(&payload).await
                }
            };

            match result {
                Ok(_) => {
                    show_snackbar("Funcionario salvo com sucesso!", "success");
                    navigate("/funcionarios", Default::default());
                }
                Err(error) => {
                    show_snackbar(
                        &format!("Erro ao salvar funcionario: {}", error.message),
                        "error",
                    );
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
                        <label class="block text-sm font-semibold text-slate-700">"Nome do funcionario"</label>
                        <input
                            class=input_class
                            type="text"
                            prop:value=move || nome.get()
                            placeholder="Digite o nome do funcionario"
                            disabled=field_disabled
                            on:input=move |event| set_nome.set(event_target_value(&event))
                        />
                    </div>

                    <div class="min-w-0 space-y-2">
                        <label class="block text-sm font-semibold text-slate-700">"Matricula"</label>
                        <input
                            class=input_class
                            type="text"
                            maxlength="10"
                            prop:value=move || matricula.get()
                            placeholder="Digite a matricula do funcionario"
                            disabled=field_disabled
                            on:input=move |event| set_matricula.set(limit_matricula(&event_target_value(&event)))
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

                    <div class="min-w-0 space-y-2">
                        <label class="block text-sm font-semibold text-slate-700">"Grupo"</label>
                        <select
                            class=input_class
                            prop:value=move || grupo.get()
                            disabled=field_disabled
                            on:change=move |event| set_grupo.set(event_target_value(&event))
                        >
                            <option value="">"Selecione"</option>
                            <option value="1">"1 - Admin"</option>
                            <option value="2">"2 - Balcao"</option>
                            <option value="3">"3 - Caixa"</option>
                        </select>
                    </div>

                    <Show when=move || !is_view_mode()>
                        <div class="min-w-0 space-y-2">
                            <label class="block text-sm font-semibold text-slate-700">
                                {move || if is_create_mode() { "Senha" } else { "Nova senha" }}
                            </label>
                            <input
                                class=input_class
                                type="password"
                                prop:value=move || senha.get()
                                placeholder=move || {
                                    if is_create_mode() {
                                        "Digite a senha do funcionario"
                                    } else {
                                        "Deixe em branco para manter"
                                    }
                                }
                                disabled=move || loading.get()
                                on:input=move |event| set_senha.set(event_target_value(&event))
                            />
                        </div>
                    </Show>
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
