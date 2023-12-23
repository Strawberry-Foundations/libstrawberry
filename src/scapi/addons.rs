pub fn badge_handler(badge: &str) -> Option<String> {
    match badge.is_empty() {
        true => None,
        false => Some(format!(" [{}]", badge)),
    }
}
