use bcrypt::{hash,  BcryptError, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use models::DatabaseConfig;
use serde::{Deserialize, Serialize};
use std::{env, result::Result};

struct TokenContainer {
    jwt_secret: String,
}

impl TokenContainer {
    pub fn new(jwt_secret: String) -> Self {
        TokenContainer { jwt_secret }
    }

    pub fn generate_token(&self, username: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let expiration = Utc::now() + Duration::days(1);

        let claims = TokenClaims {
            sub: username.to_string(),
            exp: expiration.timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
    }

    fn _verify_token(&self, token: &str) -> Result<TokenClaims, jsonwebtoken::errors::Error> { // TODO: Return a custom error type
        decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map(|data| data.claims)
    }
}


#[derive(Debug, Serialize, Deserialize)]
struct TokenClaims {
    sub: String,
    exp: usize,
}


pub struct AppState {
    pub db_config: DatabaseConfig,
    pub hash_secret: String,
    token_container: TokenContainer,
}

impl<'a> AppState {
    pub fn new() -> Self {
        // Load environment variables from .env file

        let db_config = match DatabaseConfig::new() {
            Ok(db_config) => db_config,
            Err(err) => panic!("Failed to create database config: {}", err),
        };

        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let hash_secret = env::var("HASH_SECRET").expect("HASH_SECRET must be set");

        let token_container = TokenContainer::new(jwt_secret.clone()); // Clone the jwt_secret

        AppState {
            db_config,
            hash_secret,
            token_container,
        }
    }

    pub fn hash_password(&self, password: &str) -> Result<String, BcryptError> {
        hash(password, DEFAULT_COST).map_err(BcryptError::from)
    }

    pub fn generate_token(&self, username: &str) -> Result<String, jsonwebtoken::errors::Error> {
        self.token_container.generate_token(username)
    }
}
