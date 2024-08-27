use r2d2_mysql::MysqlConnectionManager;
use r2d2::Pool;
use crate::{models::User, handlers::verify_password};

use std::error::Error;

pub fn save_user(
    db_pool: &Pool<MysqlConnectionManager>,
    user: &User,
) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
    let mut conn = db_pool.get()?;
    conn.prep_exec(
        "INSERT INTO USER_LIST (user_name, user_password) VALUES (?, ?)",
        user.get_credentials(),
    )?;
    Ok(())
}

pub fn validate_username_and_password(
    db_pool: &Pool<MysqlConnectionManager>,
    hash_secret: &str,
    username: &str,
    password: &str,
) -> std::result::Result<bool, Box<dyn Error>> {
    let user = find_user_by_username(db_pool, username)?;
    let verified = user
        .map(|u| verify_password(hash_secret, password, &u.get_credentials().1))
        .unwrap_or(Ok(false))?;
    Ok(verified) // This will return a Result type with a bool value
}

pub fn find_user_by_username(
    db_pool: &Pool<MysqlConnectionManager>,
    username: &str) -> std::result::Result<Option<User>, Box<dyn std::error::Error>> {
    let mut conn = db_pool.get()?;
    let mut users = conn.prep_exec("SELECT user_id, user_name, user_password FROM USER_LIST WHERE user_name = ?", (username,))?;

    if let Some(result) = users.next() {
        let row = result?;
        let user = User {
            user_id: row.get(0).unwrap(),
            user_name: row.get(1).unwrap(),
            user_password: row.get(2).unwrap(),
            user_token: String::from(""),
            user_email: String::from(""),
        };
        return Ok(Some(user));
    }

    Ok(None) // Return None if user is not found
}

pub fn check_user_exists(username: &str, db_pool: &Pool<MysqlConnectionManager>) -> std::result::Result<bool, Box<dyn Error>> {
    let mut conn = db_pool.get()?;
    let mut users = conn.prep_exec("SELECT user_id, user_name, user_password FROM USER_LIST WHERE user_name = ?", (username,))?;

    if let Some(result) = users.next() {
        let _row = result?;
        return Ok(true);
    }

    Ok(false) // Return false if user is not found
}