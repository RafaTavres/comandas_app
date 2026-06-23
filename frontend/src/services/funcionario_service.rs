use serde::{Deserialize, Serialize};

use crate::{
    services::api::{self, ApiError},
    utils::api_config::endpoints,
};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Funcionario {
    #[serde(default)]
    pub id: u32,
    #[serde(default)]
    pub nome: String,
    #[serde(default)]
    pub matricula: String,
    #[serde(default)]
    pub cpf: String,
    #[serde(default)]
    pub telefone: String,
    #[serde(default)]
    pub grupo: i32,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct FuncionarioCreatePayload {
    pub nome: String,
    pub matricula: String,
    pub cpf: String,
    pub telefone: String,
    pub grupo: i32,
    pub senha: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct FuncionarioUpdatePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nome: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matricula: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpf: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telefone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grupo: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub senha: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FuncionarioListParams {
    pub skip: usize,
    pub limit: usize,
    pub id: Option<u32>,
    pub nome: Option<String>,
    pub matricula: Option<String>,
    pub cpf: Option<String>,
    pub grupo: Option<String>,
    pub telefone: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum FuncionarioListResponse {
    List(Vec<Funcionario>),
    Data { data: Vec<Funcionario> },
    Results { results: Vec<Funcionario> },
    Items { items: Vec<Funcionario> },
}

impl FuncionarioListResponse {
    fn into_items(self) -> Vec<Funcionario> {
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

fn endpoint_with_query(base: &str, params: &FuncionarioListParams) -> String {
    let mut query = Vec::new();

    append_query(&mut query, "skip", params.skip);
    append_query(&mut query, "limit", params.limit);

    if let Some(id) = params.id {
        append_query(&mut query, "id", id);
    }

    if let Some(nome) = &params.nome {
        append_query(&mut query, "nome", nome);
    }

    if let Some(matricula) = &params.matricula {
        append_query(&mut query, "matricula", matricula);
    }

    if let Some(cpf) = &params.cpf {
        append_query(&mut query, "cpf", cpf);
    }

    if let Some(grupo) = &params.grupo {
        append_query(&mut query, "grupo", grupo);
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

pub async fn list(params: FuncionarioListParams) -> Result<Vec<Funcionario>, ApiError> {
    let endpoint = endpoint_with_query(endpoints::funcionario::LIST, &params);
    let response = api::get::<FuncionarioListResponse>(&endpoint).await?;

    Ok(response.into_items())
}

pub async fn get_by_id(id: u32) -> Result<Funcionario, ApiError> {
    api::get(&endpoints::funcionario::get(id)).await
}

pub async fn create(payload: &FuncionarioCreatePayload) -> Result<Funcionario, ApiError> {
    api::post(endpoints::funcionario::CREATE, payload).await
}

pub async fn update(id: u32, payload: &FuncionarioUpdatePayload) -> Result<Funcionario, ApiError> {
    api::put(&endpoints::funcionario::update(id), payload).await
}

pub async fn delete(id: u32) -> Result<(), ApiError> {
    api::delete_empty(&endpoints::funcionario::delete(id)).await
}
