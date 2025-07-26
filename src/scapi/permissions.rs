pub enum PermissionLevel {
    Custom,
    All,
    Trusted,
    Admin,
    Owner,
}

pub struct PermissionList {
    pub trusted: Vec<String>,
    pub admin: Vec<String>,
    pub custom: Vec<String>,
    pub owner: String,
}
