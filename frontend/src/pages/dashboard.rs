use leptos::prelude::*;
use crate::components::common::PageLayout;

#[component]
pub fn Dashboard() -> impl IntoView {
    let formatted_date = "Hoje".to_string();

    view! {
        <PageLayout title="Dashboard".to_string() max_width="xl".to_string()>
            <div class="space-y-6">
                <div class="rounded-3xl border border-slate-200 bg-slate-50 p-6 shadow-sm">
                    <h2 class="text-2xl font-semibold text-slate-900">"Bem-vindo ao Comandas do Zé"</h2>
                    <p class="mt-3 text-slate-600">"Aqui você acessa um resumo rápido do sistema."</p>
                    <p class="mt-4 text-sm text-slate-500">{formatted_date}</p>
                </div>
                <div class="grid gap-4 sm:grid-cols-2">
                    <div class="rounded-3xl border border-slate-200 bg-white p-6 shadow-sm">
                        <h3 class="text-lg font-semibold text-slate-900">"Produtos"</h3>
                        <p class="mt-2 text-slate-600">"Gerencie produtos, preços e estoque."</p>
                    </div>
                    <div class="rounded-3xl border border-slate-200 bg-white p-6 shadow-sm">
                        <h3 class="text-lg font-semibold text-slate-900">"Comandas"</h3>
                        <p class="mt-2 text-slate-600">"Acompanhe pedidos e movimentações do caixa."</p>
                    </div>
                </div>
            </div>
        </PageLayout>
    }
}
