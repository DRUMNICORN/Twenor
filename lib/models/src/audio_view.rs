use r2d2_mysql::mysql::{Params, prelude::Queryable};
use serde::{Serialize, Deserialize};

use crate::{Audio, DatabaseConfig, AudioMetadata, AudioState};



#[derive(Debug, Serialize, Deserialize)]
pub struct AudioView {
    id: String, // load from audio_id
    title: String, // load from metadata
    state: String // load from state
}

impl AudioView {
    pub fn list_by_token(token: &String, database_config: &DatabaseConfig) -> std::result::Result<Vec<AudioView>, Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection()?;

        let query = "SELECT user_id FROM USER_LIST WHERE user_token = ?";
        let params: Params = Params::from((token,));

        let result = conn.exec_first::<r2d2_mysql::mysql::Row, &str, Params>(query, params)?;

        let row = match result {
            Some(row) => row,
            None => {
                return Err("Failed to get user".to_string().into());
            }
        };

        let user_id = row.get("user_id").unwrap_or(-1);

        let audio_views = AudioView::by_user_id(user_id, database_config)?;
        Ok(audio_views)

    }

    pub fn by_user_id(user_id: i32, database_config: &DatabaseConfig) -> std::result::Result<Vec<AudioView>, Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection()?;

        let query = "SELECT audio_id FROM AUDIO_LIST WHERE user_id = ?";
        let params: Params = Params::from((user_id,));
        let result = conn.exec::<r2d2_mysql::mysql::Row, &str, Params>(query, params)?;

        let mut audio_ids = Vec::new();

        for row in result {
            let audio_id = row.get("audio_id").unwrap_or(-1);
            audio_ids.push(audio_id);
        }

        let mut audio_views = Vec::new();
        
        for audio_id in audio_ids {
            let audio_view = AudioView::by_id(audio_id, database_config)?;
            audio_views.push(audio_view);
        }

        Ok(audio_views)
    }

    pub fn by_id(audio_id: i32, database_config: &DatabaseConfig) -> std::result::Result<AudioView, Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection()?;

        let audio = Audio::by_id(audio_id, database_config)?;

        let metadata = AudioMetadata::by_audio_id(audio_id, database_config)?;

        let state = AudioState::by_id(audio_id, database_config)?;

        let audio_view = AudioView {
            id: audio.get_audio_id().to_string(),
            title: metadata.get_title(),
            state: state.as_str().to_string()
        };

        Ok(audio_view)
    }
}