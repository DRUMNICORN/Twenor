// models.rs

use r2d2_mysql::mysql::Params;
use serde::{Deserialize, Serialize};

use super::AppState;

// Define the SceneRequest type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneRequest {
    // Define the fields of the SceneRequest type here
    // For example:
    pub audio_file: String,
    // string list for each key
    // pub artwork: HashMap<String, Vec<String>>, 
}

// Define the Scenes type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenes {
    // Define the fields of the Scenes type here
    // For example:
    pub scenes: Vec<Scene>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    scene_id: i32,
    track_id: i32,
    scene_title: String,
    scene_description: String,
    scene_tags: String,
    scene_color: String,
    scene_chunks: Vec<i32>,


    scene_start: f32,
    scene_end: f32,
}

impl Scene {
    fn new(
        scene_id: i32,
        track_id: i32,
        scene_title: String,
        scene_description: String,
        scene_tags: String,
        scene_color: String,
        scene_chunks: Vec<i32>,
        scene_start: f32,
        scene_end: f32,
    ) -> Scene {
        let scene = Scene {
            scene_id,
            track_id,
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

    pub fn save(&self, state: rocket::State<'_, AppState>) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = state.db_pool().get()?;        
      
        let query = "INSERT INTO SCENE_LIST (track_id, scene_title, scene_description, scene_tags, scene_color, scene_chunks, scene_start, scene_end) VALUES (?, ?, ?, ?, ?, ?, ?, ?)";
        
        let chunk_ids = self.scene_chunks.clone();
        // serde to string
        let chunk_ids_string = serde_json::to_string(&chunk_ids).unwrap();

        let params: Params = Params::from((
            self.track_id,
            self.scene_title.as_str(),
            self.scene_description.as_str(),
            self.scene_tags.as_str(),
            self.scene_color.as_str(),
            chunk_ids_string.as_str(),
            self.scene_start,
            self.scene_end,
        ));
        conn.prep_exec(query, params)
            .expect("Failed to execute query");
        Ok(())
    }

    pub fn load_by_track_id(track_id: i32, state: rocket::State<'_, AppState>) -> Result<Vec<Scene>, Box<dyn std::error::Error>> {
        let mut conn = state.db_pool().get()?; 

        let query = format!("SELECT * FROM SCENE_LIST WHERE track_id = {}", track_id);
        let scene = conn.query(query).unwrap();

        let mut scenes: Vec<Scene> = Vec::new();
        
        for row in scene {
            let row = row.unwrap();
            let scene = Scene::from_row(row);
            scenes.push(scene);
        }

        Ok(scenes)
    }

    fn from_row(row: r2d2_mysql::mysql::Row) -> Scene {
        let scene_id = row.get("scene_id").unwrap();
        let track_id = row.get("track_id").unwrap();
        let scene_title = row.get("scene_title").unwrap();
        let scene_description = row.get("scene_description").unwrap();
        let scene_tags = row.get("scene_tags").unwrap();
        let scene_color = row.get("scene_color").unwrap();
        let scene_chunks_str: String = row.get("scene_chunks").unwrap();
        let scene_start = row.get("scene_start").unwrap();
        let scene_end = row.get("scene_end").unwrap();
        
        let scene_chunks: Vec<i32> = serde_json::from_str(&scene_chunks_str).unwrap();

        Scene::new(
            scene_id,
            track_id,
            scene_title,
            scene_description,
            scene_tags,
            scene_color,
            scene_chunks,
            scene_start,
            scene_end,
        )
    }
}
// Add any other necessary definitions to the models module
