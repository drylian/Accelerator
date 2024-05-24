pub fn dev() -> bool {
    let environment = std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());
    match environment.as_str() {
        "development" => {
            return true;
        }
        _ => {
            return false;
        }
    }
}