pub const API_BASE_URL: &str = match option_env!("APP_API_BASE_URL") {
    Some(value) => value,
    None => "/api",
};

pub const APP_NAME: &str = match option_env!("APP_NAME") {
    Some(value) => value,
    None => "Comandas do Ze",
};

pub const APP_VERSION: &str = match option_env!("APP_VERSION") {
    Some(value) => value,
    None => env!("CARGO_PKG_VERSION"),
};

pub const LOG_LEVEL: &str = match option_env!("APP_LOG_LEVEL") {
    Some(value) => value,
    None => "info",
};

pub fn api_timeout_ms() -> u64 {
    option_env!("APP_API_TIMEOUT_MS")
        .and_then(|value| value.parse().ok())
        .unwrap_or(15_000)
}

pub fn debug_mode() -> bool {
    option_env!("APP_DEBUG_MODE")
        .map(|value| matches!(value, "true" | "1" | "yes" | "on"))
        .unwrap_or(false)
}

pub fn api_url(path: &str) -> String {
    let base_url = API_BASE_URL.trim_end_matches('/');
    let path = path.trim_start_matches('/');

    let url = format!("{base_url}/{path}");

    if url.starts_with("http://") || url.starts_with("https://") {
        return url;
    }

    match web_sys::window().and_then(|window| window.location().origin().ok()) {
        Some(origin) => format!("{origin}/{url}", url = url.trim_start_matches('/')),
        None => url,
    }
}
