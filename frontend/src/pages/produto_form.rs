use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params_map};
use wasm_bindgen::JsValue;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

use crate::{
    components::common::PageLayout,
    context::auth::use_auth,
    services::produto_service::{self, foto_to_src, ProdutoPayload},
    utils::{
        snackbar::show_snackbar,
        user_groups::is_admin,
    },
};

fn parse_decimal(value: &str) -> Option<f64> {
    value.trim().replace(',', ".").parse::<f64>().ok()
}

const MAX_FOTO_BASE64_CHARS: usize = 55_000;
const IMAGE_COMPRESSION_ATTEMPTS: &[(f64, f64)] = &[
    (512.0, 0.72),
    (384.0, 0.66),
    (320.0, 0.60),
    (256.0, 0.55),
    (192.0, 0.48),
    (160.0, 0.42),
    (128.0, 0.36),
];

fn base64_payload_len(data_url_or_base64: &str) -> usize {
    data_url_or_base64
        .split_once("base64,")
        .map(|(_, base64)| base64.len())
        .unwrap_or_else(|| data_url_or_base64.len())
}

fn foto_payload_value(preview_or_url: Option<String>) -> String {
    let Some(value) = preview_or_url else {
        return String::new();
    };

    if let Some((_, base64)) = value.split_once("base64,") {
        return base64.to_string();
    }

    value
}

fn scaled_dimensions(width: u32, height: u32, max_side: f64) -> (u32, u32) {
    let width = width.max(1) as f64;
    let height = height.max(1) as f64;
    let scale = (max_side / width.max(height)).min(1.0);

    (
        (width * scale).round().max(1.0) as u32,
        (height * scale).round().max(1.0) as u32,
    )
}

fn compress_image_for_blob(data_url: String, set_foto_url: WriteSignal<Option<String>>) {
    let Ok(image) = HtmlImageElement::new() else {
        show_snackbar("Não foi possível processar a imagem.", "error");
        return;
    };

    let image_for_load = image.clone();
    let onload = Closure::wrap(Box::new(move || {
        let width = image_for_load.natural_width();
        let height = image_for_load.natural_height();

        if width == 0 || height == 0 {
            show_snackbar("Não foi possível ler as dimensões da imagem.", "error");
            return;
        }

        let Some(document) = web_sys::window().and_then(|window| window.document()) else {
            show_snackbar("Não foi possível acessar o documento da página.", "error");
            return;
        };

        let Ok(canvas) = document
            .create_element("canvas")
            .and_then(|element| element.dyn_into::<HtmlCanvasElement>().map_err(Into::into))
        else {
            show_snackbar("Não foi possível preparar a imagem para envio.", "error");
            return;
        };

        let Ok(Some(context)) = canvas.get_context("2d") else {
            show_snackbar("Não foi possível preparar a imagem para envio.", "error");
            return;
        };

        let Ok(context) = context.dyn_into::<CanvasRenderingContext2d>() else {
            show_snackbar("Não foi possível preparar a imagem para envio.", "error");
            return;
        };

        for (max_side, quality) in IMAGE_COMPRESSION_ATTEMPTS {
            let (target_width, target_height) = scaled_dimensions(width, height, *max_side);

            canvas.set_width(target_width);
            canvas.set_height(target_height);
            context.clear_rect(0.0, 0.0, target_width as f64, target_height as f64);

            if context
                .draw_image_with_html_image_element_and_dw_and_dh(
                    &image_for_load,
                    0.0,
                    0.0,
                    target_width as f64,
                    target_height as f64,
                )
                .is_err()
            {
                continue;
            }

            let Ok(compressed) = canvas.to_data_url_with_type_and_encoder_options(
                "image/jpeg",
                &JsValue::from_f64(*quality),
            ) else {
                continue;
            };

            if base64_payload_len(&compressed) <= MAX_FOTO_BASE64_CHARS {
                set_foto_url.set(Some(compressed));
                return;
            }
        }

        set_foto_url.set(None);
        show_snackbar(
            "A imagem é grande demais para o campo BLOB. Escolha uma imagem menor.",
            "warning",
        );
    }) as Box<dyn FnMut()>);

    let image_element: web_sys::HtmlElement = image.clone().unchecked_into();
    image_element.set_onload(Some(onload.as_ref().unchecked_ref()));
    onload.forget();

    let onerror = Closure::wrap(Box::new(move || {
        show_snackbar("Não foi possível carregar a imagem selecionada.", "error");
    }) as Box<dyn FnMut()>);
    let image_element: web_sys::HtmlElement = image.clone().unchecked_into();
    image_element.set_onerror(Some(onerror.as_ref().unchecked_ref()));
    onerror.forget();

    image.set_src(&data_url);
}

