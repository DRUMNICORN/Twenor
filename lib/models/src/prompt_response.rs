use crate::{PromptScene, PromptScript};

#[derive(serde::Deserialize, Debug, Clone)]
pub struct PromptResponse {
    script: PromptScript,
    scenes: Vec<PromptScene>,
}

impl PromptResponse {
    pub fn from_response(response: String) -> PromptResponse {
        let response: PromptResponse = match serde_json::from_str(&response) {
            Ok(response) => response,
            Err(err) => {
                log::error!("Error parsing response: {}", err);
                PromptResponse::default()
            }
        };
        response
    }

    pub fn get_scenes(&self) -> Vec<PromptScene> {
        self.scenes.clone()
    }

    pub fn get_script(&self) -> (String, String) {
        (self.script.get_title(), self.script.get_description())
    }

    pub fn default() -> PromptResponse {
        PromptResponse {
            script: PromptScript::default(),
            scenes: vec![PromptScene::default()],
        }
    }
}
