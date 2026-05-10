use leptos::prelude::*;

#[component]
pub fn ControlledInput(
    label: String,
    value: ReadSignal<String>,
    on_input: std::sync::Arc<dyn Fn(String) + Send + Sync>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] input_type: Option<String>,
    #[prop(optional)] input_mode: Option<String>,
    #[prop(optional)] error: Option<String>,
) -> impl IntoView {
    let input_type = input_type.unwrap_or_else(|| "text".to_string());
    let input_mode = input_mode.unwrap_or_default();
    let placeholder = placeholder.unwrap_or_default();
    
    let show_error = error.is_some(); 
    
    let error_message = error.unwrap_or_default(); 

    view! {
        <div class="min-w-0 space-y-2">
            <label class="block text-sm font-semibold text-slate-700">{label}</label>
            <input
                type=input_type
                class="w-full min-w-0 rounded-xl border border-slate-200 px-4 py-3 focus:border-amber-500 focus:outline-none focus:ring-2 focus:ring-amber-200"
                prop:value=move || value.get()
                placeholder=placeholder
                inputmode=input_mode
                on:input=move |ev| on_input(event_target_value(&ev))
            />
            <Show when=move || show_error>
                <p class="text-sm text-red-500">{error_message.clone()}</p>
            </Show>
        </div>
    }
}
