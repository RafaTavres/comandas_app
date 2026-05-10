use leptos::prelude::*; 
use leptos::ev::SubmitEvent;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use crate::utils::snackbar::show_snackbar;

use leptos_router::hooks::use_navigate;
use crate::components::common::PageLayout;
use crate::components::forms::controlled_input::ControlledInput;

#[component]
pub fn ProdutoForm() -> impl IntoView {
    let (nome, set_nome) = signal(String::new());
    let (descricao, set_descricao) = signal(String::new());
    let (valor_unitario, set_valor_unitario) = signal(String::new());
    let (foto_selecionada, set_foto_selecionada) = signal(None::<String>);
    let (foto_url, set_foto_url) = signal(None::<String>);

    let navigate = use_navigate();
    let on_cancel = move |_| {
        navigate("/produtos", Default::default());
    };

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        if nome.get().trim().is_empty() {
            show_snackbar("Nome do produto é obrigatório", "warning");
            return;
        }

        if descricao.get().trim().is_empty() {
            show_snackbar("Descrição do produto é obrigatória", "warning");
            return;
        }

        if valor_unitario.get().trim().is_empty() || valor_unitario.get().parse::<f64>().is_err() {
            show_snackbar("Valor unitário inválido", "warning");
            return;
        }

        show_snackbar("Produto salvo com sucesso!", "success");
    };

    let on_file_change = move |ev| {
        let input = event_target::<web_sys::HtmlInputElement>(&ev);
        
        if let Some(files) = input.files() {
            if files.length() > 0 {
                if let Some(file) = files.get(0) {
                    set_foto_selecionada.set(Some(file.name()));
                    if let Ok(reader) = web_sys::FileReader::new() {
                        let set_foto_url = set_foto_url.clone();
                        let reader_for_closure = reader.clone();
                        let onloadend = Closure::once_into_js(move |_: web_sys::ProgressEvent| {
                            if let Ok(result) = reader_for_closure.result() {
                                if let Some(url) = result.as_string() {
                                    set_foto_url.set(Some(url));
                                }
                            }
                        });
                        reader.set_onloadend(Some(onloadend.unchecked_ref()));
                        let _ = reader.read_as_data_url(&file);
                    }
                }
            }
        }
    };

    view! {
        <PageLayout title="Cadastro de Produto".to_string() max_width="lg".to_string()>
            <form on:submit=on_submit class="space-y-8">
                <div class="grid gap-6 sm:grid-cols-2">
                    <ControlledInput
                        label="Nome do produto".to_string()
                        value=nome
                        on_input=std::sync::Arc::new(move |value| set_nome.set(value))
                        placeholder="Digite o nome do produto".to_string()
                    />
                    <ControlledInput
                        label="Valor Unitário".to_string()
                        value=valor_unitario
                        on_input=std::sync::Arc::new(move |value| set_valor_unitario.set(value))
                        placeholder="0.00".to_string()
                        input_type="number".to_string()
                    />
                </div>

                <div class="space-y-2">
                    <label class="block text-sm font-semibold text-slate-700">"Descrição"</label>
                    <textarea
                        class="w-full min-h-[120px] rounded-xl border border-slate-200 px-4 py-3 text-sm text-slate-700 focus:border-amber-500 focus:outline-none focus:ring-2 focus:ring-amber-200"
                        prop:value=move || descricao.get()
                        placeholder="Digite a descrição do produto"
                        on:input=move |ev| set_descricao.set(event_target_value(&ev))
                    />
                </div>

                <div class="space-y-2">
                    <label class="block text-sm font-semibold text-slate-700">"Foto do Produto"</label>
                    <input
                        id="foto-upload"
                        type="file"
                        accept="image/*"
                        class="hidden"
                        on:change=on_file_change
                    />
                    <label for="foto-upload" class="inline-flex w-full cursor-pointer items-center justify-center gap-2 rounded-xl border border-slate-200 bg-slate-50 px-4 py-3 text-sm font-medium text-slate-700 hover:bg-slate-100 transition sm:w-auto">
                        "Selecionar Foto"
                    </label>
                    <p class="break-words text-sm text-slate-500">
                        {move || foto_selecionada.get().clone().unwrap_or_else(|| "Nenhuma foto selecionada".to_string())}
                    </p>
                </div>

                <Show when=move || foto_url.get().is_some()>
                    <div class="mt-4 max-w-sm rounded-2xl border border-slate-200 bg-slate-50 p-3 shadow-sm">
                        <img
                            class="aspect-video w-full rounded-xl object-cover"
                            src={move || foto_url.get().clone().unwrap_or_default()}
                            alt="Prévia da foto do produto"
                        />
                    </div>
                </Show>

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
