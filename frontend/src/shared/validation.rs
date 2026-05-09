// src/validation.rs

// Centralizamos as regras aqui. Se um dia a regra da senha mudar para 8 caracteres,
// mudamos apenas neste arquivo e todo o sistema (DRY) é atualizado.

pub fn validar_nome(nome: &str) -> Result<(), &'static str> {
    if nome.trim().is_empty() {
        Err("Nome é obrigatório")
    } else {
        Ok(())
    }
}

pub fn validar_cpf(cpf: &str) -> Result<(), &'static str> {
    if cpf.trim().is_empty() {
        Err("CPF é obrigatório")
    } else {
        // Aqui no futuro você pode colocar uma lógica real de cálculo de CPF
        Ok(())
    }
}

pub fn validar_senha(senha: &str) -> Result<(), &'static str> {
    if senha.is_empty() {
        return Err("Senha é obrigatória");
    }
    if senha.len() < 6 {
        return Err("Senha deve ter pelo menos 6 caracteres");
    }
    Ok(())
}

pub fn validar_valor_unitario(valor: f64) -> Result<(), &'static str> {
    if valor <= 0.0 {
        Err("Valor deve ser maior que 0")
    } else {
        Ok(())
    }
}