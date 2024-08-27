// src/state.rs

use r2d2_mysql::mysql::{prelude::Queryable, Params};
use serde::{Deserialize, Serialize};

use crate::Audio;

use super::DatabaseConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioState {

    Uploading,

    Describing,
    Converting,

    Chunking,
    Featuring,
    Correlating,
    Splitting, 
    
    Writing,
    Prompting,
    Rendering,

    Done,
    Error,
}

impl AudioState {
    pub fn next_state(&self) -> AudioState {
        match *self {
            AudioState::Uploading => AudioState::Describing,

            // AudioState::Describing => AudioState::Converting,
            AudioState::Describing => AudioState::Chunking,
            AudioState::Converting => AudioState::Chunking,

            AudioState::Chunking => AudioState::Featuring,
            AudioState::Featuring => AudioState::Correlating,
            AudioState::Correlating => AudioState::Splitting,
            AudioState::Splitting => AudioState::Writing,
            AudioState::Writing => AudioState::Prompting,
            AudioState::Prompting => AudioState::Rendering,
            AudioState::Rendering => AudioState::Done,
            AudioState::Done => AudioState::Done,
            AudioState::Error => AudioState::Error,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            AudioState::Uploading => "uploading",

            AudioState::Converting => "converting",
            AudioState::Describing => "describing",

            AudioState::Chunking => "chunking",
            AudioState::Featuring => "featuring",
            AudioState::Correlating => "correlating",
            AudioState::Splitting => "splitting",

            AudioState::Writing => "writing",
            AudioState::Prompting => "prompting",
            AudioState::Rendering => "rendering",

            AudioState::Done => "done",
            AudioState::Error => "error",
        }
    }

    pub fn from_str(audio_state: String) -> Self {
        match audio_state.as_str() {
            "uploading" => AudioState::Uploading,
        
            "converting" => AudioState::Converting,
            "describing" => AudioState::Describing,

            "chunking" => AudioState::Chunking,
            "featuring" => AudioState::Featuring,
            "correlating" => AudioState::Correlating,
            "splitting" => AudioState::Splitting,

            "writing" => AudioState::Writing,
            "prompting" => AudioState::Prompting,
            "rendering" => AudioState::Rendering,

            "done" => AudioState::Done,
            "error" => AudioState::Error,
            _ => AudioState::Error,
        }
    }

    pub fn reset_state(
        audio_id: i32,
        database_config: &DatabaseConfig,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        // set uploading to false
        Audio::by_id(audio_id, database_config)?.update_audio_loaded(false, database_config)?;
        AudioState::update(audio_id, AudioState::Uploading, "Reseted Audio State", 0.0, database_config)?;

        Ok(())
    }

    pub fn from_row(row: r2d2_mysql::mysql::Row) -> Self {
        let audio_state: String = row.get("state").unwrap();
        AudioState::from_str(audio_state)
    }

    pub fn is_equal(&self, other: &AudioState) -> bool {
        self.as_str() == other.as_str()
    }

    pub fn by_id(
        audio_id: i32,
        database_config: &DatabaseConfig,
    ) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        // Connect to the database using the connection pool
        let mut conn = database_config.get_connection()?;

        // Execute the SELECT query to get the audio state
        let query = "SELECT state FROM AUDIO_STATE WHERE audio_id = ?";
        let params: Params = Params::from((audio_id,));
        let audio_state = conn.exec_first::<r2d2_mysql::mysql::Row, &str, Params>(query, params)?;

        let audio_state: AudioState = match audio_state {
            Some(row) => AudioState::from_row(row),
            None => {
                AudioState::insert(audio_id, database_config, AudioState::Uploading)?;
                AudioState::Uploading
            },
        };

