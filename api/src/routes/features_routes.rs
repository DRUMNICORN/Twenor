
use models::AudioFeatures;
use rocket_contrib::json::Json;
use rocket::http::Status;

use crate::app_state::AppState;

#[get("/features/<audio_id>")]
pub fn get_features(
    // token: UserAuthorizationToken, TODO: Add authentication
    state: rocket::State<'_, AppState>,
    audio_id: i32,
) -> std::result::Result<Json<models::AudioFeatureList>, Status> {
    log::info!("Features for audio {}", audio_id);
    let db_config = &state.db_config;
    let features_data = match AudioFeatures::by_audio_id(audio_id, db_config) {
        Ok(features_data) => features_data,
        Err(e) => {
            log::info!("Failed to get features for audio {}: {}", audio_id, e);
            return Err(Status::InternalServerError);
        }
    };
    Ok(Json(features_data))
}

        