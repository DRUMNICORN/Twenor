use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    exp: usize,
}

pub fn generate_token(
    jwt_secret: &str,
    username: &str,
) -> std::result::Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now() + Duration::days(1);

    let claims = TokenClaims {
        sub: username.to_string(),
        exp: expiration.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
}

pub fn verify_token(
    jwt_secret: &str,
    token: &str,
) -> std::result::Result<TokenClaims, jsonwebtoken::errors::Error> {
    decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
}
