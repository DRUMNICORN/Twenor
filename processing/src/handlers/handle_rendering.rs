use models::{AudioMetadata, VideoScenes, SceneConfig};
use std::fs;
use time::Instant;

use crate::{
    state_manager::{StateResult, StateResultType},
    utils::{calculate_eta, call_t2v_api},
};

pub async fn handle_rendering_state(
    audio_id: i32,
    database_config: &crate::db::DatabaseConfig,
) -> StateResult {
    log::info!("Rendering state for audio_id: {}", audio_id);

    // Load script, scenes, and metadata
    // let script = match VideoScript::by_audio_id(audio_id, database_config) {
    //     Ok(script) => match script {
    //         Some(script) => script,
    //         None => {
    //             return StateResult {
    //                 state: StateResultType::Failure,
    //                 message: format!("No script found for audio {}", audio_id),
    //             }
    //         }
    //     },
    //     Err(e) => {
    //         return StateResult {
    //             state: StateResultType::Failure,
    //             message: format!("Failed to get script for audio {}: {}", audio_id, e),
    //         }            
    //     }
    // };
    
    let scenes = match VideoScenes::by_audio_id(audio_id, database_config){
        Ok(scenes) => scenes,
        Err(e) => {
            return StateResult {
                state: StateResultType::Failure,
                message: format!("Failed to get scenes for audio {}: {}", audio_id, e),
            };
        }
    };
    let metadata = match AudioMetadata::by_audio_id(audio_id, database_config){
        Ok(metadata) => metadata,
        Err(e) => {
            return StateResult {
                state: StateResultType::Failure,
                message: format!("Failed to get metadata for audio {}: {}", audio_id, e),
            };
        }
    };

    // Generate and save scene configurations
    let configs: Vec<SceneConfig> = SceneConfig::compute(&scenes, &metadata);
    // let choice_title = script.get_id().to_string();

    match SceneConfig::insert(&configs, metadata.get_user_id(), metadata.get_audio_id() ,"data.json") {
        Ok(_) => {}
        Err(e) => {
            return StateResult {
                state: StateResultType::Failure,
                message: format!("Failed to insert scene configs for audio {}: {}", audio_id, e),
            };
        }
    };

    // Start rendering videos
    let start_time = Instant::now();
    for (index, config) in configs.iter().enumerate() {
        let file_path = format!("{}/{}/{}/{}.mp4", ".db", metadata.get_user_id(), metadata.get_audio_id(), index);

        // Check if the video file already existshttp://localhost
        if fs::metadata(&file_path).is_ok() {
            continue;
        } else {
        }

        match call_t2v_api(config, &metadata, index).await {
            Ok(_) => {}
            Err(e) => {
                return StateResult {
                    state: StateResultType::Failure,
                    message: format!("Failed to render video for audio {}: {}", audio_id, e),
                };
            }
        }
        let eta = calculate_eta(start_time, index, configs.len());
        log::info!("ETA: {} seconds", eta);
        break;
    }

    return StateResult {
        state: StateResultType::Success,
        message: format!("Done handling rendering state for audio {}", audio_id),
    };
}
