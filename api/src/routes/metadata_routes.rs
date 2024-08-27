use crate::{app_state::AppState, models::audio_metadata};
use audio_metadata::AudioMetadata;
use models::{Audio, AudioState};
use rocket_contrib::json::Json;

#[get("/metadata/<audio_id>")]
pub fn get_metadata(
    audio_id: i32,
    state: rocket::State<'_, AppState>,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    log::info!("get_metadata: audio_id: {}", audio_id);
    let database_config = &state.db_config;

    log::info!("get_metadata: getting metadata");
    let metadata = match AudioMetadata::by_audio_id(audio_id, database_config) {
        Ok(metadata) => metadata,
        Err(_e) => {
            let audio = Audio::by_id(audio_id, database_config)?;
            let metadata = AudioMetadata::new(&audio, database_config)?;
            metadata
        }
    };

    log::info!("get_metadata: metadata: {:?}", metadata);
    let json = serde_json::to_string(&metadata)?;
    Ok(json)
}

#[post("/metadata/<audio_id>", format = "json", data = "<metadata>")]
pub fn post_metadata(
    audio_id: i32,
    metadata: Json<AudioMetadata>,
    state: rocket::State<'_, AppState>,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut audio_metadata: AudioMetadata =
        match AudioMetadata::by_audio_id(audio_id, &state.db_config) {
            Ok(metadata) => metadata,
            _ => {
                let audio = Audio::by_id(audio_id, &state.db_config)?;
                let metadata = AudioMetadata::new(&audio, &state.db_config)?;
                metadata
            }
        };

    audio_metadata.update_metadata(metadata.into_inner(), &state.db_config)?;
    let json = serde_json::to_string(&audio_metadata)?;    
    
    // let database_config = &state.db_config;
    // AudioState::reset_state(audio_id, database_config)?;

    Ok(json)
}
