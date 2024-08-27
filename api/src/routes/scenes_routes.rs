use models::VideoScenes;
use rocket_contrib::json::Json;
use rocket::http::Status;

use crate::models::{self, VideoScene};

#[post("/scenes/<audio_id>", format = "json")]
pub fn get_scenes(audio_id: i32, state: rocket::State<'_, crate::AppState>) -> Result<Json<models::VideoScenes>, Status> {
    log::info!("Scenes for audio {}", audio_id);
    // Return the scenes data (for now, let's just return the same scenes as an example).
    let db_config = &state.db_config;
    let scenes_data = VideoScene::by_audio_id(audio_id, db_config);

    match scenes_data {
        Ok(scenes_data) => {
            log::info!("Scenes data: {:?}", scenes_data);
            let scenes = VideoScenes::new(scenes_data);
            log::info!("Scenes: {:?}", scenes);
            Ok(Json(scenes))
        }
        Err(e) => {
            log::info!("Error: {:?}", e);
            Err(Status::InternalServerError)
        }
    }

}
