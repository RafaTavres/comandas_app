use std::{env, fs};

const FRONTEND_ENV_KEYS: &[&str] = &[
    "APP_API_BASE_URL",
    "APP_API_TIMEOUT_MS",
    "APP_NAME",
    "APP_VERSION",
    "APP_DEBUG_MODE",
    "APP_LOG_LEVEL",
];

fn main() {
    println!("cargo:rerun-if-changed=.env");
    println!("cargo:rerun-if-changed=.env.example");

    if let Ok(contents) = fs::read_to_string(".env") {
        for line in contents.lines() {
            if let Some((key, value)) = parse_env_line(line) {
                if FRONTEND_ENV_KEYS.contains(&key.as_str()) && env::var_os(&key).is_none() {
                    println!("cargo:rustc-env={key}={value}");
                }
            }
        }
    }
}

fn parse_env_line(line: &str) -> Option<(String, String)> {
    let line = line.trim();

    if line.is_empty() || line.starts_with('#') {
        return None;
    }

    let (key, value) = line.split_once('=')?;
    let key = key.trim();

    if key.is_empty() {
        return None;
    }

    Some((key.to_string(), unquote(value.trim())))
}

fn unquote(value: &str) -> String {
    if value.len() >= 2 {
        let first = value.as_bytes()[0];
        let last = value.as_bytes()[value.len() - 1];

        if (first == b'"' && last == b'"') || (first == b'\'' && last == b'\'') {
            return value[1..value.len() - 1].to_string();
        }
    }

    value.to_string()
}
