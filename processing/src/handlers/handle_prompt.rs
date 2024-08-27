use models::{DatabaseConfig, VideoScenes, VideoScript, MAX_SCENES_PER_REQUEST};
use crate::{state_manager::{StateResult, StateResultType}, utils::get_response};

pub async fn handle_prompt_state(audio_id: i32, database_config: &DatabaseConfig) -> StateResult {
    // Use the runtime to run the asynchronous code
    let mut video_script = match VideoScript::by_audio_id(audio_id, database_config) {
        Ok(video_script) => match video_script {
            Some(video_script) => video_script,
            None => {
                return StateResult {
                    state: StateResultType::Failure,
                    message: format!("No video script found for audio {}", audio_id),
                };
            }
        },
        Err(e) => {
            return StateResult {
                state: StateResultType::Failure,
                message: format!("Failed to get video script for audio {}: {}", audio_id, e),
            }
        }
    };

    let instructions = video_script.get_instructions();
    if instructions.len() == 0 {
        return StateResult {
            state: StateResultType::Failure,
            message: format!("No instructions found for audio {}", audio_id),
        };
    }

    let mut prompt = video_script.get_prompt();
    if prompt.len() == 0 {
        return StateResult {
            state: StateResultType::Failure,
            message: format!("No prompt found for audio {}", audio_id),
        };
    }

    // check for already generated scenes
    let db_scenes = match VideoScenes::by_audio_id(audio_id, database_config) {
        Ok(scenes) => scenes,
        Err(e) => {
            return StateResult {
                state: StateResultType::Failure,
                message: format!("Failed to get scenes for audio {}: {}", audio_id, e),
            };
        }
    };

    let mut generated_titles = Vec::new();
    let mut queued_max = MAX_SCENES_PER_REQUEST;
    let mut queued_scenes = Vec::new();
    for scene in &db_scenes {
        if !scene.is_empty() {
            generated_titles.push(scene.get_values().0);
        } else {
            if queued_max > 0 {
                queued_scenes.push(scene);
                queued_max -= 1;
            }
        }
    }

    if generated_titles.len() == db_scenes.len() {
        return StateResult {
            state: StateResultType::Success,
            message: format!("All scenes have already been generated for audio {}", audio_id),
        };
    }

    // append already generated scenes to prompt as list of titles for llm to see existing story and it will continue from there with new scenes
    // 1. explination of what is happening append to prompt
    prompt.push_str("The following scenes have already been generated:\n");
    // 2. list of already generated scenes
    for title in &generated_titles {
        prompt.push_str(&format!("{}\n", title));
    }
    // get response from llm

    let response = match get_response(&instructions, &prompt).await {
        Ok(response) => response,
        Err(e) => {
            return StateResult {
                state: StateResultType::Pending,
                message: format!("Error getting response: {}", e),
            };
        }
    };

    let scenes = response.get_scenes();

    if scenes.is_empty() {
        return StateResult {
            state: StateResultType::Failure,
            message: format!("No scenes generated for audio {}", audio_id),
        };
    }

    // Update the existing scene in the database

    for (i, scene) in scenes.iter().enumerate() {
        let (scene_title, scene_description, scene_tags, scene_color) = scene.get_values();
        if (i + generated_titles.len()) < db_scenes.len() {
            match db_scenes[i+ generated_titles.len()].update(
                scene_title,
                scene_description,
                scene_tags,
                scene_color,
                database_config,
            ) {
                Ok(_) => {}
                Err(e) => {
                    return StateResult {
                        state: StateResultType::Failure,
                        message: format!("Failed to update scene {}: {}", i, e),
                    };
                }
            }
        }
    }

    // Update the video script with the new prompt and instructions

    let (title, description) = response.get_script();
    match video_script.update(
        &prompt,
        &instructions,
        &title,
        &description,
        database_config,
    ) {
        Ok(_) => {}
        Err(e) => {
            return StateResult {
                state: StateResultType::Failure,
                message: format!("Failed to update video script: {}", e),
            };
        }
    };

    return StateResult {
        state: StateResultType::Success,
        message: format!("Done handling prompt state for audio {}", audio_id),
    };
}
