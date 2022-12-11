use time::OffsetDateTime;

#[tauri::command]
pub fn return_date_time() -> String {
    OffsetDateTime::now_utc().to_string()
}


#[tauri::command]
pub fn logger(info: &str, message: &str, details: &str) {
    println!("🔒 {} - [{}] {}: {}", return_date_time(), info, message, details);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger() {
        logger("INFO", "Test", "Test");
    }
}
