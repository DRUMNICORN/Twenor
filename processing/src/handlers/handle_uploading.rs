use models::{Audio, DatabaseConfig};

use crate::state_manager::{StateResult, StateResultType};

pub async fn handle_uploaded_state(audio_id: i32, database_config: &DatabaseConfig) -> StateResult {
    let audio = match Audio::by_id(audio_id, database_config) {
        Ok(audio) => audio,
        Err(e) => {
            return StateResult {
                state: StateResultType::Pending,
                message: format!("Failed to get audio {}: {}", audio_id, e),
            };
        }
    };

    if audio.is_loaded() {
        return StateResult {
            state: StateResultType::Success,
            message: format!("Audio {} is loaded", audio_id),
        };
    } else {
        return StateResult {
            state: StateResultType::Pending,
            message: format!("Audio {} is not loaded", audio_id),
        };
    }
}
