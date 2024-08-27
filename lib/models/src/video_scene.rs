// models.rs
use r2d2_mysql::mysql::prelude::Queryable;
use r2d2_mysql::mysql::Params;
use serde::{Deserialize, Serialize};

use crate::DatabaseConfig;

// Define the SceneRequest type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneRequest {
    // Define the fields of the SceneRequest type here
    // For example:
    audio_file: String,
    // string list for each key
    // pub artwork: HashMap<String, Vec<String>>,
}

// Define the Scenes type

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoScene {
    scene_id: i32,
    audio_id: i32,
    scene_title: String,
    scene_description: String,
    scene_tags: Vec<String>,
    scene_color: String,
    scene_chunks: Vec<i32>,
    scene_start: f32,
    scene_end: f32,
}

impl VideoScene {
    pub fn new(
        scene_id: i32,
        audio_id: i32,
        scene_title: String,
        scene_description: String,
        scene_tags: Vec<String>,
        scene_color: String,
        scene_chunks: Vec<i32>,
        scene_start: f32,
        scene_end: f32,
    ) -> VideoScene {
        let scene = VideoScene {
            scene_id,
            audio_id,
            scene_title,
            scene_description,
            scene_tags,
            scene_color,
            scene_chunks,
            scene_start,
            scene_end,
        };
        scene
    }

    pub fn default() -> VideoScene {
        let scene = VideoScene {
            scene_id: 0,
            audio_id: 0,
            scene_title: String::new(),
            scene_description: String::new(),
            scene_tags: Vec::new(),
            scene_color: String::new(),
            scene_chunks: Vec::new(),
            scene_start: 0.0,
            scene_end: 0.0,
        };
        scene
    }

    pub fn is_empty(&self) -> bool {
        let has_title = self.scene_title != "";
        let has_description = self.scene_description != "";

        if has_title && has_description {
            return false;
        }
        true
    }

    pub fn insert(&self, database_config: &DatabaseConfig) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection()?;

        let query = "INSERT INTO SCENE_LIST (audio_id, scene_title, scene_description, scene_tags, scene_color, scene_chunks, scene_start, scene_end) VALUES (?, ?, ?, ?, ?, ?, ?, ?)";

        let chunk_ids = self.scene_chunks.clone();
        // serde to string
        let chunk_ids_string = serde_json::to_string(&chunk_ids)?;

        let scene_tags = serde_json::to_string(&self.scene_tags)?;
        let params: Params = Params::from((
            self.audio_id,
            self.scene_title.as_str(),
            self.scene_description.as_str(),
            scene_tags,
            self.scene_color.as_str(),
            chunk_ids_string.as_str(),
            self.scene_start,
            self.scene_end,
        ));
        let _ = conn.exec_first::<r2d2_mysql::mysql::Row, &str, Params>(query, params);

        Ok(())
    }

    pub fn id(&self) -> i32 {
        self.scene_id
    }

    pub fn by_audio_id(
        audio_id: i32,
        database_config: &DatabaseConfig,
    ) -> Result<Vec<VideoScene>, Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection()?;

        let query = format!("SELECT * FROM SCENE_LIST WHERE audio_id = {}", audio_id);
        let scene = conn.query::<r2d2_mysql::mysql::Row, &str>(&query)?;

        let mut scenes: Vec<VideoScene> = Vec::new();

        for row in scene {
            let scene = VideoScene::from_row(row)?;
            scenes.push(scene);
        }

        Ok(scenes)
    }

    pub fn from_row(
        row: r2d2_mysql::mysql::Row,
    ) -> std::result::Result<VideoScene, Box<dyn std::error::Error>> {
        let scene_id = row
            .get("scene_id")
            .ok_or_else(|| "Failed to get scene_id")?;
        let audio_id = row
            .get("audio_id")
            .ok_or_else(|| "Failed to get audio_id")?;
        let scene_title = row
            .get("scene_title")
            .ok_or_else(|| "Failed to get scene_title")?;
        let scene_description = row
            .get("scene_description")
            .ok_or_else(|| "Failed to get scene_description")?;

        let tags_str: String = row
            .get("scene_tags")
            .ok_or_else(|| "Failed to get scene_tags")?;
        let scene_tags: Vec<String> = serde_json::from_str(&tags_str)?;

        let scene_color = row
            .get("scene_color")
            .ok_or_else(|| "Failed to get scene_color")?;
        let scene_chunks_str: String = row
            .get("scene_chunks")
            .ok_or_else(|| "Failed to get scene_chunks")?;
        let scene_start = row
            .get("scene_start")
            .ok_or_else(|| "Failed to get scene_start")?;
        let scene_end = row
            .get("scene_end")
            .ok_or_else(|| "Failed to get scene_end")?;

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

    pub fn clear(
        scene_id: i32,
        database_config: &DatabaseConfig,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection()?;

        let query = format!("DELETE FROM SCENE_LIST WHERE scene_id = {}", scene_id);
        match conn.query::<r2d2_mysql::mysql::Row, &str>(&query) {
            Ok(_) => {
                log::info!("Successfully cleared scene list");
            }
            Err(e) => {
                log::error!("Failed to clear scene list: {}", e);
            }
        };
        Ok(())
    }

    pub fn add_chunk(&mut self, chunk_id: i32) {
        self.scene_chunks.push(chunk_id);
    }

    pub fn get_chunks(&self) -> Vec<i32> {
        self.scene_chunks.clone()
    }

    pub fn update(
        &self,
        scene_title: String,
        scene_description: String,
        scene_tags: Vec<String>,
        scene_color: String,
        database_config: &DatabaseConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection()?;

        let scene_tags = scene_tags
            .iter()
            .map(|x| x.replace(" ", "_"))
            .collect::<Vec<String>>();
        let scene_tags = serde_json::to_string(&scene_tags)?;

        let query = format!("UPDATE SCENE_LIST SET scene_title = '{}', scene_description = '{}', scene_tags = '{}', scene_color = '{}' WHERE scene_id = {}", scene_title, scene_description, scene_tags, scene_color, self.scene_id);
        log::info!("Query: {}", query);
        log::info!("scene_tags: {:?}", scene_tags);
        log::info!("scene_color: {:?}", scene_color);
        conn.query::<r2d2_mysql::mysql::Row, &str>(&query)?;
        Ok(())
    }

    pub fn get_values(&self) -> (String, String, Vec<String>, String) {
        let scene_tags: Vec<String> = self.scene_tags.clone();
        (
            self.scene_title.clone(),
            self.scene_description.clone(),
            scene_tags,
            self.scene_color.clone(),
        )
    }
}
// Add any other necessary definitions to the models module
