use crate::utils::config;

pub const BASE_URL: &str = config::API_BASE_URL;

pub fn timeout_ms() -> u64 {
    config::api_timeout_ms()
}

pub fn url(endpoint: &str) -> String {
    config::api_url(endpoint)
}

pub mod endpoints {
    pub mod auth {
        pub const LOGIN: &str = "/auth/login";
        pub const REFRESH: &str = "/auth/refresh";
        pub const ME: &str = "/auth/me";
        pub const LOGOUT: &str = "/auth/logout";
    }

    pub mod funcionario {
        pub const LIST: &str = "/funcionario/";
        pub const CREATE: &str = "/funcionario/";

        pub fn get(id: impl std::fmt::Display) -> String {
            format!("/funcionario/{id}")
        }

        pub fn update(id: impl std::fmt::Display) -> String {
            format!("/funcionario/{id}")
        }

        pub fn delete(id: impl std::fmt::Display) -> String {
            format!("/funcionario/{id}")
        }
    }

    pub mod cliente {
        pub const LIST: &str = "/cliente/";
        pub const CREATE: &str = "/cliente/";

        pub fn get(id: impl std::fmt::Display) -> String {
            format!("/cliente/{id}")
        }

        pub fn update(id: impl std::fmt::Display) -> String {
            format!("/cliente/{id}")
        }

        pub fn delete(id: impl std::fmt::Display) -> String {
            format!("/cliente/{id}")
        }
    }

    pub mod produto {
        pub const PUBLIC_LIST: &str = "/produtos-publica/";
        pub const LIST: &str = "/produto/";
        pub const CREATE: &str = "/produto/";

        pub fn get(id: impl std::fmt::Display) -> String {
            format!("/produto/{id}")
        }

        pub fn update(id: impl std::fmt::Display) -> String {
            format!("/produto/{id}")
        }

        pub fn delete(id: impl std::fmt::Display) -> String {
            format!("/produto/{id}")
        }
    }

    pub mod comanda {
        pub const LIST: &str = "/comanda/";
        pub const CREATE: &str = "/comanda/";

        pub fn get(id: impl std::fmt::Display) -> String {
            format!("/comanda/{id}")
        }

        pub fn update(id: impl std::fmt::Display) -> String {
            format!("/comanda/{id}")
        }

        pub fn delete(id: impl std::fmt::Display) -> String {
            format!("/comanda/{id}")
        }

        pub fn cancel(id: impl std::fmt::Display) -> String {
            format!("/comanda/{id}/cancelar")
        }

        pub fn add_item(id: impl std::fmt::Display) -> String {
            format!("/comanda/{id}/produto")
        }

        pub fn list_items(id: impl std::fmt::Display) -> String {
            format!("/comanda/{id}/produtos")
        }

        pub fn update_item(id: impl std::fmt::Display) -> String {
            format!("/comanda/produto/{id}")
        }

        pub fn remove_item(id: impl std::fmt::Display) -> String {
            format!("/comanda/produto/{id}")
        }
    }

    pub mod caixa {
        pub const DASHBOARD: &str = "/caixa/dashboard";
        pub const SELECIONAR_COMANDAS: &str = "/caixa/comandas/selecionar";
    }

    pub mod recebimento {
        pub const LIST: &str = "/recebimento/";
        pub const CREATE: &str = "/recebimento/";

        pub fn get(id: impl std::fmt::Display) -> String {
            format!("/recebimento/{id}")
        }

        pub fn update(id: impl std::fmt::Display) -> String {
            format!("/recebimento/{id}")
        }

        pub fn delete(id: impl std::fmt::Display) -> String {
            format!("/recebimento/{id}")
        }

        pub fn comprovante(id: impl std::fmt::Display) -> String {
            format!("/recebimento/{id}/comprovante")
        }
    }

    pub mod auditoria {
        pub const LIST: &str = "/auditoria";
        pub const ACOES: &str = "/auditoria/acoes";
    }
}
