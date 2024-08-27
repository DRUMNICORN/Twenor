use rocket_contrib::json::Json;
use crate::models::{AppState, user::{UserCredentials, UserAuthenticated}};


// Define the signin route
#[post("/user", format = "json", data = "<credentials>")]
pub fn user(credentials: Json<UserCredentials>, state: rocket::State<'_, AppState>) -> String {
    let c = credentials.into_inner();
    let user_name = c.username.clone();
    let token = state.generate_token(c);
    let user_authenticated = UserAuthenticated {token, name: user_name};
    serde_json::to_string(&user_authenticated).unwrap()
}