        // If the audio state is found, return the audio state
        Ok(audio_state)
    }

    pub fn by_state(
        state: AudioState,
        database_config: &DatabaseConfig,
    ) -> std::result::Result<Option<i32>, Box<dyn std::error::Error>> {
        // Connect to the database using the connection pool
        let mut conn = database_config.get_connection()?;

        // Execute the SELECT query to search for a audio with the given state
        let query = "SELECT audio_id FROM AUDIO_STATE WHERE state = ?";
        let params: Params = Params::from((state.as_str(),));
        let result = conn.exec_first::<r2d2_mysql::mysql::Row, &str, Params>(query, params)?;
        // If the audio is found, return the audio id
        Ok(match result {
            Some(row) => {
                let audio_id = row
                    .get("audio_id")
                    .ok_or_else(|| "Failed to get audio id")?;
                Some(audio_id)
            }
            None => None,
        })
    }

    pub fn all_by_state(
        state: AudioState,
        database_config: &DatabaseConfig,
    ) -> std::result::Result<Vec<i32>, Box<dyn std::error::Error>> {
        // Connect to the database using the connection pool
        let mut conn = database_config.get_connection()?;

        // Execute the SELECT query to search for a audio with the given state
        let query = "SELECT audio_id FROM AUDIO_STATE WHERE state = ?";
        let params: Params = Params::from((state.as_str(),));
        let result = conn.exec::<r2d2_mysql::mysql::Row, &str, Params>(query, params)?;
        // If the audio is found, return the audio id
        Ok(result
            .into_iter()
            .map(|row| {
                let audio_id = row
                    .get("audio_id")
                    .ok_or_else(|| "Failed to get audio id")
                    .unwrap();
                audio_id
            })
            .collect())
    }


    pub fn insert(
        audio_id: i32,
        database_config: &DatabaseConfig,
        audio_state: AudioState,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        // Connect to the database using the connection pool

        let user_id = Audio::by_id(audio_id, database_config)?.get_user_id();

        let mut conn = database_config.get_connection()?;

        // Execute the INSERT query to insert the audio state
        let query = "INSERT INTO AUDIO_STATE (audio_id, user_id, state) VALUES (?, ?, ?)";
        let params: Params = Params::from((audio_id, user_id, audio_state.as_str()));
        match conn.exec_first::<r2d2_mysql::mysql::Row, &str, Params>(query, params) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }

    fn update_progress(
        audio_id: i32,
        progress: f32,
        database_config: &DatabaseConfig,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        // Connect to the database using the connection pool
        let mut conn = database_config.get_connection()?;

        // Execute the UPDATE query to update the audio state
        let query = "UPDATE AUDIO_STATE SET progress = ? WHERE audio_id = ?";
        let params: Params = Params::from((progress, audio_id));
        conn.exec_first::<usize, &str, Params>(query, params)?;
        Ok(())
    }

    fn progress(
        audio_id: i32,
        database_config: &DatabaseConfig,
    ) -> std::result::Result<f32, Box<dyn std::error::Error>> {
        // Connect to the database using the connection pool
        let mut conn = database_config.get_connection()?;

        // Execute the SELECT query to get the audio state
        let query = "SELECT progress FROM AUDIO_STATE WHERE audio_id = ?";
        let params: Params = Params::from((audio_id,));
        let progress = conn.exec_first::<r2d2_mysql::mysql::Row, &str, Params>(query, params)?;

        let progress: f32 = match progress {
            Some(row) => row
                .get("progress")
                .ok_or_else(|| "Failed to get progress")?,
            None => 0.0,
        };

        Ok(progress)
    }

    pub fn update(
        audio_id: i32,
        audio_state: AudioState,
        message: &str,
        progress: f32,
        database_config: &DatabaseConfig,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        // check if state exists else create it
        let existing_audio_state = AudioState::by_id(audio_id, database_config)?;
        if existing_audio_state.is_equal(&audio_state) {
            return Ok(());
        }

        // Connect to the database using the connection pool
        let mut conn = database_config.get_connection()?;

        // Execute the UPDATE query to update the audio state
        let query = "UPDATE AUDIO_STATE SET state = ?, message = ?, progress = ? WHERE audio_id = ?";
        let params: Params = Params::from((audio_state.as_str(), message, progress, audio_id));
        log::info!("Updating audio state to {}", audio_state.as_str());
        conn.exec_first::<usize, &str, Params>(query, params)?;
        Ok(())
    }


}
