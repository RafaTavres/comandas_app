use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::{
    constants::comanda_status::ComandaStatus,
    services::api::{self, ApiError},
    utils::api_config::endpoints,
};

fn deserialize_string_or_default<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;

    Ok(match value {
        Some(Value::String(value)) => value,
        Some(Value::Number(value)) => value.to_string(),
        Some(Value::Bool(value)) => value.to_string(),
        Some(Value::Object(value)) => value
            .get("nome")
            .or_else(|| value.get("name"))
            .or_else(|| value.get("descricao"))
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_string(),
        _ => String::new(),
    })
}

fn deserialize_option_u32<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;

    match value {
        Some(Value::Number(value)) => value
            .as_u64()
            .and_then(|value| u32::try_from(value).ok())
            .map(Some)
            .ok_or_else(|| de::Error::custom("valor numerico invalido")),
        Some(Value::String(value)) if !value.trim().is_empty() => value
            .trim()
            .parse::<u32>()
            .map(Some)
            .map_err(de::Error::custom),
        _ => Ok(None),
    }
}

fn deserialize_f64_or_default<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;

    match value {
        Some(Value::Number(value)) => value
            .as_f64()
            .ok_or_else(|| de::Error::custom("valor numerico invalido")),
        Some(Value::String(value)) if !value.trim().is_empty() => value
            .trim()
            .replace(',', ".")
            .parse::<f64>()
            .map_err(de::Error::custom),
        _ => Ok(0.0),
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Comanda {
    #[serde(default)]
    pub id: u32,
    #[serde(default, deserialize_with = "deserialize_string_or_default")]
    pub comanda: String,
    #[serde(default)]
    pub status: i32,
    #[serde(default, deserialize_with = "deserialize_option_u32")]
    pub funcionario_id: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_option_u32")]
    pub cliente_id: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_string_or_default", alias = "cliente")]
    pub cliente_nome: String,
    #[serde(default, deserialize_with = "deserialize_string_or_default", alias = "funcionario")]
    pub funcionario_nome: String,
    #[serde(default, alias = "abertura", alias = "data_abertura")]
    pub data_hora: Option<String>,
    #[serde(default, alias = "fechamento")]
    pub data_fechamento: Option<String>,
    #[serde(default, alias = "valor_total", alias = "total")]
    pub total: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ComandaItem {
    #[serde(default)]
    pub id: u32,
    #[serde(default, deserialize_with = "deserialize_option_u32")]
    pub comanda_id: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_option_u32")]
    pub produto_id: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_option_u32")]
    pub funcionario_id: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_string_or_default", alias = "produto")]
    pub produto_nome: String,
    #[serde(default, deserialize_with = "deserialize_string_or_default", alias = "funcionario")]
    pub funcionario_nome: String,
    #[serde(default, deserialize_with = "deserialize_f64_or_default")]
    pub quantidade: f64,
    #[serde(default, deserialize_with = "deserialize_f64_or_default")]
    pub valor_unitario: f64,
    #[serde(default, deserialize_with = "deserialize_f64_or_default", alias = "total")]
    pub valor_total: f64,
    #[serde(default)]
    pub observacao: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ComandaListParams {
    pub skip: usize,
    pub limit: usize,
    pub id: Option<u32>,
    pub comanda: Option<String>,
    pub status: Option<i32>,
    pub funcionario_id: Option<String>,
    pub cliente_id: Option<String>,
    pub data_inicio: Option<String>,
    pub data_fim: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ComandaItemListParams {
    pub skip: usize,
    pub limit: usize,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ComandaStatusPayload {
    pub status: i32,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ComandaListResponse {
    List(Vec<Comanda>),
    Data { data: Vec<Comanda> },
    Results { results: Vec<Comanda> },
    Items { items: Vec<Comanda> },
}

impl ComandaListResponse {
    fn into_items(self) -> Vec<Comanda> {
        match self {
            Self::List(items)
            | Self::Data { data: items }
            | Self::Results { results: items }
            | Self::Items { items } => items,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ComandaItemListResponse {
    List(Vec<ComandaItem>),
    Data { data: Vec<ComandaItem> },
    Results { results: Vec<ComandaItem> },
    Items { items: Vec<ComandaItem> },
}

impl ComandaItemListResponse {
    fn into_items(self) -> Vec<ComandaItem> {
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

fn endpoint_with_query(base: &str, query: Vec<String>) -> String {
    if query.is_empty() {
        base.to_string()
    } else {
        format!("{base}?{}", query.join("&"))
    }
}

fn comanda_endpoint_with_query(base: &str, params: &ComandaListParams) -> String {
    let mut query = Vec::new();

    append_query(&mut query, "skip", params.skip);
    append_query(&mut query, "limit", params.limit);

    if let Some(id) = params.id {
        append_query(&mut query, "id", id);
    }

    if let Some(comanda) = &params.comanda {
        append_query(&mut query, "comanda", comanda);
    }

    if let Some(status) = params.status {
        append_query(&mut query, "status", status);
    }

    if let Some(funcionario_id) = &params.funcionario_id {
        append_query(&mut query, "funcionario_id", funcionario_id);
    }

    if let Some(cliente_id) = &params.cliente_id {
        append_query(&mut query, "cliente_id", cliente_id);
    }

    if let Some(data_inicio) = &params.data_inicio {
        append_query(&mut query, "data_inicio", data_inicio);
    }

    if let Some(data_fim) = &params.data_fim {
        append_query(&mut query, "data_fim", data_fim);
    }

    endpoint_with_query(base, query)
}

fn item_endpoint_with_query(base: &str, params: &ComandaItemListParams) -> String {
    let mut query = Vec::new();

    append_query(&mut query, "skip", params.skip);
    append_query(&mut query, "limit", params.limit);

    endpoint_with_query(base, query)
}

pub async fn list(params: ComandaListParams) -> Result<Vec<Comanda>, ApiError> {
    let endpoint = comanda_endpoint_with_query(endpoints::comanda::LIST, &params);
    let response = api::get::<ComandaListResponse>(&endpoint).await?;

    Ok(response.into_items())
}

pub async fn get_by_id(id: u32) -> Result<Comanda, ApiError> {
    api::get(&endpoints::comanda::get(id)).await
}

pub async fn check_in_use(
    comanda: impl Into<String>,
    current_id: Option<u32>,
) -> Result<Option<Comanda>, ApiError> {
    let comanda = comanda.into();

    if comanda.trim().is_empty() {
        return Ok(None);
    }

    let items = list(ComandaListParams {
        skip: 0,
        limit: 100,
        comanda: Some(comanda),
        status: Some(ComandaStatus::Aberta as i32),
        ..ComandaListParams::default()
    })
    .await?;

    Ok(items
        .into_iter()
        .find(|item| current_id.map_or(true, |id| item.id != id)))
}

pub async fn create<B>(payload: &B) -> Result<Comanda, ApiError>
where
    B: Serialize + ?Sized,
{
    api::post(endpoints::comanda::CREATE, payload).await
}

pub async fn update<B>(id: u32, payload: &B) -> Result<Comanda, ApiError>
where
    B: Serialize + ?Sized,
{
    api::put(&endpoints::comanda::update(id), payload).await
}

pub async fn delete(id: u32) -> Result<(), ApiError> {
    let delete_error = match api::delete_empty(&endpoints::comanda::delete(id)).await {
        Ok(()) => return Ok(()),
        Err(error) => error,
    };

    // Algumas versoes da API removem a comanda no banco, mas retornam 500 depois do commit.
    // Se a comanda ja nao existir mais na listagem, a operacao deve ser tratada como sucesso.
    match list(ComandaListParams {
        skip: 0,
        limit: 1,
        id: Some(id),
        ..ComandaListParams::default()
    })
    .await
    {
        Ok(items) if items.iter().all(|item| item.id != id) => Ok(()),
        _ => Err(delete_error),
    }
}

pub async fn cancel(id: u32) -> Result<(), ApiError> {
    api::put_empty(&endpoints::comanda::cancel(id)).await
}

pub async fn close(id: u32) -> Result<Comanda, ApiError> {
    update(
        id,
        &ComandaStatusPayload {
            status: ComandaStatus::Fechada as i32,
        },
    )
    .await
}

pub async fn add_item<B>(comanda_id: u32, payload: &B) -> Result<ComandaItem, ApiError>
where
    B: Serialize + ?Sized,
{
    api::post(&endpoints::comanda::add_item(comanda_id), payload).await
}

pub async fn list_items(
    comanda_id: u32,
    params: ComandaItemListParams,
) -> Result<Vec<ComandaItem>, ApiError> {
    let endpoint = item_endpoint_with_query(&endpoints::comanda::list_items(comanda_id), &params);
    let response = api::get::<ComandaItemListResponse>(&endpoint).await?;

    Ok(response.into_items())
}

pub fn item_total(item: &ComandaItem) -> f64 {
    if item.valor_total > 0.0 {
        item.valor_total
    } else {
        item.quantidade * item.valor_unitario
    }
}

pub fn items_total(items: &[ComandaItem]) -> f64 {
    items.iter().map(item_total).sum()
}

pub async fn update_item<B>(
    id: u32,
    payload: &B,
) -> Result<ComandaItem, ApiError>
where
    B: Serialize + ?Sized,
{
    api::put(&endpoints::comanda::update_item(id), payload).await
}

pub async fn remove_item(id: u32) -> Result<(), ApiError> {
    api::delete_empty(&endpoints::comanda::remove_item(id)).await
}
