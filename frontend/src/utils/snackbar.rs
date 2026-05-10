use wasm_bindgen::JsValue;
use web_sys::{window, CustomEvent};

pub fn show_snackbar(message: &str, severity: &str) {
    let detail = format!("{}\0{}", message, severity);

    if let Some(win) = window() {
        if let Ok(event) = CustomEvent::new("showSnackbar") {
            event.init_custom_event_with_can_bubble_and_cancelable_and_detail(
                "showSnackbar",
                true,
                false,
                &JsValue::from_str(&detail),
            );
            let _ = win.dispatch_event(&event);
        }
    }
}
