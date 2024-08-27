use r2d2::Pool;
use r2d2_mysql::MysqlConnectionManager;
use serde::{Deserialize, Serialize};
use std::{env, error::Error};

use crate::handlers::{hash_password, find_user_by_username, verify_token, generate_token, check_user_exists, save_user, validate_username_and_password};

use super::{Db, User, UserCredentials};

#[derive(Debug, Serialize, Deserialize)]
struct TokenClaims {
    sub: String,
    exp: usize,
}

pub struct AppState {
    pub db: Db,
    pub jwt_secret: String,
    pub hash_secret: String,
}

impl<'a> AppState {
    pub fn new() -> Self {
        let db = Db::new();

        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not found in the environment");
        let hash_secret =
            env::var("HASH_SECRET").expect("HASH_SECRET not found in the environment");

        AppState {
            db,
            jwt_secret,
            hash_secret,
        }
    }

    pub fn db_pool(&self) -> &Pool<MysqlConnectionManager> {
        &self.db.pool
    }

    pub fn get_user_id_by_token(
        &self,
        token: &str,
    ) -> std::result::Result<Option<i32>, Box<dyn Error>> {
        let claims = verify_token(&self.jwt_secret, token)?;
        let username = claims.sub;
        if let Some(user) = find_user_by_username(self.db_pool(), &username)? {
            Ok(Some(user.user_id))
        } else {
            Ok(None)
        }
    }

    pub fn generate_token(&self, c: UserCredentials) -> String {
        let user_exists = check_user_exists(&c.username, self.db_pool()).unwrap();
    
        let token: Result<_, jsonwebtoken::errors::Error> = generate_token(&self.jwt_secret, &c.username);
        if let Err(_) = token {
            return String::from("Failed to generate token");
        }
    
        let token = token.unwrap();
    
        if !user_exists {
            // If the user doesn't exist, register a new user
            let hashed_password = match hash_password(&self.hash_secret, &c.password) {
                Ok(hashed) => hashed,
                Err(err) => return format!("Error hashing password: {:?}", err),
            };
            let user = User::new(&c.username, &hashed_password, &token);
            match save_user(self.db_pool(), &user) {
                Ok(_) => (),
                Err(err) => return format!("Error saving user: {:?}", err),
            };
            generate_token(&self.jwt_secret, &c.username).unwrap()
        } else {
            // If the user exists, validate the password
            let is_valid = validate_username_and_password(self.db_pool(), &self.hash_secret, &c.username, &c.password).unwrap();
            if !is_valid {
                return String::from("Incorrect password");
            }
            token
        }
    }  
}
