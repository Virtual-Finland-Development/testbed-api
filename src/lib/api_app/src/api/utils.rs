pub fn get_stage() -> String {
    std::env::var("STAGE").unwrap_or_else(|_| "local".to_string())
}
