use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::{
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
            .or_else(|| value.get("comanda"))
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
pub struct CaixaProdutoComanda {
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
    #[serde(default, alias = "foto", alias = "imagem")]
    pub foto: Option<String>,
    #[serde(default, deserialize_with = "deserialize_f64_or_default", alias = "total")]
    pub valor_total: f64,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CaixaComandaDetalhe {
    #[serde(default)]
    pub id: u32,
    #[serde(default, deserialize_with = "deserialize_string_or_default")]
    pub comanda: String,
    #[serde(default, alias = "abertura", alias = "data_abertura")]
    pub data_hora: Option<String>,
    #[serde(default)]
    pub status: i32,
    #[serde(default, deserialize_with = "deserialize_option_u32")]
    pub funcionario_id: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_string_or_default", alias = "funcionario")]
    pub funcionario_nome: String,
    #[serde(default, deserialize_with = "deserialize_option_u32")]
    pub cliente_id: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_string_or_default", alias = "cliente")]
    pub cliente_nome: String,
    #[serde(default)]
    pub produtos: Vec<CaixaProdutoComanda>,
    #[serde(default, deserialize_with = "deserialize_f64_or_default", alias = "total")]
    pub valor_total: f64,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CaixaComandaResumo {
    #[serde(default, alias = "comanda_id")]
    pub id: u32,
    #[serde(default, deserialize_with = "deserialize_string_or_default")]
    pub comanda: String,
    #[serde(
        default,
        deserialize_with = "deserialize_f64_or_default",
        alias = "valor_comanda",
        alias = "total"
    )]
    pub valor_total: f64,
    #[serde(default, deserialize_with = "deserialize_option_u32")]
    pub cliente_id: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_string_or_default", alias = "cliente")]
    pub cliente_nome: String,
    #[serde(default)]
    pub produtos: Vec<CaixaProdutoComanda>,
    #[serde(default)]
    pub detalhe: Option<CaixaComandaDetalhe>,
}

impl CaixaComandaResumo {
    pub fn produtos(&self) -> Vec<CaixaProdutoComanda> {
        self.detalhe
            .as_ref()
            .map(|detalhe| detalhe.produtos.clone())
            .filter(|produtos| !produtos.is_empty())
            .unwrap_or_else(|| self.produtos.clone())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CaixaResumoResponse {
    #[serde(default)]
    pub comandas: Vec<CaixaComandaResumo>,
    #[serde(default)]
    pub quantidade_comandas: u32,
    #[serde(default, deserialize_with = "deserialize_f64_or_default", alias = "total")]
    pub valor_total: f64,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CaixaSelecaoComandasRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comanda_ids: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeros_comandas: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct RecebimentoCreatePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comanda_ids: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeros_comandas: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cliente_id: Option<u32>,
    pub desconto: f64,
    pub acrescimo: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observacao: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct RecebimentoUpdatePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cliente_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desconto: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acrescimo: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observacao: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecebimentoListParams {
    pub skip: usize,
    pub limit: usize,
    pub id: Option<u32>,
    pub funcionario_id: Option<u32>,
    pub cliente_id: Option<u32>,
    pub data_inicio: Option<String>,
    pub data_fim: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Recebimento {
    #[serde(default)]
    pub id: u32,
    #[serde(default)]
    pub data_hora: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_u32")]
    pub funcionario_id: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_string_or_default", alias = "funcionario")]
    pub funcionario_nome: String,
    #[serde(default, deserialize_with = "deserialize_option_u32")]
    pub cliente_id: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_string_or_default", alias = "cliente")]
    pub cliente_nome: String,
    #[serde(default, deserialize_with = "deserialize_f64_or_default")]
    pub valor_total: f64,
    #[serde(default, deserialize_with = "deserialize_f64_or_default")]
    pub desconto: f64,
    #[serde(default, deserialize_with = "deserialize_f64_or_default")]
    pub acrescimo: f64,
    #[serde(default, deserialize_with = "deserialize_f64_or_default")]
    pub valor_final: f64,
    #[serde(default)]
    pub observacao: Option<String>,
    #[serde(default)]
    pub comandas: Vec<CaixaComandaResumo>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ComprovantePagamento {
    #[serde(flatten)]
    pub recebimento: Recebimento,
    #[serde(default)]
    pub mensagem: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum CaixaResumoApiResponse {
    Summary(CaixaResumoResponse),
    List(Vec<CaixaComandaResumo>),
    Data { data: Vec<CaixaComandaResumo> },
    Results { results: Vec<CaixaComandaResumo> },
    Items { items: Vec<CaixaComandaResumo> },
}

impl CaixaResumoApiResponse {
    fn into_summary(self) -> CaixaResumoResponse {
        match self {
            Self::Summary(summary) => summary,
            Self::List(comandas)
            | Self::Data { data: comandas }
            | Self::Results { results: comandas }
            | Self::Items { items: comandas } => {
                let valor_total = comandas.iter().map(|item| item.valor_total).sum();

                CaixaResumoResponse {
                    quantidade_comandas: comandas.len() as u32,
                    comandas,
                    valor_total,
                }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum RecebimentoListResponse {
    List(Vec<Recebimento>),
    Data { data: Vec<Recebimento> },
    Results { results: Vec<Recebimento> },
    Items { items: Vec<Recebimento> },
}

impl RecebimentoListResponse {
    fn into_items(self) -> Vec<Recebimento> {
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

fn recebimento_endpoint_with_query(base: &str, params: &RecebimentoListParams) -> String {
    let mut query = Vec::new();

    append_query(&mut query, "skip", params.skip);
    append_query(&mut query, "limit", params.limit);

    if let Some(id) = params.id {
        append_query(&mut query, "id", id);
    }

    if let Some(funcionario_id) = params.funcionario_id {
        append_query(&mut query, "funcionario_id", funcionario_id);
    }

    if let Some(cliente_id) = params.cliente_id {
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

fn dashboard_endpoint(skip: usize, limit: usize) -> String {
    let mut query = Vec::new();

    append_query(&mut query, "skip", skip);
    append_query(&mut query, "limit", limit);

    endpoint_with_query(endpoints::caixa::DASHBOARD, query)
}

pub async fn dashboard(skip: usize, limit: usize) -> Result<CaixaResumoResponse, ApiError> {
    let response = api::get::<CaixaResumoApiResponse>(&dashboard_endpoint(skip, limit)).await?;

    Ok(response.into_summary())
}

pub async fn selecionar_comandas(
    payload: &CaixaSelecaoComandasRequest,
) -> Result<CaixaResumoResponse, ApiError> {
    let response =
        api::post::<_, CaixaResumoApiResponse>(endpoints::caixa::SELECIONAR_COMANDAS, payload)
            .await?;

    Ok(response.into_summary())
}

pub async fn list_recebimentos(
    params: RecebimentoListParams,
) -> Result<Vec<Recebimento>, ApiError> {
    let endpoint = recebimento_endpoint_with_query(endpoints::recebimento::LIST, &params);
    let response = api::get::<RecebimentoListResponse>(&endpoint).await?;

    Ok(response.into_items())
}

pub async fn get_recebimento(id: u32) -> Result<Recebimento, ApiError> {
    api::get(&endpoints::recebimento::get(id)).await
}

pub async fn create_recebimento(
    payload: &RecebimentoCreatePayload,
) -> Result<Recebimento, ApiError> {
    api::post(endpoints::recebimento::CREATE, payload).await
}

pub async fn update_recebimento(
    id: u32,
    payload: &RecebimentoUpdatePayload,
) -> Result<Recebimento, ApiError> {
    api::put(&endpoints::recebimento::update(id), payload).await
}

pub async fn delete_recebimento(id: u32) -> Result<(), ApiError> {
    api::delete_empty(&endpoints::recebimento::delete(id)).await
}

pub async fn comprovante(id: u32) -> Result<ComprovantePagamento, ApiError> {
    api::get(&endpoints::recebimento::comprovante(id)).await
}
