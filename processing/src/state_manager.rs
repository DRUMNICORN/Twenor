use crate::handlers::*;
use models::{AudioState, DatabaseConfig};

pub struct StateResult {
    pub state: StateResultType,
    pub message: String,
}


#[derive(Clone)]
pub enum StateResultType {
    Success, // Process is done and can move to the next state
    Pending, // Process is not done and should be retried, state stays the same
    Failure, // Process failed and should be retried, state back to Queded State

}

pub struct StateManager {
    database_config: DatabaseConfig,
}

impl StateManager {
    pub fn new(database_config: DatabaseConfig) -> Self {
        Self { database_config }
    }

    pub async fn handle_state_transition(&self, state: AudioState, audio_id: i32) {
        let result = match state {
            AudioState::Rendering => handle_rendering_state(audio_id, &self.database_config).await,
            AudioState::Prompting => handle_prompt_state(audio_id, &self.database_config).await,
            AudioState::Writing => handle_writing_state(audio_id, &self.database_config).await,
            AudioState::Splitting => handle_scene_state(audio_id, &self.database_config).await,
            AudioState::Correlating => handle_correlation_state(audio_id, &self.database_config).await,
            AudioState::Featuring => handle_features_state(audio_id, &self.database_config).await,
            AudioState::Chunking => handle_chunking_state(audio_id, &self.database_config).await,
            AudioState::Converting => handle_convert_state(audio_id, &self.database_config).await,
            AudioState::Describing => handle_metadata_state(audio_id, &self.database_config).await,
            AudioState::Uploading => handle_uploaded_state(audio_id, &self.database_config).await,

            AudioState::Error => {
                StateResult {
                    state: StateResultType::Failure,
                    message: "Error state reached".to_string(),
                }                
            },

            AudioState::Done => {
                StateResult {
                    state: StateResultType::Pending,
                    message: "Done wait for changes".to_string(),
                }
            },

            // _ => {
            //     StateResult {
            //         state: StateResultType::Failure,
            //         message: "Unknown state reached".to_string(),
            //     }
            // }
        };

        
        match result.state {
            StateResultType::Success => {
                log::info!("Audio {} is done, {}", audio_id, result.message);
                match self.next_state(state.next_state(), audio_id, &result.message) {
                    Ok(_) => {},
                    Err(e) => log::error!("Failed to update audio state for audio {}: {}", audio_id, e),
                };
            }
            StateResultType::Pending => {
                log::debug!("Audio {} is pending, {}", audio_id, result.message);
            }
            StateResultType::Failure => {
                log::error!("Audio {} failed because: {}", audio_id, result.message);
                match self.next_state(AudioState::Error, audio_id, &result.message) {
                    Ok(_) => {},
                    Err(e) => log::error!("Failed to update audio state for audio {}: {}", audio_id, e),
                };
            }
        }

    }

    fn next_state(&self, current_state: AudioState, audio_id: i32, message: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        match AudioState::update(audio_id, current_state.clone(), message, 0.0, &self.database_config) {
            Ok(_) => {},
            Err(e) => {
                return Err(e);
            }
        };
        // TODO: handle_removing_state(audio_id, current_state.clone(), &self.database_config);
        Ok(())
    }
}
