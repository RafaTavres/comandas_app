use leptos::prelude::*;
use leptos_icons::Icon;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CustomEvent;

#[component]
pub fn SnackbarGlobal() -> impl IntoView {
   
    let (open, set_open) = signal(false);
    let (message, set_message) = signal(String::new());
    let (severity, set_severity) = signal(String::from("success"));

    
    Effect::new(move |_| {
        if open.get() {
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

        let listener = Closure::wrap(Box::new(move |event: web_sys::Event| {
            if let Some(custom) = event.dyn_ref::<CustomEvent>() {
                if let Some(detail) = custom.detail().as_string() {
                    if let Some((message_text, severity_text)) = detail.split_once('\0') {
                        set_message.set(message_text.to_string());
                        set_severity.set(severity_text.to_string());
                        set_open.set(true);
                    }
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
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}