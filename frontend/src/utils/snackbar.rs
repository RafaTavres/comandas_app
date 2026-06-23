use wasm_bindgen::JsValue;
use web_sys::{window, CustomEvent};

const SHOW_SNACKBAR_EVENT: &str = "showSnackbar";

pub fn show_snackbar(message: &str, severity: &str) {
    let detail = format!("{}\0{}", message, severity);
    dispatch_snackbar_event(&detail);
}

pub fn show_confirm_snackbar(
    message: &str,
    severity: &str,
    confirm_label: &str,
    cancel_label: &str,
    action_id: &str,
) {
    let detail = format!(
        "{}\0{}\0{}\0{}\0{}",
        message, severity, confirm_label, cancel_label, action_id
    );

    dispatch_snackbar_event(&detail);
}

fn dispatch_snackbar_event(detail: &str) {
    if let Some(win) = window() {
        if let Ok(event) = CustomEvent::new(SHOW_SNACKBAR_EVENT) {
            event.init_custom_event_with_can_bubble_and_cancelable_and_detail(
                SHOW_SNACKBAR_EVENT,
                true,
                false,
                &JsValue::from_str(&detail),
            );
            let _ = win.dispatch_event(&event);
        }
    }
}
