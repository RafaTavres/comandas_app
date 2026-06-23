use serde::{Deserialize, Serialize};

use crate::{
    services::api::{self, ApiError},
    utils::api_config::endpoints,
};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Cliente {
    #[serde(default)]
    pub id: u32,
    #[serde(default)]
    pub nome: String,
    #[serde(default)]
    pub cpf: String,
    #[serde(default)]
    pub telefone: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ClientePayload {
    pub nome: String,
    pub cpf: String,
    pub telefone: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClienteListParams {
    pub skip: usize,
    pub limit: usize,
    pub id: Option<u32>,
    pub nome: Option<String>,
    pub cpf: Option<String>,
    pub telefone: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ClienteListResponse {
    List(Vec<Cliente>),
    Data { data: Vec<Cliente> },
    Results { results: Vec<Cliente> },
    Items { items: Vec<Cliente> },
}

impl ClienteListResponse {
    fn into_items(self) -> Vec<Cliente> {
        match self {
            Self::List(items)
            | Self::Data { data: items }
            | Self::Results { results: items }
            | Self::Items { items } => items,
        }
    }
}

fn append_query(query: &mut Vec<String>, key: &str, value: impl ToString) {
    let value = value.to_string();
    let encoded = js_sys::encode_uri_component(&value)
        .as_string()
        .unwrap_or(value);

    query.push(format!("{key}={encoded}"));
}

fn endpoint_with_query(base: &str, params: &ClienteListParams) -> String {
    let mut query = Vec::new();

    append_query(&mut query, "skip", params.skip);
    append_query(&mut query, "limit", params.limit);

    if let Some(id) = params.id {
        append_query(&mut query, "id", id);
    }

    if let Some(nome) = &params.nome {
        append_query(&mut query, "nome", nome);
    }

    if let Some(cpf) = &params.cpf {
        append_query(&mut query, "cpf", cpf);
    }

    if let Some(telefone) = &params.telefone {
        append_query(&mut query, "telefone", telefone);
    }

    if query.is_empty() {
        base.to_string()
    } else {
        format!("{base}?{}", query.join("&"))
    }
}

pub async fn list(params: ClienteListParams) -> Result<Vec<Cliente>, ApiError> {
    let endpoint = endpoint_with_query(endpoints::cliente::LIST, &params);
    let response = api::get::<ClienteListResponse>(&endpoint).await?;

    Ok(response.into_items())
}

pub async fn get_by_id(id: u32) -> Result<Cliente, ApiError> {
    api::get(&endpoints::cliente::get(id)).await
}

pub async fn create(payload: &ClientePayload) -> Result<Cliente, ApiError> {
    api::post(endpoints::cliente::CREATE, payload).await
}

pub async fn update(id: u32, payload: &ClientePayload) -> Result<Cliente, ApiError> {
    api::put(&endpoints::cliente::update(id), payload).await
}

pub async fn delete(id: u32) -> Result<(), ApiError> {
    api::delete_empty(&endpoints::cliente::delete(id)).await
}
