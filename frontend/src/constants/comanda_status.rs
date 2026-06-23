use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum ComandaStatus {
    Aberta = 0,
    Fechada = 1,
    Cancelada = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusConfig {
    pub label: &'static str,
    pub badge_class: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusOption {
    pub value: i32,
    pub label: &'static str,
}

pub const STATUS_OPTIONS: &[StatusOption] = &[
    StatusOption {
        value: ComandaStatus::Aberta as i32,
        label: "Aberta",
    },
    StatusOption {
        value: ComandaStatus::Fechada as i32,
        label: "Fechada",
    },
    StatusOption {
        value: ComandaStatus::Cancelada as i32,
        label: "Cancelada",
    },
];

pub fn status_from_i32(status: i32) -> Option<ComandaStatus> {
    match status {
        0 => Some(ComandaStatus::Aberta),
        1 => Some(ComandaStatus::Fechada),
        2 => Some(ComandaStatus::Cancelada),
        _ => None,
    }
}

pub fn status_config(status: i32) -> StatusConfig {
    match status_from_i32(status) {
        Some(ComandaStatus::Aberta) => StatusConfig {
            label: "Aberta",
            badge_class: "bg-emerald-100 text-emerald-700 border-emerald-200",
        },
        Some(ComandaStatus::Fechada) => StatusConfig {
            label: "Fechada",
            badge_class: "bg-red-100 text-red-700 border-red-200",
        },
        Some(ComandaStatus::Cancelada) => StatusConfig {
            label: "Cancelada",
            badge_class: "bg-orange-100 text-orange-700 border-orange-200",
        },
        None => StatusConfig {
            label: "Desconhecido",
            badge_class: "bg-slate-100 text-slate-700 border-slate-200",
        },
    }
}
