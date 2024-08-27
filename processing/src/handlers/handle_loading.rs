use models::{DatabaseConfig, Audio, AudioMetadata, AudioChunks};
use crate::state_manager::{StateResult, StateResultType};
use std::fs;

pub async fn handle_convert_state(audio_id: i32, database_config: &DatabaseConfig) -> StateResult {
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
    let audio_dir_path = format!("{}/{}/{}", ".db", user_id, audio_id);

    // Create the directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(&audio_dir_path) {
        return StateResult {
            state: StateResultType::Failure,
            message: format!("Failed to create directory: {}", e),
        }
    }

    // Read the directory entries
    match fs::read_dir(&audio_dir_path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if let Some(audio_path) = path.to_str() {
                            // Retrieve audio metadata
                            let metadata = match AudioMetadata::by_audio_id(audio_id, database_config) {
                                Ok(metadata) => metadata,
                                Err(e) => {
                                    return StateResult {
                                        state: StateResultType::Failure,
                                        message: format!("Failed to get metadata for audio {}: {}", audio_id, e),
                                    }
                                }
                            };

                            // Process audio chunks
                            let mut chunks = match AudioChunks::from_path(audio_path, metadata.get_bpm(), database_config) {
                                Ok(chunks) => chunks,
                                Err(e) => {
                                    return StateResult {
                                        state: StateResultType::Failure,
                                        message: format!("Failed to get chunks for audio {}: {}", audio_id, e),
                                    }
                                }
                            };

                            if chunks.len() == 0 {
                                return StateResult {
                                    state: StateResultType::Failure,
                                    message: format!("No chunks found for audio {}", audio_id),
                                }
                            }

                            log::info!("Found {} chunks", chunks.len());

                            // Update chunks in the database
                            match chunks.update(audio_id, user_id, database_config) {
                                Ok(_) => {
                                    return StateResult {
                                        state: StateResultType::Success,
                                        message: format!("Done handling loading state for audio {}", audio_id),
                                    }
                                },
                                Err(e) => {
                                    return StateResult {
                                        state: StateResultType::Failure,
                                        message: format!("Failed to update chunks for audio {}: {}", audio_id, e),
                                    }
                                }
                            }
                        } else {
                            return StateResult {
                                state: StateResultType::Failure,
                                message: format!("Failed to get audio path"),
                            }
                        }
                    },
                    Err(e) => {
                        return StateResult {
                            state: StateResultType::Failure,
                            message: format!("Failed to read directory entry: {}", e),
                        }
                    }
                }
            }
            log::info!("Done reading directory");
        },
        Err(e) => {
            return StateResult {
                state: StateResultType::Failure,
                message: format!("Failed to read directory: {}", e),
            }
        }
    }

    return StateResult {
        state: StateResultType::Success,
        message: format!("Done handling loading state for audio {}", audio_id),
    }
}
