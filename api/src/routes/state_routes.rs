use crate::{app_state::AppState, models::AudioState};

#[get("/audiostate/<audio_id>")]
pub fn get_state(
    audio_id: i32,
    state: rocket::State<'_, AppState>,
) -> Result<String, Box<dyn std::error::Error>> {
    let audio_state = match AudioState::by_id(audio_id, &state.db_config) {
        Ok(state) => state,
        Err(e) => {
            return Err(e);
        }
    };

    let audio_state = audio_state.as_str();
    Ok(audio_state.to_string())
}
