use leptos::prelude::*;
use web_sys::window;
use crate::components::common::{ActionButtons, PageLayout};

#[derive(Clone)]
struct Produto {
    id: u32,
    nome: String,
    descricao: String,
    valor_unitario: f64,
}

#[component]
pub fn ProdutoList() -> impl IntoView {
    let produtos = vec![
        Produto { id: 1, nome: "Hambúrguer Clássico".to_string(), descricao: "Pão, carne, alface, tomate e queijo".to_string(), valor_unitario: 25.90 },
        Produto { id: 2, nome: "Batata Frita".to_string(), descricao: "Porção média de batata crocante".to_string(), valor_unitario: 12.50 },
        Produto { id: 3, nome: "Refrigerante".to_string(), descricao: "Lata 350ml".to_string(), valor_unitario: 8.00 },
    ];

    let on_view = std::sync::Arc::new(move |produto: Produto| {
        if let Some(win) = window() {
            let _ = win.alert_with_message(&format!("Visualizar produto: {}", produto.nome));
        }
    });

    let on_edit = std::sync::Arc::new(move |produto: Produto| {
        if let Some(win) = window() {
            let _ = win.alert_with_message(&format!("Editar produto: {}", produto.nome));
        }
    });

    let on_delete = std::sync::Arc::new(move |produto: Produto| {
        if let Some(win) = window() {
            let _ = win.alert_with_message(&format!("Excluir produto: {}", produto.nome));
        }
    });

    let format_currency = |value: f64| format!("R$ {:.2}", value);

    view! {
        <PageLayout title="Produtos".to_string() max_width="7xl".to_string()>
         <div class="mb-6 flex justify-end">
                <div class="mb-6 flex justify-end">
                    <a 
                        href="/produtos/novo" 
                        class="rounded-xl bg-amber-500 px-6 py-3 font-semibold text-white hover:bg-amber-600 transition"
                    >
                        "+ Novo Produto"
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
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Descrição"</th>
                                <th class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-slate-500">"Valor Unitário"</th>
                                <th class="px-6 py-4 text-right text-xs font-semibold uppercase tracking-wider text-slate-500">"Ações"</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-slate-200 bg-white">
                            {produtos.clone().into_iter().map(|produto| {
                                let preco = format_currency(produto.valor_unitario);
                                view! {
                                    <tr>
                                        <td class="px-6 py-4 text-slate-700">{produto.id}</td>
                                        <td class="px-6 py-4 text-slate-700 font-semibold">{produto.nome.clone()}</td>
                                        <td class="px-6 py-4 text-slate-700 max-w-xs truncate">{produto.descricao.clone()}</td>
                                        <td class="px-6 py-4 text-slate-700 text-success-600">{preco}</td>
                                        <td class="px-6 py-4 text-right">
                                            <ActionButtons
                                                item=produto
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
                    {produtos.into_iter().map(|produto| {
                        let preco = format_currency(produto.valor_unitario);
                        view! {
                            <div class="rounded-3xl border border-slate-200 bg-white p-4 shadow-sm">
                                <div class="flex items-center justify-between gap-4">
                                    <div>
                                        <p class="text-lg font-semibold text-slate-900">{produto.nome.clone()}</p>
                                        <p class="text-sm text-slate-500">{"ID: "}{produto.id}</p>
                                    </div>
                                    <div class="text-right">
                                        <p class="text-sm font-semibold text-success-600">{preco}</p>
                                    </div>
                                </div>
                                <div class="mt-4 space-y-3">
                                    <p class="text-sm text-slate-600">{produto.descricao.clone()}</p>
                                    <div class="flex justify-end">
                                        <ActionButtons
                                            item=produto
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
