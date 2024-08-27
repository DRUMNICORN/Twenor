use r2d2_mysql::mysql::{Row, prelude::Queryable};
use serde::{Serialize, Deserialize};

use crate::{VideoScene, DatabaseConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoScenes {
    audio_path: String,
    audio_id: i32,
    user_id: i32,
    scenes: Vec<VideoScene>,
}

impl  VideoScenes {
    pub fn new(scenes: Vec<VideoScene>) -> VideoScenes {
        VideoScenes {
            audio_path: String::new(),
            audio_id: 0,
            user_id: 0,
            scenes,
        }
    }
    
    pub fn by_audio_id(audio_id: i32, database_config: &DatabaseConfig) -> std::result::Result<Vec<VideoScene>, Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection()?;
        let query = format!("SELECT * FROM SCENE_LIST WHERE audio_id = {}", audio_id);
        let scene = conn.query::<Row, &str>(&query)?;
        let mut scenes: Vec<VideoScene> = Vec::new();

        for row in scene {
            let scene = VideoScene::from_row(row)?;
            scenes.push(scene);
        }

        Ok(scenes)
    }

    pub fn from_row(row: &Row) -> std::result::Result<VideoScene, Box<dyn std::error::Error>> {
        let scene_id = row.get("scene_id").ok_or_else(|| "Failed to get scene_id")?;
        let audio_id = row.get("audio_id").ok_or_else(|| "Failed to get audio_id")?;
        let scene_title = row.get("scene_title").ok_or_else(|| "Failed to get scene_title")?;
        let scene_description = row.get("scene_description").ok_or_else(|| "Failed to get scene_description")?;
        let tags_str: String = row.get("scene_tags").ok_or_else(|| "Failed to get scene_tags")?;
        let scene_tags: Vec<String> = serde_json::from_str(&tags_str)?;
        let scene_color = row.get("scene_color").ok_or_else(|| "Failed to get scene_color")?;
        let scene_chunks_str: String = row.get("scene_chunks").ok_or_else(|| "Failed to get scene_chunks")?;
        let scene_start = row.get("scene_start").ok_or_else(|| "Failed to get scene_start")?;
        let scene_end = row.get("scene_end").ok_or_else(|| "Failed to get scene_end")?;
        
        let scene_chunks: Vec<i32> = serde_json::from_str(&scene_chunks_str)?;

        Ok(VideoScene::new(
            scene_id,
            audio_id,
            scene_title,
            scene_description,
            scene_tags,
            scene_color,
            scene_chunks,
            scene_start,
            scene_end,
        ))
    }

    pub fn clear(audio_id: i32, database_config: &DatabaseConfig) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection()?;

        let query = format!("DELETE FROM SCENE_LIST WHERE audio_id = {}", audio_id);
        match conn.query::<Row, &str>(&query) {
            Ok(_) => {
                log::info!("Successfully cleared scene list");
            },
            Err(e) => {
                log::error!("Failed to clear scene list: {}", e);
            }
        };
        Ok(())
    }
}