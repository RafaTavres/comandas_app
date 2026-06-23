use serde::{Deserialize, Serialize};

use crate::{
    services::api::{self, ApiError},
    utils::api_config::endpoints,
};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Produto {
    #[serde(default)]
    pub id: u32,
    #[serde(default)]
    pub nome: String,
    #[serde(default)]
    pub descricao: String,
    #[serde(default, alias = "valor", alias = "valorUnitario")]
    pub valor_unitario: f64,
    #[serde(default, alias = "imagem")]
    pub foto: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ProdutoPayload {
    pub nome: String,
    pub descricao: String,
    pub valor_unitario: f64,
    pub foto: String,
}

pub fn foto_to_src(foto: Option<&str>) -> Option<String> {
    let foto = foto?.trim();

    if foto.is_empty() {
        return None;
    }

    if foto.starts_with("data:") || foto.starts_with("http://") || foto.starts_with("https://") {
        return Some(foto.to_string());
    }

    let compact = foto.chars().filter(|char| !char.is_whitespace()).collect::<String>();
    let looks_like_base64 = compact.len() > 32
        && compact
            .chars()
            .all(|char| char.is_ascii_alphanumeric() || matches!(char, '+' | '/' | '='));

    if looks_like_base64 {
        Some(format!("data:{};base64,{compact}", base64_image_mime(&compact)))
    } else if foto.starts_with('/') {
        Some(foto.to_string())
    } else {
        None
    }
}

fn base64_image_mime(base64: &str) -> &'static str {
    if base64.starts_with("/9j/") {
        "image/jpeg"
    } else if base64.starts_with("iVBORw0KGgo") {
        "image/png"
    } else if base64.starts_with("R0lGOD") {
        "image/gif"
    } else if base64.starts_with("UklGR") {
        "image/webp"
    } else if base64.starts_with("Qk") {
        "image/bmp"
    } else {
        "image/jpeg"
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProdutoListParams {
    pub skip: usize,
    pub limit: usize,
    pub id: Option<u32>,
    pub nome: Option<String>,
    pub descricao: Option<String>,
    pub valor: Option<f64>,
    pub valor_min: Option<f64>,
    pub valor_max: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ProdutoListResponse {
    List(Vec<Produto>),
    Data { data: Vec<Produto> },
    Results { results: Vec<Produto> },
    Items { items: Vec<Produto> },
}

impl ProdutoListResponse {
    fn into_items(self) -> Vec<Produto> {
        match self {
            Self::List(items) | Self::Data { data: items } | Self::Results { results: items } | Self::Items { items } => items,
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

fn endpoint_with_query(base: &str, params: &ProdutoListParams) -> String {
    let mut query = Vec::new();

    append_query(&mut query, "skip", params.skip);
    append_query(&mut query, "limit", params.limit);

    if let Some(id) = params.id {
        append_query(&mut query, "id", id);
    }

    if let Some(nome) = &params.nome {
        append_query(&mut query, "nome", nome);
    }

    if let Some(descricao) = &params.descricao {
        append_query(&mut query, "descricao", descricao);
    }

    if let Some(valor) = params.valor {
        append_query(&mut query, "valor", valor);
    }

    if let Some(valor_min) = params.valor_min {
        append_query(&mut query, "valor_min", valor_min);
    }

    if let Some(valor_max) = params.valor_max {
        append_query(&mut query, "valor_max", valor_max);
    }

    if query.is_empty() {
        base.to_string()
    } else {
        format!("{base}?{}", query.join("&"))
    }
}

pub async fn list(params: ProdutoListParams) -> Result<Vec<Produto>, ApiError> {
    let endpoint = endpoint_with_query(endpoints::produto::LIST, &params);
    let response = api::get::<ProdutoListResponse>(&endpoint).await?;

    Ok(response.into_items())
}

pub async fn list_public(params: ProdutoListParams) -> Result<Vec<Produto>, ApiError> {
    let endpoint = endpoint_with_query(endpoints::produto::PUBLIC_LIST, &params);
    let response = api::get_public::<ProdutoListResponse>(&endpoint).await?;

    Ok(response.into_items())
}

pub async fn get_by_id(id: u32) -> Result<Produto, ApiError> {
    api::get(&endpoints::produto::get(id)).await
}

pub async fn create(payload: &ProdutoPayload) -> Result<Produto, ApiError> {
    api::post(endpoints::produto::CREATE, payload).await
}

pub async fn update(id: u32, payload: &ProdutoPayload) -> Result<Produto, ApiError> {
    api::put(&endpoints::produto::update(id), payload).await
}

pub async fn delete(id: u32) -> Result<(), ApiError> {
    api::delete_empty(&endpoints::produto::delete(id)).await
}
