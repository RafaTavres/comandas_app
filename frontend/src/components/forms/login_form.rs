use leptos::prelude::*;
use leptos::ev;
use leptos_icons::Icon; 

use crate::context::auth::use_auth;
use crate::hooks::masks::apply_cpf_mask;
use crate::hooks::validation::{validar_cpf, validar_senha};

#[component]
pub fn LoginForm() -> impl IntoView {
    let auth_context = use_auth();
    let login_action = auth_context.login;

    let (cpf, set_cpf) = signal(String::new());
    let (senha, set_senha) = signal(String::new());
    let (show_password, set_show_password) = signal(false);

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

    view! {
        <div class="min-h-screen bg-slate-200 flex flex-col">
            
            <header class="bg-sky-300 border-b border-sky-400 p-4 shadow-sm">
                <div class="max-w-7xl mx-auto flex items-center gap-3">
                    <div class="text-sky-900 text-2xl">
                         <Icon icon=icondata::FaNoteStickySolid />
                    </div>
                    <h1 class="text-xl font-bold text-sky-900">"E-Comandas"</h1>
                </div>
            </header>

            <main class="flex-grow flex items-center justify-center p-6">
                <div class="bg-white rounded-2xl shadow-xl w-full max-w-md overflow-hidden border border-slate-300">
                    
                    <div class="bg-slate-50 p-8 text-center border-b border-slate-100">
                        <div class="mx-auto w-16 h-16 bg-sky-400 rounded-full flex items-center justify-center mb-4 shadow-md">
                             <Icon icon=icondata::FaUserAstronautSolid/>
                        </div>
                        <h2 class="text-2xl font-bold text-slate-700">"Login"</h2>
                        <p class="text-slate-500 text-sm">"Entre com suas credenciais"</p>
                    </div>

                    <form on:submit=on_submit class="p-8 space-y-5">
                        <div class="space-y-1">
                            <label class="text-xs font-semibold text-slate-400 uppercase tracking-wider">"Usuário"</label>
                            <input
                                type="text"
                                placeholder="CPF"
                                inputmode="numeric"
                                class="w-full px-4 py-3 border border-slate-300 rounded-lg bg-white focus:outline-none focus:ring-2 focus:ring-sky-400 focus:bg-white transition-all"
                                prop:value=cpf
                                on:input=move |ev| set_cpf.set(apply_cpf_mask(&event_target_value(&ev)))
                            />
                            <Show when=move || cpf_error.get().is_some()>
                                <p class="text-red-500 text-xs mt-1">{move || cpf_error.get()}</p>
                            </Show>
                        </div>

                        <div class="space-y-1 relative">
                            <label class="text-xs font-semibold text-slate-400 uppercase tracking-wider">"Senha"</label>
                            <input
                                type=move || if show_password.get() { "text" } else { "password" }
                                placeholder="••••••••"
                                class="password-input w-full px-4 py-3 border border-slate-300 rounded-lg bg-white focus:outline-none focus:ring-2 focus:ring-sky-400 focus:bg-white transition-all pr-10"
                                prop:value=senha
                                on:input=move |ev| set_senha.set(event_target_value(&ev))
                            />
                            
                            <button
                                type="button"
                                class="absolute right-3 top-9 flex h-6 w-6 items-center justify-center text-slate-400 hover:text-sky-500 transition-colors"
                                title=move || if show_password.get() { "Ocultar senha" } else { "Mostrar senha" }
                                aria-label=move || if show_password.get() { "Ocultar senha" } else { "Mostrar senha" }
                                on:click=move |_| set_show_password.update(|v| *v = !*v)
                            >
                                <Show
                                    when=move || show_password.get()
                                    fallback=move || view! {
                                        <Icon icon=icondata::FaEyeSolid width="1.1em" height="1.1em" />
                                    }
                                >
                                    <Icon icon=icondata::FaEyeSlashSolid width="1.1em" height="1.1em" />
                                </Show>
                            </button>

                             <Show when=move || senha_error.get().is_some()>
                                <p class="text-red-500 text-xs mt-1">{move || senha_error.get()}</p>
                            </Show>
                        </div>

                        <button
                            type="submit"
                            class="w-full bg-sky-500 text-white font-bold py-3 rounded-lg hover:bg-sky-600 transition-all duration-300 mt-2 shadow-md"
                        >
                            "Entrar"
                        </button>
                    </form>
                </div>
            </main>
        </div>
    }
}
