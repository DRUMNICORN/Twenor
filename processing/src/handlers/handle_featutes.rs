use models::{
    Audio, AudioChunk, AudioChunks, AudioFeatures, DatabaseConfig, MAX_FEATURES_PER_REQUEST,
};

use crate::state_manager::{StateResult, StateResultType};
use crate::utils::compute_audio_features;

pub async fn handle_features_state(audio_id: i32, database_config: &DatabaseConfig) -> StateResult {
    let audio = match Audio::by_id(audio_id, database_config) {
        Ok(audio) => audio,
        Err(e) => return error_result(format!("Failed to get audio {}: {}", audio_id, e)),
    };
    let audio_id = audio.get_audio_id();

    let chunks: Vec<AudioChunk> = match AudioChunks::by_audio_id(audio_id, database_config) {
        Ok(chunks) => chunks,
        Err(e) => {
            return error_result(format!(
                "Failed to get chunks for audio {}: {}",
                audio_id, e
            ))
        }
    };

    let already_generated_chunks = match AudioFeatures::by_audio_id(audio_id, database_config) {
        Ok(features) => features.to_vec().len() as i32,
        Err(e) => {
            return error_result(format!(
                "Failed to get features for audio {}: {}",
                audio_id, e
            ))
        }
    };

    if chunks.is_empty() {
        return StateResult {
            state: StateResultType::Failure,
            message: format!("No chunks found for audio {}", audio_id),
        };
    }

    let generated_chunks: i32 = process_chunks(audio_id, chunks.clone(), already_generated_chunks, database_config);

    if generated_chunks == chunks.len() as i32 {
        return finalize_features(audio_id, database_config);
    } else {
        return StateResult {
            state: StateResultType::Pending,
            message: format!("Not all chunks have been generated, waiting for the rest"),
        };
    }
}

fn process_chunks(
    audio_id: i32,
    chunks: Vec<AudioChunk>,
    already_generated_chunks: i32,
    database_config: &DatabaseConfig,
) -> i32 {
    let mut generated_chunks = 0;
    let mut current_generated_chunks = 0;

    
    for chunk in &chunks {
        let (start, end, index) = chunk.get_start_end_index();

        if (index as i32) < already_generated_chunks {
            generated_chunks += 1;
            continue;
        } else if (index as i32) == already_generated_chunks {
            log::debug!("Continue from chunk {}", index);
        }

        if current_generated_chunks >= MAX_FEATURES_PER_REQUEST {
            log::info!(
                "Generated chunks: {}, Total chunks: {}/{}",
                current_generated_chunks,
                generated_chunks,
                chunks.len()
            );
            break;
        }

        let data: Vec<f64> = chunk.get_chunk_values().iter().map(|&x| x as f64).collect();
        let data: &[f64] = &data;

        let chunks_count = chunks.len() as f64;
        let chunk_index = index as f64;
        let chunk_percentage = chunk_index / chunks_count;
        let chunk_percentage = (chunk_percentage * 100.0).round();

        log::debug!(
            "Chunk: {}/{} ({}%)",
            chunk.get_chunk_index() + 1,
            chunks.len(),
            chunk_percentage
        );

        let chunk_features = match compute_audio_features(data) {
            Ok(features) => features.into_features(-1, audio_id, chunk.get_chunk_id()),
            Err(e) => {
                log::error!("Failed to compute audio features: {}", e);
                continue;
            }
        };

    }
    return generated_chunks;
}

fn finalize_features(audio_id: i32, database_config: &DatabaseConfig) -> StateResult {
    log::info!("All chunks have been generated");

    match AudioFeatures::by_audio_id(audio_id, database_config) {
        Ok(features) => {
            let features = features.to_vec();
            let normalized = AudioFeatures::normalize(features);
            match AudioFeatures::clear(audio_id, database_config) {
                Ok(_) => {}
                Err(e) => {
                    return error_result(format!(
                        "Failed to clear features for audio {}: {}",
                        audio_id, e
                    ))
                }
            };

            for feature in normalized.iter() {
                match feature.save(database_config) {
                    Ok(_) => {}
                    Err(e) => {
                        return error_result(format!(
                            "Failed to save features for audio {}: {}",
                            audio_id, e
                        ))
                    }
                }
            }

            StateResult {
                state: StateResultType::Success,
                message: format!("Done handling features state for audio {}", audio_id),
            }
        }
        Err(e) => {
            return error_result(format!(
                "Failed to get features for audio {}: {}",
                audio_id, e
            ))
        }
    }
}

fn error_result(message: String) -> StateResult {
    StateResult {
        state: StateResultType::Failure,
        message,
    }
}
