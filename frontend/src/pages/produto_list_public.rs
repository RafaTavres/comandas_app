use leptos::prelude::*;

use crate::{
    components::common::{PageLayout, Pagination, ProdutoFilters, ProdutoFiltersState},
    services::produto_service::{self, foto_to_src, Produto, ProdutoListParams},
    utils::snackbar::show_snackbar,
};

const MAX_API_LIMIT: usize = 1000;

fn list_params(page: usize, limit: usize, filters: ProdutoFiltersState) -> ProdutoListParams {
    let limit = limit.max(1);

    ProdutoListParams {
        skip: page.saturating_sub(1) * limit,
        limit: limit.saturating_add(1).min(MAX_API_LIMIT),
        id: filters.id,
        nome: filters.nome,
        descricao: filters.descricao,
        valor: filters.valor,
        valor_min: filters.valor_min,
        valor_max: filters.valor_max,
    }
}

#[component]
pub fn ProdutoListPublic() -> impl IntoView {
    let (produtos, set_produtos) = signal(Vec::<Produto>::new());
    let (loading, set_loading) = signal(false);
    let (filters, set_filters) = signal(ProdutoFiltersState::default());
    let (current_page, set_current_page) = signal(1usize);
    let (items_per_page, set_items_per_page) = signal(6usize);
    let (has_next_page, set_has_next_page) = signal(false);
    let (request_version, set_request_version) = signal(0usize);

    Effect::new(move |_| {
        let page = current_page.get();
        let per_page = items_per_page.get().max(1);
        let params = list_params(page, per_page, filters.get());
        let next_request_version = request_version.get_untracked().wrapping_add(1);

        set_request_version.set(next_request_version);
        set_loading.set(true);

        leptos::task::spawn_local(async move {
            match produto_service::list_public(params).await {
                Ok(mut items) => {
                    if request_version.get_untracked() != next_request_version {
                        return;
                    }

                    let has_more_items = items.len() > per_page;
                    items.truncate(per_page);

                    if items.is_empty() && page > 1 {
                        set_has_next_page.set(false);
                        set_current_page.set(page - 1);
                        return;
                    }

                    set_has_next_page.set(has_more_items);
                    set_produtos.set(items);
                }
                Err(error) => {
                    if request_version.get_untracked() != next_request_version {
                        return;
                    }

                    set_has_next_page.set(false);
                    set_produtos.set(Vec::new());
                    show_snackbar(
                        &format!("Erro ao carregar produtos publicos: {}", error.message),
                        "error",
                    );
                }
            }

            set_loading.set(false);
        });
    });

    let on_filter = std::sync::Arc::new(move |next_filters| {
        set_filters.set(next_filters);
        set_current_page.set(1);
    });

    let on_clear_filters = std::sync::Arc::new(move || {
        set_filters.set(ProdutoFiltersState::default());
        set_current_page.set(1);
    });

    let on_page_change = std::sync::Arc::new(move |page| {
        set_current_page.set(page);
    });

    let on_items_per_page_change = std::sync::Arc::new(move |value| {
        set_items_per_page.set(value);
        set_current_page.set(1);
    });

    let loading_for_filters = std::sync::Arc::new(move || loading.get());
    let loading_for_pagination = loading_for_filters.clone();
    let has_next_page_for_pagination = std::sync::Arc::new(move || has_next_page.get());

    let format_currency = |value: f64| format!("R$ {:.2}", value);

    view! {
        <PageLayout title="Produtos Publicos".to_string() max_width="7xl".to_string()>
            <div class="space-y-6">
                <ProdutoFilters
                    on_filter=on_filter
                    on_clear=on_clear_filters
                    loading=loading_for_filters
                />

                {move || {
                    if loading.get() {
                        return view! {
                            <div class="rounded-2xl border border-slate-200 bg-slate-50 p-8 text-center text-sm font-medium text-slate-500">
                                "Carregando produtos..."
                            </div>
                        }.into_any();
                    }

                    if produtos.get().is_empty() {
                        return view! {
                            <div class="rounded-2xl border border-slate-200 bg-slate-50 p-8 text-center text-sm font-medium text-slate-500">
                                "Nenhum produto encontrado."
                            </div>
                        }.into_any();
                    }

                    view! {
                        <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                            {produtos.get().into_iter().map(|produto| {
                                let preco = format_currency(produto.valor_unitario);
                                let foto_view = match foto_to_src(produto.foto.as_deref()) {
                                    Some(foto) => view! {
                                        <img
                                            class="h-full w-full object-cover"
                                            src=foto
                                            alt={format!("Foto do produto {}", produto.nome)}
                                        />
                                    }.into_any(),
                                    None => view! {
                                        <span class="text-sm font-medium text-slate-500">"Sem foto"</span>
                                    }.into_any(),
                                };

                                view! {
                                    <article class="flex min-h-full flex-col overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-sm transition hover:-translate-y-0.5 hover:shadow-lg">
                                        <div class="flex aspect-[4/3] w-full items-center justify-center bg-slate-100">
                                            {foto_view}
                                        </div>
                                        <div class="flex flex-1 flex-col gap-3 p-4">
                                            <div class="space-y-2">
                                                <h2 class="break-words text-lg font-semibold text-slate-900">{produto.nome.clone()}</h2>
                                                <p class="line-clamp-3 min-h-16 break-words text-sm leading-6 text-slate-600">
                                                    {produto.descricao.clone()}
                                                </p>
                                            </div>
                                            <div class="mt-auto flex items-center justify-between gap-3 border-t border-slate-100 pt-3">
                                                <span class="text-xs font-semibold uppercase text-slate-500">"Valor"</span>
                                                <span class="text-lg font-bold text-emerald-700">{preco}</span>
                                            </div>
                                        </div>
                                    </article>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_any()
                }}

                <Pagination
                    current_page=current_page
                    items_per_page=items_per_page
                    on_page_change=on_page_change
                    on_items_per_page_change=on_items_per_page_change
                    loading=loading_for_pagination
                    has_next_page=has_next_page_for_pagination
                />
            </div>
        </PageLayout>
    }
}
