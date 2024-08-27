use crate::app_state::AppState;
use models::{User, UserAuthenticated, UserCredentials};
use rocket_contrib::json::Json;

#[post("/user", format = "json", data = "<credentials>")]
pub fn user(
    credentials: Json<UserCredentials>,
    state: rocket::State<'_, AppState>,
) -> Result<String, String> {
    let c = credentials.into_inner();

    let token = state.generate_token(&c.get_user_name())
        .map_err(|err| format!("Error generating token: {:?}", err))?;

    let user_acc = match User::from_user_name(&c.get_user_name(), &state.db_config) {
        Ok(Some(mut user)) => {
            match user.refresh_token(&token, &state.db_config){
                Ok(_) => {
                    log::debug!("User exists, refreshing token");
                    user
                }
                Err(err) => {
                    return Err(format!("Error: {}", err));
                }
            }
        }
        Ok(None) => {
            let access_token = state.hash_password(&c.get_token())
            .map_err(|err| format!("Error hashing password: {:?}", err))?;
            log::debug!("User does not exist, creating new user");
            User::create_new_user(&c.get_user_name(), &token, &c.get_email(), &access_token, &state.db_config)
        }
        Err(err) => {
            return Err(format!("Error: {}", err));
        }
    };

    let user_id = user_acc.id();
    let user_authenticated = UserAuthenticated::new(token, user_acc.get_user_name(), user_id);
    serde_json::to_string(&user_authenticated)
        .map_err(|err| format!("Error serializing user: {:?}", err))
}




