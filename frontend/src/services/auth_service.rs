use serde::{de::DeserializeOwned, Serialize};
use web_sys::window;

use crate::{
    services::api::{
        self, AuthTokens, EXPIRES_AT_KEY, LOGIN_REALIZADO_KEY,
    },
    utils::api_config::endpoints,
};

const TOKEN_EXPIRING_SOON_MS: u64 = 5 * 60 * 1000;

#[derive(Debug, Serialize)]
struct LoginRequest {
    cpf: String,
    senha: String,
}

#[derive(Debug, Clone)]
pub struct AuthResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> AuthResult<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.into()),
        }
    }
}

pub async fn login(cpf: &str, senha: &str) -> AuthResult<AuthTokens> {
    let payload = LoginRequest {
        cpf: only_digits(cpf),
        senha: senha.to_string(),
    };

    match api::post::<_, AuthTokens>(endpoints::auth::LOGIN, &payload).await {
        Ok(tokens) => match api::save_tokens(tokens.clone()) {
            Ok(()) => AuthResult::success(tokens),
            Err(error) => AuthResult::error(error.message),
        },
        Err(error) => AuthResult::error(if error.message.is_empty() {
            "Erro ao realizar login".to_string()
        } else {
            error.message
        }),
    }
}

pub async fn get_user_data<T>() -> Option<T> where T: DeserializeOwned,
{
    api::get::<T>(endpoints::auth::ME).await.ok()
}

pub fn logout() {
    if let Some(storage) = api::session_storage() {
        storage.clear().ok();
    }

    if let Some(window) = window() {
        window.location().set_href("/login").ok();
    }
}

pub fn is_authenticated() -> bool {
    let Some(token) = api::session_item(api::ACCESS_TOKEN_KEY) else {
        return false;
    };

    let Some(expires_at) = api::session_item(EXPIRES_AT_KEY) else {
        return false;
    };

    if token.is_empty() {
        return false;
    }

    let Some(expires_at) = expires_at.parse::<u64>().ok() else {
        return false;
    };

    now_ms() < expires_at
}

pub fn is_token_expiring_soon() -> bool {
    let Some(expires_at) = api::session_item(EXPIRES_AT_KEY) else {
        return true;
    };

    let Some(expires_at) = expires_at.parse::<u64>().ok() else {
        return true;
    };

    now_ms() > expires_at.saturating_sub(TOKEN_EXPIRING_SOON_MS)
}

pub fn mark_authenticated() -> Result<(), api::ApiError> {
    let storage = api::session_storage()
        .ok_or_else(|| api::ApiError::new("Session storage indisponivel"))?;

    storage
        .set_item(LOGIN_REALIZADO_KEY, "true")
        .map_err(|_| api::ApiError::new("Nao foi possivel salvar loginRealizado"))
}

fn only_digits(value: &str) -> String {
    value.chars().filter(|char| char.is_ascii_digit()).collect()
}

fn now_ms() -> u64 {
    js_sys::Date::now() as u64
}
