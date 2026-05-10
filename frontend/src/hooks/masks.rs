pub fn only_digits(value: &str) -> String {
    value.chars().filter(|char| char.is_ascii_digit()).collect()
}

pub fn apply_cpf_mask(value: &str) -> String {
    let digits = only_digits(value);
    let digits = digits.chars().take(11).collect::<String>();
    let len = digits.len();

    if len <= 3 {
        return digits;
    }

    if len <= 6 {
        return format!("{}.{}", &digits[..3], &digits[3..]);
    }

    if len <= 9 {
        return format!("{}.{}.{}", &digits[..3], &digits[3..6], &digits[6..]);
    }

    format!(
        "{}.{}.{}-{}",
        &digits[..3],
        &digits[3..6],
        &digits[6..9],
        &digits[9..]
    )
}

pub fn apply_phone_mask(value: &str) -> String {
    let digits = only_digits(value);
    let digits = digits.chars().take(11).collect::<String>();
    let len = digits.len();

    if len <= 2 {
        return digits;
    }

    if len <= 6 {
        return format!("({}) {}", &digits[..2], &digits[2..]);
    }

    if len <= 10 {
        return format!("({}) {}-{}", &digits[..2], &digits[2..6], &digits[6..]);
    }

    format!("({}) {}-{}", &digits[..2], &digits[2..7], &digits[7..])
}
