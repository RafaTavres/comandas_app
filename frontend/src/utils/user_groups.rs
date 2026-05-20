use serde_json::Value;

pub mod user_groups {
    pub const ADMINISTRADOR: i32 = 1;
    pub const ATENDENTE: i32 = 2;
    pub const CAIXA: i32 = 3;
}

#[derive(Clone, Copy)]
pub struct GroupInfo {
    pub label: &'static str,
    pub color_class: &'static str,
}

pub fn get_group_info(group: Option<i32>) -> GroupInfo {
    match group {
        Some(user_groups::ADMINISTRADOR) => GroupInfo {
            label: "Administrador",
            color_class: "bg-red-100 text-red-700",
        },
        Some(user_groups::ATENDENTE) => GroupInfo {
            label: "Atendente",
            color_class: "bg-sky-100 text-sky-700",
        },
        Some(user_groups::CAIXA) => GroupInfo {
            label: "Caixa",
            color_class: "bg-emerald-100 text-emerald-700",
        },
        _ => GroupInfo {
            label: "Nao definido",
            color_class: "bg-slate-100 text-slate-700",
        },
    }
}

pub fn read_group(user: &Value) -> Option<i32> {
    user.get("grupo")
        .or_else(|| user.get("group"))
        .or_else(|| user.get("role"))
        .and_then(|value| {
            value
                .as_i64()
                .map(|group| group as i32)
                .or_else(|| value.as_str()?.parse::<i32>().ok())
        })
}
