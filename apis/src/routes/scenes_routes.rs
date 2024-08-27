use rocket_contrib::json::Json;
use rocket::http::Status;

use crate::models::{self, Scene};

#[post("/scenes/<track_id>", format = "json")]
pub fn scenes(track_id: i32, state: rocket::State<'_, crate::AppState>) -> Result<Json<models::Scenes>, Status> {
    println!("Scenes for track {}", track_id);
    // Return the scenes data (for now, let's just return the same scenes as an example).
    let scenes_data = Scene::load_by_track_id(track_id, state);

    match scenes_data {
        Ok(scenes_data) => {
            println!("Scenes data: {:?}", scenes_data);
            let scenes = models::Scenes {
                scenes: scenes_data,
            };
            Ok(Json(scenes))
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(Status::InternalServerError)
        }
    }

}
