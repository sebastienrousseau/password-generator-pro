#[tauri::command]
pub fn logger(time: &str, info: &str, message: &str, details: &str) {
    println!("🔒 {} - [{}] {}: {}", time, info, message, details);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger() {
        logger("2021-08-01 12:00:00", "INFO", "Test", "Test");
        assert!(true);
    }
}
