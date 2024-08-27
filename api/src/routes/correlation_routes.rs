
use models::AudioCorrelation;
use rocket_contrib::json::Json;
use rocket::http::Status;

use crate::app_state::AppState;

#[get("/correlation/<audio_id>")]
pub fn get_correlation(
    // token: UserAuthorizationToken, TODO: Add authentication
    state: rocket::State<'_, AppState>,
    audio_id: i32,
) -> std::result::Result<Json<models::AudioCorrelation>, Status> {

    // let correlation_data = match AudioCorrelation::get_correlation(audio_id, &state.db_config) {
    //     Some(correlation_data) => correlation_data,
    //     None => {
    //         return Err(Status::NotFound);
    //     }
    // };

    let correlation_data = match AudioCorrelation::get_correlation(audio_id, &state.db_config) {
        Ok(correlation_data) => correlation_data,
        Err(err) => {
            log::error!("Error getting correlation data: {}", err);
            return Err(Status::NotFound);
        }
    };

    match correlation_data {
        Some(correlation_data) => {
            return Ok(Json(correlation_data));
        },
        None => {
            return Err(Status::NotFound);
        }
    }
}

        