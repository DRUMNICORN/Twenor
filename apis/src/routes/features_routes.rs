
use rocket_contrib::json::Json;
use rocket::http::Status;

use crate::{models::{self, AppState, AuthorizationHeaderToken}, handlers::get_features};

#[get("/features/<track_id>")]
pub fn features(
    token: AuthorizationHeaderToken,
    state: rocket::State<'_, AppState>,
    track_id: i32,
) -> std::result::Result<Json<models::FeaturesPackage>, Status> {
    println!("Features for track {}", track_id);
        // Return the scenes data (for now, let's just return the same scenes as an example).
    

    let features_data = get_features(track_id, state)
        .map_err(|error| {
            println!("Error: {:?}", error);
            Status::InternalServerError
        })?;

    Ok(Json(features_data))
}

        