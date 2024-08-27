use models::{
    AudioFeatures, AudioFeaturesPython, AudioMetadata, DatabaseConfig, PromptInstructions,
    VideoScenes, VideoScript,
};

use crate::{
    state_manager::{StateResult, StateResultType},
    utils::calculate_scene_string,
};

pub async fn handle_writing_state(audio_id: i32, database_config: &DatabaseConfig) -> StateResult {
    log::info!("Handling writing state");
    let t_scenes = match VideoScenes::by_audio_id(audio_id, database_config) {
        Ok(scenes) => scenes,
        Err(e) => {
            return StateResult {
                state: StateResultType::Failure,
                message: format!("Failed to get scenes for audio {}: {}", audio_id, e),
            };
        }
    };
    log::info!("Scenes found: {}", t_scenes.len());
    let t_scenes_len = t_scenes.len();
    if t_scenes_len == 0 {
        return StateResult {
            state: StateResultType::Failure,
            message: format!("No scenes found for audio {}", audio_id),
        };
    }

    let t_metadata = match AudioMetadata::by_audio_id(audio_id, database_config) {
        Ok(metadata) => metadata,
        Err(e) => {
            return StateResult {
                state: StateResultType::Failure,
                message: format!("Failed to get metadata for audio {}: {}", audio_id, e),
            };
        }
    };
    let t_scene_features = match AudioFeatures::by_audio_id(audio_id, database_config) {
        Ok(features) => features,
        Err(e) => {
            return StateResult {
                state: StateResultType::Failure,
                message: format!("Failed to get features for audio {}: {}", audio_id, e),
            };
        }
    };

    let config: PromptInstructions = PromptInstructions::default();

    // Generate the story and plot the scene features

    let t_scene_features = t_scene_features
        .to_vec()
        .iter()
        .map(|feature| AudioFeaturesPython::from_features(feature))
        .collect::<Vec<AudioFeaturesPython>>();
    let t_story = calculate_scene_string(audio_id, &t_scene_features, &t_scenes, database_config);
    // Generate the prompt

    let t_instructions = config.get();
    let t_prompt = get_prompt(&t_metadata, &t_story);

    let _script = VideoScript::new(-1, audio_id, t_prompt, t_instructions, database_config);

    return StateResult {
        state: StateResultType::Success,
        message: format!("Done handling writing state for audio {}", audio_id),
    };
}

fn get_prompt(metadata: &AudioMetadata, story: &str) -> String {
    let mut prompt: String = "".to_string();

    for key in metadata.names() {
        let value = metadata.get(&key);
        prompt.push_str(&format!("{}: {}\n", key, value));
    }

    prompt.push_str(&format!("\n{}", story));

    prompt
}
