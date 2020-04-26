pub fn require(value: &str, name: &str) -> Option<String> {
    if value.chars().count() == 0 {
        Some(format!("{}を入力してください。", name))
    } else {
        None
    }
}

pub fn enumrate<T: std::str::FromStr>(value: &str, name: &str) -> Option<String> {
    match T::from_str(value) {
        Ok(_) => None,
        Err(_) => Some(format!("{}を入力してください。", name)),
    }
}
