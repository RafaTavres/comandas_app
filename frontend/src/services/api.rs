use reqwest::{Client, Method, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use web_sys::{window, Storage};

use crate::utils::api_config::{endpoints, url};

pub(crate) const ACCESS_TOKEN_KEY: &str = "access_token";
pub(crate) const REFRESH_TOKEN_KEY: &str = "refresh_token";
pub(crate) const TOKEN_TYPE_KEY: &str = "token_type";
pub(crate) const EXPIRES_IN_KEY: &str = "expires_in";
pub(crate) const REFRESH_EXPIRES_IN_KEY: &str = "refresh_expires_in";
pub(crate) const EXPIRES_AT_KEY: &str = "expires_at";
pub(crate) const REFRESH_EXPIRES_AT_KEY: &str = "refresh_expires_at";
pub(crate) const LOGIN_REALIZADO_KEY: &str = "loginRealizado";

#[derive(Debug, Clone)]
pub struct ApiError {
    pub status: Option<StatusCode>,
    pub message: String,
}

impl ApiError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            status: None,
            message: message.into(),
        }
    }

    fn with_status(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status: Some(status),
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.message)
    }
}

impl std::error::Error for ApiError {}

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        Self {
            status: error.status(),
            message: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(error: serde_json::Error) -> Self {
        Self::new(error.to_string())
    }
}

#[derive(Debug, Deserialize)]
struct ApiErrorBody {
    detail: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Serialize)]
struct RefreshRequest {
    refresh_token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_expires_in: u64,
}

pub async fn get<T>(endpoint: &str) -> Result<T, ApiError>
where
    T: DeserializeOwned,
{
    request(Method::GET, endpoint, None, true, true).await
}

pub async fn get_public<T>(endpoint: &str) -> Result<T, ApiError>
where
    T: DeserializeOwned,
{
    request(Method::GET, endpoint, None, false, false).await
}

pub async fn post<B, T>(endpoint: &str, body: &B) -> Result<T, ApiError>
where
    B: Serialize + ?Sized,
    T: DeserializeOwned,
{
    request(Method::POST, endpoint, Some(serde_json::to_value(body)?), true, true).await
}

pub async fn post_public<B, T>(endpoint: &str, body: &B) -> Result<T, ApiError>
where
    B: Serialize + ?Sized,
    T: DeserializeOwned,
{
    request(Method::POST, endpoint, Some(serde_json::to_value(body)?), false, false).await
}

pub async fn put<B, T>(endpoint: &str, body: &B) -> Result<T, ApiError>
where
    B: Serialize + ?Sized,
    T: DeserializeOwned,
{
    request(Method::PUT, endpoint, Some(serde_json::to_value(body)?), true, true).await
}

pub async fn put_without_body<T>(endpoint: &str) -> Result<T, ApiError>
where
    T: DeserializeOwned,
{
    request(Method::PUT, endpoint, None, true, true).await
}

pub async fn put_empty(endpoint: &str) -> Result<(), ApiError> {
    request_empty(Method::PUT, endpoint, None, true, true).await
}

pub async fn delete<T>(endpoint: &str) -> Result<T, ApiError>
where
    T: DeserializeOwned,
{
    request(Method::DELETE, endpoint, None, true, true).await
}

pub async fn delete_empty(endpoint: &str) -> Result<(), ApiError> {
    request_empty(Method::DELETE, endpoint, None, true, true).await
}

async fn request<T>(
    method: Method,
    endpoint: &str,
    body: Option<Value>,
    retry_on_unauthorized: bool,
    include_auth: bool,
) -> Result<T, ApiError>
where
    T: DeserializeOwned,
{
    let response = send(method.clone(), endpoint, body.clone(), include_auth).await?;

    if response.status() == StatusCode::UNAUTHORIZED && retry_on_unauthorized {
        refresh_access_token().await?;

        let response = send(method, endpoint, body, include_auth).await?;
        return parse_response(response).await;
    }

    parse_response(response).await
}

async fn request_empty(
    method: Method,
    endpoint: &str,
    body: Option<Value>,
    retry_on_unauthorized: bool,
    include_auth: bool,
) -> Result<(), ApiError> {
    let response = send(method.clone(), endpoint, body.clone(), include_auth).await?;

    if response.status() == StatusCode::UNAUTHORIZED && retry_on_unauthorized {
        refresh_access_token().await?;

        let response = send(method, endpoint, body, include_auth).await?;
        return parse_empty_response(response).await;
    }

    parse_empty_response(response).await
}

