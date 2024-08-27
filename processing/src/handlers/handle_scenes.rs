
use models::{VideoScene, AudioCorrelation, AudioChunks, DatabaseConfig, VideoScenes, DIFF_ABS_THRESHOLD, DIFF_CURR_THRESHOLD, DIFF_PEV_THRESHOLD};

use crate::state_manager::{StateResult, StateResultType};

pub async fn handle_scene_state(audio_id: i32, database_config: &DatabaseConfig) -> StateResult {

    let corr_list = match AudioCorrelation::get_correlation(audio_id, database_config) {
        Ok(corr) => match corr {
            Some(corr) => corr,
            None => {
                return StateResult {
                    state: StateResultType::Failure,
                    message: format!("No correlation found for audio {}", audio_id),
                };
            }
        },
        Err(e) => {
            return StateResult {
                state: StateResultType::Failure,
                message: format!("Failed to get correlation for audio {}: {}", audio_id, e),
            };            
        }
    }.into_vec();

    let chunk_list =  match AudioChunks::by_audio_id(audio_id, database_config) {
        Ok(chunks) => chunks,
        Err(e) => {
            return StateResult {
                state: StateResultType::Failure,
                message: format!("Failed to get chunks for audio {}: {}", audio_id, e),
            };
        }
    };

    if chunk_list.len() != corr_list.len() {
        return StateResult {
            state: StateResultType::Failure,
            message: format!("Chunk list and correlation list are not the same length for audio {}", audio_id),
        }
    }

    let mut scene_list: Vec<VideoScene> = Vec::new();
    let mut current_chunk_ids: Vec<i32> = Vec::new();
    let mut start_index_chunk = 0;
    
    for i in 0..chunk_list.len() {
        if current_chunk_ids.len() == 0 {
            start_index_chunk = i;
        }
        current_chunk_ids.push(chunk_list[i].get_chunk_id());
        let corr = corr_list[i];

        let next_corr = corr_list[(i + 1) % corr_list.len()];
        let pev_corr = corr_list[(i + corr_list.len() - 1) % corr_list.len()];

        let aplifitude_prev_to_curr = (corr - pev_corr).abs();
        let aplifitude_curr_to_next = (corr - next_corr).abs();

        let diff = aplifitude_prev_to_curr - aplifitude_curr_to_next;
        let diff_pass_zero = (diff > 0.0) && (aplifitude_prev_to_curr > 0.0) && (aplifitude_curr_to_next > 0.0);
        let diff_pass_threshold = (diff.abs() > DIFF_ABS_THRESHOLD.into()) && (aplifitude_prev_to_curr < DIFF_CURR_THRESHOLD.into()) && (aplifitude_curr_to_next < DIFF_PEV_THRESHOLD.into());

        if diff_pass_threshold || diff_pass_zero {
            let scene = VideoScene::new(
                -1,
                audio_id,
                String::from(format!("Scene {}", scene_list.len() + 1)),
                String::from("No description"),
                vec![],
                String::from("#F0F0F0"),
                current_chunk_ids.clone(),
                chunk_list[start_index_chunk].get_start_end_index().0,
                chunk_list[i].get_start_end_index().1,
            );

            scene_list.push(scene);
            current_chunk_ids.clear();

            continue;
        }
    }

    match VideoScenes::clear(audio_id, database_config) {
        Ok(_) => {}
        Err(e) => {
            return StateResult {
                state: StateResultType::Failure,
                message: format!("Failed to clear scenes for audio {}: {}", audio_id, e),
            }
        }
    };
    for scene in scene_list {
        match scene.insert(database_config) {
            Ok(_) => {}
            Err(e) => {
                return StateResult {
                    state: StateResultType::Failure,
                    message: format!("Failed to insert scene for audio {}: {}", audio_id, e),
                }
            }
        } 
    }

    return StateResult {
        state: StateResultType::Success,
        message: format!("Audio {} is chunked", audio_id),
    }
}
