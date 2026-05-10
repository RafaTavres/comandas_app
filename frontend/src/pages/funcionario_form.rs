use leptos::prelude::*; 
use leptos::ev::SubmitEvent;

use leptos_router::hooks::use_navigate;
use crate::components::common::PageLayout;
use crate::components::forms::controlled_input::ControlledInput;
use crate::hooks::masks::{apply_cpf_mask, apply_phone_mask};
use crate::utils::snackbar::show_snackbar;

#[component]
pub fn FuncionarioForm() -> impl IntoView {
    let (nome, set_nome) = signal(String::new());
    let (matricula, set_matricula) = signal(String::new());
    let (cpf, set_cpf) = signal(String::new());
    let (telefone, set_telefone) = signal(String::new());
    let (grupo, set_grupo) = signal(String::new());
    let (senha, set_senha) = signal(String::new());

    let navigate = use_navigate();
    let on_cancel = move |_| {
        navigate("/funcionarios", Default::default());
    };

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        if nome.get().trim().is_empty() {
            show_snackbar("Nome do funcionário é obrigatório", "warning");
            return;
        }

        if cpf.get().trim().is_empty() {
            show_snackbar("CPF do funcionário é obrigatório", "warning");
            return;
        }

        if matricula.get().trim().is_empty() {
            show_snackbar("Matrícula do funcionário é obrigatória", "warning");
            return;
        }

        show_snackbar("Funcionário salvo com sucesso!", "success");
    };

    view! {
        <PageLayout title="Cadastro de Funcionário".to_string() max_width="lg".to_string()>
            <form on:submit=on_submit class="space-y-8">
                <div class="grid gap-6 sm:grid-cols-2">
                    <ControlledInput
                        label="Nome do funcionário".to_string()
                        value=nome
                        on_input=std::sync::Arc::new(move |value| set_nome.set(value))
                        placeholder="Digite o nome do funcionário".to_string()
                    />
                    <ControlledInput
                        label="Matrícula".to_string()
                        value=matricula
                        on_input=std::sync::Arc::new(move |value| set_matricula.set(value))
                        placeholder="Digite a matrícula do funcionário".to_string()
                    />
                    <ControlledInput
                        label="CPF".to_string()
                        value=cpf
                        on_input=std::sync::Arc::new(move |value| set_cpf.set(apply_cpf_mask(&value)))
                        input_mode="numeric".to_string()
                        placeholder="Digite o CPF do funcionário".to_string()
                    />
                    <ControlledInput
                        label="Telefone".to_string()
                        value=telefone
                        on_input=std::sync::Arc::new(move |value| set_telefone.set(apply_phone_mask(&value)))
                        input_type="tel".to_string()
                        input_mode="numeric".to_string()
                        placeholder="Digite o telefone do funcionário".to_string()
                    />
                    <ControlledInput
                        label="Grupo".to_string()
                        value=grupo
                        on_input=std::sync::Arc::new(move |value| set_grupo.set(value))
                        placeholder="Selecione o grupo do funcionário".to_string()
                    />
                    <ControlledInput
                        label="Senha".to_string()
                        value=senha
                        on_input=std::sync::Arc::new(move |value| set_senha.set(value))
                        input_type="password".to_string()
                        placeholder="Digite a senha do funcionário".to_string()
                    />
                </div>

                <div class="flex flex-col gap-3 sm:flex-row sm:justify-end">
                    <button
                        type="button"
                        on:click=on_cancel
                        class="rounded-xl border border-slate-200 bg-white px-6 py-3 font-semibold text-slate-700 hover:border-slate-300 transition"
                    >
                        "Cancelar"
                    </button>
                    <button
                        type="submit"
                        class="rounded-xl bg-amber-500 px-6 py-3 font-semibold text-white hover:bg-amber-600 transition"
                    >
                        "Cadastrar"
                    </button>
                </div>

            </form>
        </PageLayout>
    }
}
