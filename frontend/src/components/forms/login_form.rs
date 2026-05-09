// frontend/src/components/forms/login_form.rs

use leptos::prelude::*;
use leptos::ev;
use crate::context::auth::use_auth;
use crate::shared::validation::{validar_cpf, validar_senha};

#[component]
pub fn LoginForm() -> impl IntoView {
    // --- Toda a lógica de antes permanece exatamente a mesma ---
    let auth_context = use_auth();
    let login_action = auth_context.login;

    let (cpf, set_cpf) = signal(String::new());
    let (senha, set_senha) = signal(String::new());

    let (cpf_error, set_cpf_error) = signal(None::<String>);
    let (senha_error, set_senha_error) = signal(None::<String>);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        set_cpf_error.set(None);
        set_senha_error.set(None);

        let is_cpf_valid = match validar_cpf(&cpf.get()) {
            Ok(_) => true,
            Err(msg) => { set_cpf_error.set(Some(msg.to_string())); false }
        };

        let is_senha_valid = match validar_senha(&senha.get()) {
            Ok(_) => true,
            Err(msg) => { set_senha_error.set(Some(msg.to_string())); false }
        };

        if is_cpf_valid && is_senha_valid {
            login_action(cpf.get(), senha.get());
        }
    };

    // --- A renderização agora inclui o layout da página ---
    view! {
        // Esta tag <main> cria o fundo da página e centraliza o conteúdo
        <main class="min-h-screen bg-alabaster-500 flex items-center justify-center p-4">
            // Este <div> é o "card" do formulário, exatamente como era antes
            <div class="bg-white p-8 sm:p-10 rounded-2xl shadow-xl border-t-4 border-amber-500 w-full max-w-md">
                <div class="mx-auto w-16 h-16 bg-deepblue-500 rounded-full flex items-center justify-center mb-4">
                    <span class="text-white text-2xl">"🔒"</span>
                </div>
                <h2 class="text-2xl font-bold text-center text-deepblue-500">"Bem-vindo"</h2>
                <p class="text-gray-500 text-center mb-6">"Faça login para acessar o sistema"</p>

                <form on:submit=on_submit class="flex flex-col gap-5">
                    <div>
                        <input
                            type="text"
                            placeholder="CPF"
                            class="w-full px-4 py-3 border rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-500"
                            prop:value=cpf
                            on:input=move |ev| set_cpf.set(event_target_value(&ev))
                            aria-invalid=move || cpf_error.get().is_some()
                        />
                        <Show when=move || cpf_error.get().is_some()>
                            <p class="text-red-500 text-sm mt-1">{move || cpf_error.get()}</p>
                        </Show>
                    </div>

                    <div>
                        <input
                            type="password"
                            placeholder="Senha"
                            class="w-full px-4 py-3 border rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-500"
                            prop:value=senha
                            on:input=move |ev| set_senha.set(event_target_value(&ev))
                            aria-invalid=move || senha_error.get().is_some()
                        />
                         <Show when=move || senha_error.get().is_some()>
                            <p class="text-red-500 text-sm mt-1">{move || senha_error.get()}</p>
                        </Show>
                    </div>

                    <button
                        type="submit"
                        class="w-full bg-deepblue-500 text-white font-bold py-3 rounded-lg hover:bg-amber-500 transition-colors duration-300"
                    >
                        "Entrar"
                    </button>
                </form>
            </div>
        </main>
    }
}