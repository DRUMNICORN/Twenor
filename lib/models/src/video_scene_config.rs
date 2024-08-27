use serde::{Deserialize, Serialize};

use crate::{VideoScene, AudioMetadata};
use std::fs;

#[derive(Serialize, Deserialize, Debug)] 
pub struct SceneConfig {
    prompt: String,
    n_prompt: String,
    steps: i32,
    frames: i32,
    seed: i32,
    cfg_scale: i32,
    width: i32,
    height: i32,
    do_vid2vid: bool,
    fps: i32,
}

impl SceneConfig {
    pub fn new(prompt: String, n_prompt: String, steps: i32, frames: i32, seed: i32, cfg_scale: i32, width: i32, height: i32, do_vid2vid: bool, fps: i32) -> Self {
        SceneConfig {
            prompt,
            n_prompt,
            steps,
            frames,
            seed,
            cfg_scale,
            width,
            height,
            do_vid2vid,
            fps,
        }
    }

    pub fn default() -> Self {
        SceneConfig {
            prompt: String::from("A video of a cat"),
            n_prompt: String::from("A video of a cat"),
            steps: 35,
            frames: 16,
            seed: -1,
            cfg_scale: 17,
            width: 256,
            height: 192,
            do_vid2vid: false,
            fps: 16,
        }
    }

    pub fn compute(scenes: &[VideoScene], metadata: &AudioMetadata) -> Vec<SceneConfig> {
        scenes
            .iter()
            .map(|scene| SceneConfig::generate_scene_config(scene, metadata))
            .collect()
    }

    pub fn insert(
        configs: &[SceneConfig],
        user_id: i32,
        audio_id: i32,
        filename: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let folder_path = format!("{}/{}/{}", ".db", user_id, audio_id);
        fs::create_dir_all(&folder_path)?;
    
        let file_path = format!("{}/{}", folder_path, filename);
        let json_data = serde_json::to_string_pretty(configs)?;
        fs::write(&file_path, json_data)?;
    
        Ok(())
    }
    

    pub fn with_prompt(mut self, prompt: String) -> Self {
        self.prompt = prompt;
        self
    }

    pub fn with_fps(mut self, fps: i32) -> Self {
        self.fps = fps;
        self
    }

    pub fn generate_scene_config(scene: &VideoScene, metadata: &AudioMetadata) -> SceneConfig {
        let (title, description, tags, color) = scene.get_values();
    
        let prompt = format!(
            "{} {} {} {} {}",
            title,
            description,
            tags.join(" "),
            metadata.get_artstyle(),
            color
        );
    
        SceneConfig::default()
            .with_prompt(prompt)
            .with_fps(metadata.clone().get_fps() as i32)
    }
    
    pub fn get_prompt(&self) -> String {
        self.prompt.clone()
    }

    pub fn get_n_prompt(&self) -> String {
        self.n_prompt.clone()
    }

    

}
