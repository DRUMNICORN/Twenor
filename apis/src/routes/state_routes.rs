#[get("/trackstate/<track_id>")]
pub fn get_state(track_id: i32, state: rocket::State<'_, crate::models::AppState>) -> Result<String, Box<dyn std::error::Error>> {
    let track_state = crate::handlers::get_track_state(track_id, &state)?;
    Ok(track_state.as_str().to_string())
}
