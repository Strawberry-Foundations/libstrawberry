pub fn badge_handler(badge: &str) -> String {
    if badge != "" {
        format!(" [{}]", badge)
    } else {
        "".to_string()
    }
}