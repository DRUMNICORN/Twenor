

#[derive(serde::Deserialize, Debug, Clone)]
pub struct PromptScene {
    scene_title: String,
    scene_description: String,
    scene_tags: Vec<String>,
    scene_color: String,
}


impl PromptScene{
    pub fn default() -> PromptScene {
        PromptScene {
            scene_title: "".to_string(),
            scene_description: "".to_string(),
            scene_tags: vec![
                "".to_string()
            ],
            scene_color: "#ffffff".to_string()
        }
    }
    pub fn get_values(&self) -> (String, String, Vec<String>, String) {
        (self.scene_title.clone(), self.scene_description.clone(), self.scene_tags.clone(), self.scene_color.clone())
    }

    pub fn from_object(scene: &serde_json::Map<String, serde_json::Value>) -> PromptScene {
        let scene_title = scene["scene_title"].as_str().unwrap_or("").to_string();
        let scene_description = scene["scene_description"].as_str().unwrap_or("").to_string();
        let scene_tags = scene["scene_tags"].as_array().unwrap_or(&vec![]).iter().map(|tag| tag.as_str().unwrap_or("").to_string()).collect();
        let scene_color = scene["scene_color"].as_str().unwrap_or("").to_string();

        PromptScene {
            scene_title,
            scene_description,
            scene_tags,
            scene_color,
        }
    }

    pub fn update(&self, scene_title: String, scene_description: String, scene_tags: Vec<String>, scene_color: String) {
        let mut scene = PromptScene::default();
        scene.scene_title = scene_title;
        scene.scene_description = scene_description;
        scene.scene_tags = scene_tags;
        scene.scene_color = scene_color;
    }
}
