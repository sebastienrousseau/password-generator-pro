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
        logger("2021-08-01 12:00:00", "INFO", "Test", "Test");
    }
}
