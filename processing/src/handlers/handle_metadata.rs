
use models::{DatabaseConfig, AudioMetadata, Audio};

use crate::state_manager::{StateResult, StateResultType};



pub async fn handle_metadata_state(audio_id: i32, database_config: &DatabaseConfig) -> StateResult {
      let mut audio_metadata =  match AudioMetadata::by_audio_id(audio_id, database_config) {
            Ok(metadata) => metadata,
            _ => {
                let audio = match Audio::by_id(audio_id, database_config) {
                    Ok(audio) => audio,
                    Err(e) => {
                        return StateResult {
                            state: StateResultType::Failure,
                            message: format!("Failed to get audio {}: {}", audio_id, e),
                        }
                    }
                };
                let metadata = AudioMetadata::new(&audio, database_config);
                match metadata {
                    Ok(metadata) => metadata,
                    Err(e) => {
                        return StateResult {
                            state: StateResultType::Failure,
                            message: format!("Failed to create metadata for audio {}: {}", audio_id, e),
                        }
                    }
                }
            }
        };

    // if metadata bpm is not 0, then it is already filled in
    if audio_metadata.is_filled() {
        return StateResult {
            state: StateResultType::Success,
            message: format!("Metadata for audio {} is already filled in", audio_id),
        }
    } else {
        match audio_metadata.fill_in() {
            Ok(_) => {
                return StateResult {
                    state: StateResultType::Success,
                    message: format!("Metadata for audio {} is filled in", audio_id),
                }
            },
            Err(e) => {
                return StateResult {
                    state: StateResultType::Failure,
                    message: format!("Failed to fill in metadata for audio {}: {}", audio_id, e),
                }
            }
        }
    }

}


