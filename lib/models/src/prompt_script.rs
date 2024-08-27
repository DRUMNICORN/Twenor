#[derive(serde::Deserialize, Debug, Clone)]
pub struct PromptScript {
    title: String,
    description: String,
}

impl PromptScript {
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn default() -> PromptScript {
        PromptScript {
            title: "".to_string(),
            description: "".to_string(),
        }
    }
}
