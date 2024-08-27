use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct UserCredentials {
    username: String,
    token: String,
    email: String,
}

impl UserCredentials {
    pub fn get_user_name(&self) -> String {
        self.username.clone()
    }

    pub fn get_token(&self) -> String {
        self.token.clone()
    }

    pub fn get_email(&self) -> String {
        self.email.clone()
    }
}