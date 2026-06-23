use leptos::prelude::*;
use leptos_icons::Icon;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::CustomEvent;

const SNACKBAR_CONFIRMED_EVENT: &str = "snackbarConfirmed";

#[component]
pub fn SnackbarGlobal() -> impl IntoView {
   
    let (open, set_open) = signal(false);
    let (message, set_message) = signal(String::new());
    let (severity, set_severity) = signal(String::from("success"));
    let (confirm_label, set_confirm_label) = signal(String::new());
    let (cancel_label, set_cancel_label) = signal(String::new());
    let (action_id, set_action_id) = signal(String::new());

    
    Effect::new(move |_| {
        if open.get() && action_id.get().is_empty() {
            let set_open = set_open;
            let closure = Closure::once_into_js(move || {
                set_open.set(false);
            });

            let win = window();
            let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                4000,
            );
        }
    });

   
    Effect::new(move |_| {
        let set_open = set_open;
        let set_message = set_message;
        let set_severity = set_severity;
        let set_confirm_label = set_confirm_label;
        let set_cancel_label = set_cancel_label;
        let set_action_id = set_action_id;

        let listener = Closure::wrap(Box::new(move |event: web_sys::Event| {
            if let Some(custom) = event.dyn_ref::<CustomEvent>() {
                if let Some(detail) = custom.detail().as_string() {
                    let parts = detail.split('\0').collect::<Vec<_>>();

                    set_message.set(parts.first().copied().unwrap_or_default().to_string());
                    set_severity.set(parts.get(1).copied().unwrap_or("success").to_string());
                    set_confirm_label.set(parts.get(2).copied().unwrap_or_default().to_string());
                    set_cancel_label.set(parts.get(3).copied().unwrap_or("Cancelar").to_string());
                    set_action_id.set(parts.get(4).copied().unwrap_or_default().to_string());
                    set_open.set(true);
                }
            }
        }) as Box<dyn FnMut(_)>);

        let win = window();
        let _ = win.add_event_listener_with_callback(
            "showSnackbar",
            listener.as_ref().unchecked_ref(),
        );

        listener.forget();
    });

    let on_confirm = move |_| {
        let current_action_id = action_id.get_untracked();
        set_open.set(false);

        if current_action_id.is_empty() {
            return;
        }

        if let Ok(event) = CustomEvent::new(SNACKBAR_CONFIRMED_EVENT) {
            event.init_custom_event_with_can_bubble_and_cancelable_and_detail(
                SNACKBAR_CONFIRMED_EVENT,
                true,
                false,
                &JsValue::from_str(&current_action_id),
            );

            let _ = window().dispatch_event(&event);
        }
    };

    let on_cancel = move |_| {
        set_open.set(false);
    };

    let severity_class = move || match severity.get().as_str() {
        "error" => "bg-red-50 border-red-200 text-red-900",
        "warning" => "bg-amber-50 border-amber-200 text-amber-900",
        "info" => "bg-sky-50 border-sky-200 text-sky-900",
        _ => "bg-emerald-50 border-emerald-200 text-emerald-900",
    };

    let icon = move || match severity.get().as_str() {
        "error" => view! { <Icon icon=icondata::FaCircleExclamationSolid width="1.2em" height="1.2em" /> },
        "warning" => view! { <Icon icon=icondata::FaCircleExclamationSolid width="1.2em" height="1.2em" /> },
        "info" => view! { <Icon icon=icondata::FaCircleInfoSolid width="1.2em" height="1.2em" /> },
        _ => view! { <Icon icon=icondata::FaCircleCheckSolid width="1.2em" height="1.2em" /> },
    };

    let title = move || match severity.get().as_str() {
        "error" => "Erro",
        "warning" => "Atenção",
        "info" => "Info",
        _ => "Sucesso",
    };

    view! {
        <Show when=move || open.get() fallback=|| view! {}>
            <div class="fixed inset-x-0 top-6 z-50 flex justify-center px-4">
                <div class={format!("w-full max-w-lg rounded-2xl border p-4 shadow-xl {}", severity_class())}>
                    <div class="flex items-start gap-3">
                        <div class="mt-1 text-2xl">{icon()}</div>
                        <div class="flex-1 min-w-0">
                            <p class="text-sm font-semibold uppercase tracking-wider">{title()}</p>
                            <p class="mt-2 text-sm leading-6">{message.get()}</p>
                            <Show when=move || !action_id.get().is_empty() fallback=|| view! {}>
                                <div class="mt-4 flex justify-end gap-2">
                                    <button
                                        type="button"
                                        class="rounded-lg border border-slate-200 bg-white px-4 py-2 text-sm font-semibold text-slate-700 transition hover:bg-slate-50"
                                        on:click=on_cancel
                                    >
                                        {move || cancel_label.get()}
                                    </button>
                                    <button
                                        type="button"
                                        class="rounded-lg bg-red-600 px-4 py-2 text-sm font-semibold text-white transition hover:bg-red-700"
                                        on:click=on_confirm
                                    >
                                        {move || confirm_label.get()}
                                    </button>
                                </div>
                            </Show>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}
