use bcrypt::{hash, BcryptError, DEFAULT_COST, verify};

pub fn hash_password(_hash_secret: &str, password: &str) -> std::result::Result<String, BcryptError> {
    hash(password, DEFAULT_COST).map_err(BcryptError::from)
}

pub fn verify_password(_hash_secret: &str, password: &str, hashed_password: &str) -> std::result::Result<bool, BcryptError> {
    Ok(verify(password, hashed_password)?)
}
