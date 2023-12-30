pub fn badge_handler(badge: &str) -> Option<String> {
    if badge.is_empty() {
        None
    }
    else {
        Some(format!(" [{badge}]"))
    }
}
