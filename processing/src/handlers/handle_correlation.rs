use models::{AudioChunks, AudioCorrelation, AudioFeatures, DatabaseConfig, AudioFeaturesPython};

use crate::{
    state_manager::{StateResult, StateResultType},
    utils::compute_correlation,
};

pub async fn handle_correlation_state(
    audio_id: i32,
    database_config: &DatabaseConfig,
) -> StateResult {
    let features = match AudioFeatures::by_audio_id(audio_id, database_config) {
        Ok(features) => features,
        Err(e) => {
            log::info!("Failed to get features for audio {}: {}", audio_id, e);
            return StateResult {
                state: StateResultType::Failure,
                message: format!("Failed to get features for audio {}: {}", audio_id, e),
            };
        }
    }.to_vec();

    if features.is_empty() {
        return StateResult {
            state: StateResultType::Failure,
            message: format!("No features found for audio {}", audio_id),
        }
    }

    let chunks = match AudioChunks::by_audio_id(audio_id, database_config) {
        Ok(chunks) => chunks,
        Err(e) => {
            return StateResult {
                state: StateResultType::Failure,
                message: format!("Failed to get chunks for audio {}: {}", audio_id, e),
            }
        }
    };

    if chunks.is_empty() {
        return StateResult {
            state: StateResultType::Failure,
            message: format!("No chunks found for audio {}", audio_id),
        }
    }
    // map all features to AudioFeaturesPython::from_features(feature)
    let features = features.iter().map(|feature| AudioFeaturesPython::from_features(feature)).collect::<Vec<AudioFeaturesPython>>();
    let (_chunk_ids, correlation_values) = compute_correlation(&features, &chunks);

    log::info!("Calculated correlation state for audio_id: {}", audio_id);
    log::info!("Similar audio snippets: {:?}", correlation_values);

    let mut correlation = AudioCorrelation::new(-1, audio_id, correlation_values);

    match AudioCorrelation::clear(audio_id, database_config){
        Ok(_) => {}
        Err(e) => {
            return StateResult {
                state: StateResultType::Failure,
                message: format!("Failed to clear correlation for audio {}: {}", audio_id, e),
            }
        }
    };
    correlation.map();
    match correlation.insert(database_config) {
        Ok(_) => {}
        Err(e) => {
            return StateResult {
                state: StateResultType::Failure,
                message: format!("Failed to insert correlation for audio {}: {}", audio_id, e),
            }
        }
    }
    log::info!("Saved correlation state for audio_id: {}", audio_id);
    return StateResult {
        state: StateResultType::Success,
        message: format!("Saved correlation state for audio_id: {}", audio_id),
    }
}