#[component]
pub fn ProdutoForm() -> impl IntoView {
    let params = use_params_map();
    let navigate = use_navigate();
    let auth = use_auth();

    let route_operation = params.with_untracked(|params| params.get("opr"));
    let route_id = params
        .with_untracked(|params| params.get("id"))
        .and_then(|id| id.parse::<u32>().ok());
    let is_view = route_operation.as_deref() == Some("view");
    let is_edit = route_operation.as_deref() == Some("edit");

    let is_view_mode = move || is_view;
    let page_title = move || {
        if is_view {
            "Visualizar Produto".to_string()
        } else if is_edit {
            "Editar Produto".to_string()
        } else {
            "Cadastro de Produto".to_string()
        }
    };

    let (nome, set_nome) = signal(String::new());
    let (descricao, set_descricao) = signal(String::new());
    let (valor_unitario, set_valor_unitario) = signal(String::new());
    let (foto_selecionada, set_foto_selecionada) = signal(None::<String>);
    let (foto_url, set_foto_url) = signal(None::<String>);
    let (loading, set_loading) = signal(false);

    let nav_permission = navigate.clone();
    let auth_loading = auth.loading;
    let auth_user = auth.user;
    Effect::new(move |_| {
        if auth_loading.get() {
            return;
        }

        if !is_view && !is_admin(auth_user.get().as_ref()) {
            show_snackbar(
                "Acesso negado: apenas administradores podem cadastrar ou editar produtos.",
                "warning",
            );
            nav_permission("/produtos", Default::default());
        }
    });

    Effect::new(move |_| {
        let Some(id) = route_id else {
            return;
        };

        leptos::task::spawn_local(async move {
            set_loading.set(true);

            match produto_service::get_by_id(id).await {
                Ok(produto) => {
                    set_nome.set(produto.nome);
                    set_descricao.set(produto.descricao);
                    set_valor_unitario.set(format!("{:.2}", produto.valor_unitario));
                    set_foto_url.set(produto.foto);
                    set_foto_selecionada.set(None);
                }
                Err(error) => {
                    show_snackbar(&format!("Erro ao carregar produto: {}", error.message), "error");
                }
            }

            set_loading.set(false);
        });
    });

    let nav_cancel = navigate.clone();
    let on_cancel = move |_| {
        nav_cancel("/produtos", Default::default());
    };

    let nav_after_submit = navigate.clone();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        if is_view_mode() {
            return;
        }

        let nome_value = nome.get().trim().to_string();
        let descricao_value = descricao.get().trim().to_string();
        let Some(valor_value) = parse_decimal(&valor_unitario.get()) else {
            show_snackbar("Valor unitário inválido", "warning");
            return;
        };

        if nome_value.is_empty() {
            show_snackbar("Nome do produto é obrigatório", "warning");
            return;
        }

        if descricao_value.is_empty() {
            show_snackbar("Descrição do produto é obrigatória", "warning");
            return;
        }

        if valor_value <= 0.0 {
            show_snackbar("Valor unitário deve ser maior que zero", "warning");
            return;
        }

        let foto_value = foto_payload_value(foto_url.get());

        if foto_value.len() > MAX_FOTO_BASE64_CHARS {
            show_snackbar(
                "A imagem é grande demais para o campo BLOB. Escolha uma imagem menor.",
                "warning",
            );
            return;
        }

        let payload = ProdutoPayload {
            nome: nome_value,
            descricao: descricao_value,
            valor_unitario: valor_value,
            foto: foto_value,
        };
        let id = route_id;
        let navigate = nav_after_submit.clone();

        leptos::task::spawn_local(async move {
            set_loading.set(true);

            let result = match id {
                Some(id) => produto_service::update(id, &payload).await,
                None => produto_service::create(&payload).await,
            };

            match result {
                Ok(_) => {
                    show_snackbar("Produto salvo com sucesso!", "success");
                    navigate("/produtos", Default::default());
                }
                Err(error) => {
                    show_snackbar(&format!("Erro ao salvar produto: {}", error.message), "error");
                }
            }

            set_loading.set(false);
        });
    };

    let on_file_change = move |ev| {
        let input = event_target::<web_sys::HtmlInputElement>(&ev);

        if let Some(files) = input.files()
            && files.length() > 0
            && let Some(file) = files.get(0)
        {
            set_foto_selecionada.set(Some(file.name()));

            if let Ok(reader) = web_sys::FileReader::new() {
                let reader_for_closure = reader.clone();
                let onloadend = Closure::once_into_js(move |_: web_sys::ProgressEvent| {
                    if let Ok(result) = reader_for_closure.result()
                        && let Some(url) = result.as_string()
                    {
                        compress_image_for_blob(url, set_foto_url);
                    }
                });

                reader.set_onloadend(Some(onloadend.unchecked_ref()));
                let _ = reader.read_as_data_url(&file);
            }
        }
    };

    let field_disabled = move || loading.get() || is_view_mode();
    let input_class = "w-full min-w-0 rounded-xl border border-slate-200 px-4 py-3 focus:border-amber-500 focus:outline-none focus:ring-2 focus:ring-amber-200 disabled:cursor-not-allowed disabled:bg-slate-100 disabled:text-slate-500";

    view! {
        <PageLayout title=page_title() max_width="lg".to_string()>
            <form on:submit=on_submit class="space-y-8">
                <div class="grid gap-6 sm:grid-cols-2">
                    <div class="min-w-0 space-y-2">
                        <label class="block text-sm font-semibold text-slate-700">"Nome do produto"</label>
                        <input
                            class=input_class
                            type="text"
                            prop:value=move || nome.get()
                            placeholder="Digite o nome do produto"
                            disabled=field_disabled
                            on:input=move |event| set_nome.set(event_target_value(&event))
                        />
                    </div>

                    <div class="min-w-0 space-y-2">
                        <label class="block text-sm font-semibold text-slate-700">"Valor Unitário"</label>
                        <input
                            class=input_class
                            type="number"
                            step="0.01"
                            prop:value=move || valor_unitario.get()
                            placeholder="0.00"
                            disabled=field_disabled
                            on:input=move |event| set_valor_unitario.set(event_target_value(&event))
                        />
                    </div>
                </div>

                <div class="space-y-2">
                    <label class="block text-sm font-semibold text-slate-700">"Descrição"</label>
                    <textarea
                        class="w-full min-h-[120px] rounded-xl border border-slate-200 px-4 py-3 text-sm text-slate-700 focus:border-amber-500 focus:outline-none focus:ring-2 focus:ring-amber-200 disabled:cursor-not-allowed disabled:bg-slate-100 disabled:text-slate-500"
                        prop:value=move || descricao.get()
                        placeholder="Digite a descrição do produto"
                        disabled=field_disabled
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
                        disabled=field_disabled
                        on:change=on_file_change
                    />
                    <label
                        for="foto-upload"
                        class="inline-flex w-full cursor-pointer items-center justify-center gap-2 rounded-xl border border-slate-200 bg-slate-50 px-4 py-3 text-sm font-medium text-slate-700 hover:bg-slate-100 transition sm:w-auto"
                        class=("pointer-events-none", field_disabled)
                        class=("opacity-60", field_disabled)
                    >
                        "Selecionar Foto"
                    </label>
                    <p class="break-words text-sm text-slate-500">
                        {move || foto_selecionada.get().clone().unwrap_or_else(|| {
                            if foto_url.get().is_some() {
                                "Foto carregada".to_string()
                            } else {
                                "Nenhuma foto selecionada".to_string()
                            }
                        })}
                    </p>
                </div>

                <Show when=move || foto_to_src(foto_url.get().as_deref()).is_some()>
                    <div class="mt-4 max-w-sm rounded-2xl border border-slate-200 bg-slate-50 p-3 shadow-sm">
                        <img
                            class="aspect-video w-full rounded-xl object-cover"
                            src={move || foto_to_src(foto_url.get().as_deref()).unwrap_or_default()}
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
                        {move || if is_view_mode() { "Voltar" } else { "Cancelar" }}
                    </button>

                    <Show when=move || !is_view_mode()>
                        <button
                            type="submit"
                            disabled=move || loading.get()
                            class="rounded-xl bg-amber-500 px-6 py-3 font-semibold text-white hover:bg-amber-600 transition disabled:cursor-not-allowed disabled:opacity-60"
                        >
                            {move || if loading.get() { "Salvando..." } else { "Salvar" }}
                        </button>
                    </Show>
                </div>
            </form>
        </PageLayout>
    }
}
