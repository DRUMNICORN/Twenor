use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct UserAuthenticated {
    token: String,
    name: String,
    id: i32,
}

impl UserAuthenticated {
    pub fn new(token: String, name: String, id: i32) -> UserAuthenticated {
        let user = UserAuthenticated {
            token,
            name,
            id,
        };
        user
    }

    pub fn get_token(&self) -> String {
        self.token.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }
}