async fn send(
    method: Method,
    endpoint: &str,
    body: Option<Value>,
    include_auth: bool,
) -> Result<reqwest::Response, ApiError> {
    let client = Client::new();
    let mut builder = client
        .request(method, url(endpoint))
        .header("Content-Type", "application/json");

    if include_auth && let Some(token) = session_item(ACCESS_TOKEN_KEY) {
        builder = builder.bearer_auth(token);
    }

    if let Some(body) = body {
        builder = builder.json(&body);
    }

    Ok(builder.send().await?)
}

async fn parse_response<T>(response: reqwest::Response) -> Result<T, ApiError>
where
    T: DeserializeOwned,
{
    let status = response.status();

    if status.is_success() {
        return Ok(response.json::<T>().await?);
    }

    let error_body = response.json::<ApiErrorBody>().await.ok();
    let message = error_body
        .and_then(|body| body.detail.or(body.message))
        .unwrap_or_else(|| "Erro desconhecido".to_string());

    Err(ApiError::with_status(status, message))
}

async fn parse_empty_response(response: reqwest::Response) -> Result<(), ApiError> {
    let status = response.status();

    if status.is_success() {
        return Ok(());
    }

    let error_body = response.json::<ApiErrorBody>().await.ok();
    let message = error_body
        .and_then(|body| body.detail.or(body.message))
        .unwrap_or_else(|| "Erro desconhecido".to_string());

    Err(ApiError::with_status(status, message))
}

async fn refresh_access_token() -> Result<(), ApiError> {
    let refresh_token = match session_item(REFRESH_TOKEN_KEY) {
        Some(token) => token,
        None => {
            clear_session_and_redirect();
            return Err(ApiError::new("Sessao expirada"));
        }
    };

    let payload = RefreshRequest { refresh_token };
    let response = send(
        Method::POST,
        endpoints::auth::REFRESH,
        Some(serde_json::to_value(payload)?),
        false,
    )
    .await?;

    if !response.status().is_success() {
        clear_session_and_redirect();
        return Err(ApiError::with_status(response.status(), "Sessao expirada"));
    }

    let tokens = response.json::<AuthTokens>().await?;
    save_tokens(tokens)?;

    Ok(())
}

pub(crate) fn save_tokens(tokens: AuthTokens) -> Result<(), ApiError> {
    let storage = session_storage().ok_or_else(|| ApiError::new("Session storage indisponivel"))?;
    let now = js_sys::Date::now() as u64;
    let expires_at = now + (tokens.expires_in * 1000);
    let refresh_expires_at = now + (tokens.refresh_expires_in * 1000);

    set_storage_item(&storage, ACCESS_TOKEN_KEY, &tokens.access_token)?;
    set_storage_item(&storage, REFRESH_TOKEN_KEY, &tokens.refresh_token)?;
    set_storage_item(&storage, TOKEN_TYPE_KEY, &tokens.token_type)?;
    set_storage_item(&storage, EXPIRES_IN_KEY, &tokens.expires_in.to_string())?;
    set_storage_item(
        &storage,
        REFRESH_EXPIRES_IN_KEY,
        &tokens.refresh_expires_in.to_string(),
    )?;
    set_storage_item(&storage, EXPIRES_AT_KEY, &expires_at.to_string())?;
    set_storage_item(&storage, REFRESH_EXPIRES_AT_KEY, &refresh_expires_at.to_string())?;
    set_storage_item(&storage, LOGIN_REALIZADO_KEY, "true")?;

    Ok(())
}

fn set_storage_item(storage: &Storage, key: &str, value: &str) -> Result<(), ApiError> {
    storage
        .set_item(key, value)
        .map_err(|_| ApiError::new(format!("Nao foi possivel salvar {key}")))
}

pub(crate) fn session_item(key: &str) -> Option<String> {
    session_storage()?.get_item(key).ok().flatten()
}

pub(crate) fn session_storage() -> Option<Storage> {
    window()?.session_storage().ok().flatten()
}

pub fn clear_session_and_redirect() {
    if let Some(storage) = session_storage() {
        storage.clear().ok();
    }

    if let Some(window) = window() {
        window.location().set_href("/login").ok();
    }
}
