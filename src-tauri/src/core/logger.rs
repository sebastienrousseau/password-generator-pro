
pub fn logger (
    time: &str, info: &str, message: &str, details: &str
)
{
    println!("🔒 {} - [{}] {}: {}", time, info, message, details);
}
