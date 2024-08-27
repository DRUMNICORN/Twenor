use models::{DatabaseConfig, AudioChunks, AudioMetadata};
use std::fs;
use log::info;

pub fn create_audio_directory(audio_id: i32, user_id: i32) -> Result<String, std::io::Error> {
    let audio_dir_path = format!("{}/{}/{}", ".db", user_id, audio_id);
    match fs::create_dir_all(&audio_dir_path){
        Ok(_) => {
            info!("Created directory {}", audio_dir_path);
            Ok(audio_dir_path)
        },
        Err(e) => {
            Err(e)
        }
    }
}

pub fn process_audio_chunks(
    audio_id: i32,
    audio_dir_path: &str,
    user_id: i32,
    database_config: &DatabaseConfig,
) -> Result<(), String> {
    match fs::read_dir(audio_dir_path) {
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
                                    return Err(format!("Failed to get metadata for audio {}: {}", audio_id, e));
                                }
                            };
                            
                            // Process audio chunks
                            let mut chunks = match AudioChunks::from_path(audio_path, metadata.get_bpm(), database_config) {
                                Ok(chunks) => chunks,
                                Err(e) => {
                                    return Err(format!("Failed to get chunks for audio {}: {}", audio_id, e));
                                }
                            };

                            if chunks.len() == 0 {
                                return Err(format!("No chunks found for audio {}", audio_id));
                            }

                            info!("Found {} chunks", chunks.len());

                            // Update chunks in the database
                            if let Err(e) = chunks.update(audio_id, user_id, database_config) {
                                return Err(format!("Failed to update chunks for audio {}: {}", audio_id, e));
                            }
                        } else {
                            return Err("Failed to get audio path".to_string());
                        }
                    },
                    Err(e) => {
                        return Err(format!("Failed to read directory entry: {}", e));
                    }
                }
            }
            info!("Done reading directory");
            Ok(())
        },
        Err(e) => {
            Err(format!("Failed to read directory: {}", e))
        }
    }
}
