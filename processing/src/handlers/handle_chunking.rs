use models::{DatabaseConfig, Audio, AudioChunks};
use crate::{state_manager::{StateResult, StateResultType}, utils::{create_audio_directory, process_audio_chunks}};

pub async fn handle_chunking_state(audio_id: i32, database_config: &DatabaseConfig) -> StateResult {
    let audio = match Audio::by_id(audio_id, database_config) {
        Ok(audio) => audio,
        Err(e) => {
            return StateResult {
                state: StateResultType::Failure,
                message: format!("Failed to get audio {}: {}", audio_id, e),
            }
        }
    };

    let user_id: i32 = audio.get_user_id();

    // if let Err(e) = create_audio_directory(audio_id, user_id) {
    //     return StateResult {
    //         state: StateResultType::Failure,
    //         message: format!("Failed to create directory: {}", e),
    //     }
    // }

    let audio_dir_path = match create_audio_directory(audio_id, user_id) {
        Ok(audio_dir_path) => audio_dir_path,
        Err(e) => {
            return StateResult {
                state: StateResultType::Failure,
                message: format!("Failed to create directory: {}", e),
            }
        }
    };

    // check if there are already chunks
    let chunks = match AudioChunks::by_audio_id(audio_id, database_config) {
        Ok(chunks) => chunks,
        Err(e) => {
            return StateResult {
                state: StateResultType::Failure,
                message: format!("Failed to get chunks for audio {}: {}", audio_id, e),
            }
        }
    };

    if chunks.len() > 0 {
        return StateResult {
            state: StateResultType::Success,
            message: format!("Done handling loading state for audio {}", audio_id),
        }
    }

    // Read the directory entries and process audio chunks
    if let Err(e) = process_audio_chunks(audio_id, &audio_dir_path, user_id, database_config) {
        return StateResult {
            state: StateResultType::Failure,
            message: format!("Failed to process audio chunks for audio {}: {}", audio_id, e),
        }
    }

    return StateResult {
        state: StateResultType::Success,
        message: format!("Done handling loading state for audio {}", audio_id),
    }
}
