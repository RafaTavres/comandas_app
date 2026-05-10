use leptos::prelude::*;
use web_sys::window;
use crate::components::common::{ActionButtons, PageLayout};

#[derive(Clone)]
struct Funcionario {
    id: u32,
    nome: String,
    cpf: String,
    telefone: f64,
    grupo: i32,
}

#[component]
pub fn FuncionarioList() -> impl IntoView {
    let funcionarios = vec![
        Funcionario { id: 1, nome: "João Silva".to_string(), cpf: "123.456.789-00".to_string(), telefone: 11999999999.0, grupo: 1 },
        Funcionario { id: 2, nome: "Maria Oliveira".to_string(), cpf: "987.654.321-00".to_string(), telefone: 11888888888.0, grupo: 2 },
        Funcionario { id: 3, nome: "Carlos Souza".to_string(), cpf: "456.789.123-00".to_string(), telefone: 11777777777.0, grupo: 3 },
    ];

    let on_view = std::sync::Arc::new(move |funcionario: Funcionario| {
        if let Some(win) = window() {
            let _ = win.alert_with_message(&format!("Visualizar funcionário: {}", funcionario.nome));
        }
    });

    let on_edit = std::sync::Arc::new(move |funcionario: Funcionario| {
        if let Some(win) = window() {
            let _ = win.alert_with_message(&format!("Editar funcionário: {}", funcionario.nome));
        }
    });

    let on_delete = std::sync::Arc::new(move |funcionario: Funcionario| {
        if let Some(win) = window() {
            let _ = win.alert_with_message(&format!("Excluir funcionário: {}", funcionario.nome));
        }
    });

    view! {
        <PageLayout title="Funcionários".to_string() max_width="7xl".to_string()>
            <div class="mb-6 flex">
                <div class="flex w-full justify-stretch sm:justify-end">
                    <a 
                        href="/funcionarios/novo" 
                        class="w-full rounded-xl bg-amber-500 px-6 py-3 text-center font-semibold text-white hover:bg-amber-600 transition sm:w-auto"
                    >
                        "+ Novo Funcionário"
                    </a>
                </div>
            </div>
            <div class="space-y-6">
                <div class="hidden md:block overflow-hidden rounded-3xl border border-slate-200 bg-white shadow-sm">
                    <table class="min-w-full divide-y divide-slate-200">
                        <thead class="bg-slate-50">
                            <tr>
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"ID"</th>
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Nome"</th>
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"CPF"</th>
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Telefone"</th>
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Grupo"</th>
                                <th class="px-6 py-4 text-right text-xs font-semibold uppercase tracking-wider text-slate-500">"Ações"</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-slate-200 bg-white">
                            {funcionarios.clone().into_iter().map(|funcionario| {
                                let telefone = format!("({})", funcionario.telefone);
                                view! {
                                    <tr>
                                        <td class="px-6 py-4 text-slate-700">{funcionario.id}</td>
                                        <td class="px-6 py-4 text-slate-700 font-semibold">{funcionario.nome.clone()}</td>
                                        <td class="px-6 py-4 text-slate-700">{funcionario.cpf.clone()}</td>
                                        <td class="px-6 py-4 text-slate-700">{telefone}</td>
                                        <td class="px-6 py-4 text-slate-700">{funcionario.grupo.clone()}</td>
                                        <td class="px-6 py-4 text-right">
                                            <ActionButtons
                                                item=funcionario
                                                on_view=on_view.clone()
                                                on_edit=on_edit.clone()
                                                on_delete=on_delete.clone()
                                            />
                                        </td>
                                    </tr>
                                }
                            }).collect::<Vec<_>>() }
                        </tbody>
                    </table>
                </div>

                <div class="md:hidden space-y-4">
                    {funcionarios.into_iter().map(|funcionario| {
                        let telefone = format!("({})", funcionario.telefone);
                        view! {
                            <div class="rounded-2xl border border-slate-200 bg-white p-4 shadow-sm">
                                <div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between sm:gap-4">
                                    <div class="min-w-0">
                                        <p class="break-words text-lg font-semibold text-slate-900">{funcionario.nome.clone()}</p>
                                        <p class="text-sm text-slate-500">{"ID: "}{funcionario.id}</p>
                                    </div>
                                    <div class="min-w-0 sm:text-right">
                                        <p class="break-words text-sm font-semibold text-success-600">{funcionario.cpf.clone()}</p>
                                    </div>
                                </div>
                                <div class="mt-4 space-y-3">
                                    <p class="text-sm text-slate-600">{telefone}</p>
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
                        }
                    }).collect::<Vec<_>>() }
                </div>
            </div>
        </PageLayout>
    }
}
