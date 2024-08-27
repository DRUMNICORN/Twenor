// Module defining the User struct


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub user_name: String,
    pub user_password: String,
    pub user_token: String,
    pub user_email: String,
}

impl User {
    pub fn new(username: &str, hashed_password: &str, token: &str) -> Self {
        User {
            user_id: 0,
            user_name: username.to_string(),
            user_password: hashed_password.to_string(),
            user_token: token.to_string(),
            user_email: "".to_string(),
        }
    }

    pub fn get_credentials(&self) -> (String, String) {
        (self.user_name.clone(), self.user_password.clone())
    }
}


use rocket::{request::{self, FromRequest, Request}, http::Status};
use serde::{Deserialize, Serialize};
pub struct AuthorizationHeaderToken {
   pub id: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UserAuthenticated {
    pub token: String,
    pub name: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UserCredentials {
    pub username: String,
    pub password: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthorizationHeaderToken {
   type Error = std::io::Error;

   fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthorizationHeaderToken, Self::Error> {
       let keys: Vec<_> = request.headers().get("Authorization").collect();
       if keys.len() != 1 {
           return request::Outcome::Failure((
               Status::BadRequest,
               std::io::Error::new(std::io::ErrorKind::Other, "No authorization"),
           ));
       }
       let authorization_header = keys[0];
       let token = match authorization_header.strip_prefix("Bearer ") {
           Some(key) => key.to_string(),
           None => authorization_header.to_string(),
       };
       if token == "undefined" {
           return request::Outcome::Failure((
               Status::BadRequest,
               std::io::Error::new(std::io::ErrorKind::Other, "No authorization"),
           ));
       }
       request::Outcome::Success(AuthorizationHeaderToken { id: token }) // TODO: Check if token is valid
   }
}

