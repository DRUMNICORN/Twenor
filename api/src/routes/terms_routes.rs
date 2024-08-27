use models::User;

use crate::app_state::AppState;

#[post("/terms/<user_id>/accept")]
pub fn post_terms_accept(
    user_id: i32,
    state: rocket::State<'_, AppState>,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut user = User::by_id(user_id, &state.db_config)?;
    user.accept_terms(&state.db_config)?;

    Ok(())
}

#[get("/terms/<user_id>")]
pub fn get_terms(
    user_id: i32,
    state: rocket::State<'_, AppState>,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let user = User::by_id(user_id, &state.db_config)?;
    let terms = user.get_terms(&state.db_config)?.to_string();

    Ok(terms)
}