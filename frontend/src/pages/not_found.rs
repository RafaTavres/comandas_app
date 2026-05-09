use leptos::prelude::*;
use crate::components::common::PageLayout;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <PageLayout title="404 - NotFound".to_string() max_width="md".to_string()>
            <div class="rounded-3xl border border-slate-200 bg-slate-50 p-8 text-center shadow-sm">
                <p class="text-lg font-semibold text-slate-900">"Página não encontrada."</p>
                <p class="mt-3 text-slate-500">"Verifique a URL ou retorne à página inicial."</p>
            </div>
        </PageLayout>
    }
}
