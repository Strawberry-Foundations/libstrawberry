pub struct Credentials {
    pub email: String,
    pub password: String,
}

impl Credentials {
    pub fn new(email: impl ToString, password: impl ToString) -> Self {
        Self {
            email: email.to_string(),
            password: password.to_string(),
        }
    }
}
