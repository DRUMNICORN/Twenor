
use rocket_contrib::json::Json;
use rocket::http::Status;

use crate::{models::{self, AppState, AuthorizationHeaderToken}, handlers::get_correlation};

#[get("/correlation/<track_id>")]
pub fn correlation(
    token: AuthorizationHeaderToken,
    state: rocket::State<'_, AppState>,
    track_id: i32,
) -> std::result::Result<Json<models::Correlation>, Status> {
    println!("Correlation for track {}", track_id);
        // Return the scenes data (for now, let's just return the same scenes as an example).
    

    let correlation_data = get_correlation(track_id, state)
        .map_err(|error| {
            println!("Error: {:?}", error);
            Status::InternalServerError
        })?;

    Ok(Json(correlation_data))
}

        