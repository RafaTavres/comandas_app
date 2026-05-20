use leptos::prelude::*;
use leptos_icons::Icon;
use serde_json::Value;

use crate::utils::user_groups::{get_group_info, read_group};

fn read_text(user: &Value, keys: &[&str]) -> Option<String> {
    keys.iter()
        .find_map(|key| user.get(*key).and_then(Value::as_str))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn read_number_text(user: &Value, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|key| {
        let value = user.get(*key)?;
        value
            .as_str()
            .map(ToOwned::to_owned)
            .or_else(|| value.as_i64().map(|number| number.to_string()))
    })
}

fn format_cpf(cpf: &str) -> String {
    let digits: String = cpf.chars().filter(|char| char.is_ascii_digit()).collect();

    if digits.len() != 11 {
        return cpf.to_string();
    }

    format!(
        "{}.{}.{}-{}",
        &digits[..3],
        &digits[3..6],
        &digits[6..9],
        &digits[9..]
    )
}

#[component]
pub fn UserProfile(
    user: ReadSignal<Option<Value>>,
    open: ReadSignal<bool>,
    set_open: WriteSignal<bool>,
) -> impl IntoView {
    let close = move |_| set_open.set(false);

    view! {
        <Show when=move || open.get()>
            <div class="absolute right-3 top-full z-50 mt-2 w-80 max-w-[calc(100vw-1.5rem)] overflow-hidden rounded-lg border border-slate-200 bg-white shadow-xl">
                <div class="flex items-center justify-between bg-slate-50 px-4 py-3 border-b border-slate-200">
                    <h2 class="text-sm font-semibold text-slate-800">"Perfil do usuario"</h2>
                    <button
                        type="button"
                        class="flex h-8 w-8 items-center justify-center rounded-lg text-slate-500 hover:bg-slate-200 hover:text-slate-900"
                        aria-label="Fechar perfil"
                        on:click=close
                    >
                        "x"
                    </button>
                </div>

                {move || {
                    let user_data = user.get();

                    match user_data {
                        Some(user_value) => {
                            let name = read_text(&user_value, &["nome", "name", "usuario", "username"])
                                .unwrap_or_else(|| "Usuario".to_string());
                            let cpf = read_number_text(&user_value, &["cpf", "documento"]);
                            let matricula = read_number_text(&user_value, &["matricula", "registration"]);
                            let group_info = get_group_info(read_group(&user_value));
                            let initial = name
                                .chars()
                                .find(|char| !char.is_whitespace())
                                .map(|char| char.to_uppercase().collect::<String>())
                                .unwrap_or_else(|| "U".to_string());

                            view! {
                                <div class="p-4">
                                    <div class="flex items-center gap-3 border-b border-slate-100 pb-4">
                                        <div class="flex h-12 w-12 shrink-0 items-center justify-center rounded-full bg-sky-500 text-lg font-bold text-white">
                                            {initial}
                                        </div>
                                        <div class="min-w-0">
                                            <p class="truncate text-base font-semibold text-slate-900">{name}</p>
                                            <span class=format!("mt-1 inline-flex rounded-full px-2 py-0.5 text-xs font-semibold {}", group_info.color_class)>
                                                {group_info.label}
                                            </span>
                                        </div>
                                    </div>

                                    <div class="space-y-3 pt-4">
                                        <div>
                                            <p class="text-xs font-semibold uppercase text-slate-400">"CPF"</p>
                                            <p class="text-sm font-medium text-slate-800">
                                                {cpf.map(|value| format_cpf(&value)).unwrap_or_else(|| "Nao informado".to_string())}
                                            </p>
                                        </div>
                                        <div>
                                            <p class="text-xs font-semibold uppercase text-slate-400">"Matricula"</p>
                                            <p class="text-sm font-medium text-slate-800">
                                                {matricula.unwrap_or_else(|| "Nao informada".to_string())}
                                            </p>
                                        </div>
                                    </div>
                                </div>
                            }.into_any()
                        }
                        None => view! {
                            <div class="flex items-center gap-3 p-4 text-sm text-slate-600">
                                <Icon icon=icondata::FaCircleUserSolid width="1.4em" height="1.4em" />
                                "Dados do usuario indisponiveis"
                            </div>
                        }.into_any(),
                    }
                }}
            </div>
        </Show>
    }
}
