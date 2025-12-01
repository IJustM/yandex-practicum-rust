pub fn description_trim(value: &str) -> Result<String, ()> {
    if value.starts_with('"') && value.ends_with('"') {
        Ok(value.trim_matches('"').to_string())
    } else {
        Err(())
    }
}
