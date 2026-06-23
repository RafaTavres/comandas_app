use std::sync::Arc;

use leptos::prelude::*;

use crate::{
    constants::comanda_status::status_config,
    services::comanda_service::Comanda,
};

#[component]
pub fn ComandaValidator(
    open: ReadSignal<bool>,
    existing_record: ReadSignal<Option<Comanda>>,
    on_close: Arc<dyn Fn() + Send + Sync>,
    #[prop(optional)] on_clear_field: Option<Arc<dyn Fn() + Send + Sync>>,
    #[prop(optional)] record_type: Option<String>,
    #[prop(optional)] on_view: Option<Arc<dyn Fn(Comanda) + Send + Sync>>,
    #[prop(optional)] on_edit: Option<Arc<dyn Fn(Comanda) + Send + Sync>>,
) -> impl IntoView {
    let record_type = record_type.unwrap_or_else(|| "comanda".to_string());

    view! {
        {move || {
            if !open.get() {
                return view! {}.into_any();
            }

            let close_button = {
                let on_close = on_close.clone();
                let on_clear_field = on_clear_field.clone();

                view! {
                    <button
                        type="button"
                        class="rounded-xl border border-slate-200 bg-white px-4 py-2 text-sm font-semibold text-slate-700 transition hover:bg-slate-50"
                        on:click=move |_| {
                            if let Some(clear_field) = &on_clear_field {
                                clear_field();
                            }

                            on_close();
                        }
                    >
                        "Fechar"
                    </button>
                }
            };

            let view_action = on_view.clone().map(|on_view| {
                let on_close = on_close.clone();

                view! {
                    <button
                        type="button"
                        class="rounded-xl bg-slate-900 px-4 py-2 text-sm font-semibold text-white transition hover:bg-slate-800"
                        on:click=move |_| {
                            if let Some(record) = existing_record.get_untracked() {
                                on_view(record);
                            }

                            on_close();
                        }
                    >
                        "Visualizar"
                    </button>
                }
            });

            let edit_action = on_edit.clone().map(|on_edit| {
                let on_close = on_close.clone();

                view! {
                    <button
                        type="button"
                        class="rounded-xl bg-amber-500 px-4 py-2 text-sm font-semibold text-white transition hover:bg-amber-600"
                        on:click=move |_| {
                            if let Some(record) = existing_record.get_untracked() {
                                on_edit(record);
                            }

                            on_close();
                        }
                    >
                        "Editar"
                    </button>
                }
            });

            let content = existing_record.get().map(|record| {
                let status = status_config(record.status);
                let data_hora = record
                    .data_hora
                    .clone()
                    .unwrap_or_else(|| "Nao informada".to_string());
                let cliente = record
                    .cliente_nome
                    .clone()
                    .trim()
                    .to_string();
                let cliente = if cliente.is_empty() {
                    record
                        .cliente_id
                        .map(|id| id.to_string())
                        .unwrap_or_else(|| "Nao informado".to_string())
                } else {
                    cliente
                };

                view! {
                    <div class="rounded-xl bg-slate-100 p-4 text-sm text-slate-700">
                        <p class="font-semibold">"ID: " {record.id}</p>
                        <p class="mt-1 font-semibold">"Comanda: " {record.comanda.clone()}</p>
                        <p class="mt-3">"Abertura: " {data_hora}</p>
                        <p class="mt-1">
                            "Status: "
                            <span class=format!("inline-flex rounded-full border px-2 py-0.5 text-xs font-semibold {}", status.badge_class)>
                                {status.label}
                            </span>
                        </p>
                        <p class="mt-1">"Cliente: " {cliente}</p>
                    </div>
                }.into_any()
            }).unwrap_or_else(|| view! {
                <div class="rounded-xl bg-slate-100 p-4 text-sm text-slate-700">
                    "Registro nao encontrado."
                </div>
            }.into_any());

            view! {
                <div class="fixed inset-0 z-50 flex items-center justify-center bg-slate-950/50 px-4 py-6">
                    <div
                        role="dialog"
                        aria-modal="true"
                        aria-labelledby="comanda-validator-title"
                        class="w-full max-w-lg overflow-hidden rounded-2xl bg-white shadow-2xl"
                    >
                        <header class="bg-red-500 px-5 py-4 text-white">
                            <h2 id="comanda-validator-title" class="text-base font-semibold">
                                "Registro ja existente em " {record_type.clone()}
                            </h2>
                        </header>

                        <div class="p-5">
                            {content}
                        </div>

                        <footer class="flex flex-col gap-3 border-t border-slate-200 px-5 py-4 sm:flex-row sm:justify-end">
                            {close_button}
                            {view_action}
                            {edit_action}
                        </footer>
                    </div>
                </div>
            }.into_any()
        }}
    }
}
