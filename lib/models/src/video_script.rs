use r2d2_mysql::mysql::{Params, prelude::Queryable};

use crate::DatabaseConfig;

pub struct VideoScript {
    script_id: i32,
    audio_id: i32,
    script_description: String,
    script_title: String,

    script_prompt: String,
    script_instructions: String,
}

impl VideoScript {
    pub fn get_id(&self) -> i32 {
        self.script_id
    }

    pub fn get_prompt(&self) -> String {
        self.script_prompt.clone()
    }

    pub fn get_instructions(&self) -> String {
        self.script_instructions.clone()
    }

    pub fn get_title(&self) -> String {
        self.script_title.clone()
    }

    pub fn get_description(&self) -> String {
        self.script_description.clone()
    }

    pub fn new(
        script_id: i32,
        audio_id: i32,
        script_prompt: String,
        script_instructions: String,
        database_config: &DatabaseConfig,
    ) -> std::result::Result<VideoScript, Box<dyn std::error::Error>> {
        let script = VideoScript {
            script_id,
            audio_id,
            script_prompt,
            script_instructions,
            script_description: "".to_string(),
            script_title: "".to_string(),
        };

        script.insert(database_config)?;

        Ok(script)
    }

    pub fn by_audio_id(audio_id: i32, database_config: &DatabaseConfig) -> std::result::Result<Option<VideoScript>, Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection()?;
        let query = format!("SELECT * FROM SCRIPT_LIST WHERE audio_id = {}", audio_id);
        println!("{}", query);
        let script = conn.query::<r2d2_mysql::mysql::Row, &str>(&query)?;

        let mut script_id = -1;
        let mut audio_id = audio_id;
        let mut script_prompt = "".to_string();
        let mut script_instructions = "".to_string();
        let mut script_title = "".to_string();
        let mut script_description = "".to_string();

        for row in script {

            log::info!("Found script for audio {}", audio_id);

            script_id = row.get("script_id").unwrap_or(-1);
            audio_id = row.get("audio_id").unwrap_or(-1);
            script_prompt = row.get("script_prompt").unwrap_or("".to_string());
            script_instructions = row.get("script_instructions").unwrap_or("".to_string());
            script_title = row.get("script_title").unwrap_or("".to_string());
            script_description = row.get("script_description").unwrap_or("".to_string());
        }

        if script_id == -1 {
            return Ok(None);
        }

        if audio_id == -1 {
            return Ok(None);
        }

        Ok(Some(VideoScript {
            script_id,
            audio_id,
            script_prompt,
            script_instructions,
            script_title,
            script_description,
        }))
    }

    fn insert(&self, database_config: &DatabaseConfig) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection()?;
        let query = "INSERT INTO SCRIPT_LIST (audio_id, script_prompt) VALUES (?, ?)";
        let params: Params = Params::from((self.audio_id, self.script_prompt.as_str()));
        let _ = conn.exec_first::<r2d2_mysql::mysql::Row, &str, Params>(query, params);
        Ok(())
    }

    pub fn update(
        &mut self,
        prompt: &str,
        inst: &str,

        title: &str,
        description: &str,
        database_config: &DatabaseConfig,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut conn = database_config
            .get_connection()?;
        let query = "UPDATE SCRIPT_LIST SET script_prompt = ?, script_instructions = ?, script_title = ?, script_description = ? WHERE audio_id = ?";
        let params: Params = Params::from((prompt, inst, title, description, self.audio_id));
        let _ = conn.exec_first::<r2d2_mysql::mysql::Row, &str, Params>(query, params);
        Ok(())
    }

    pub fn clear(audio_id: i32, database_config: &DatabaseConfig) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection()?;
        let query = format!("DELETE FROM SCRIPT_LIST WHERE audio_id = {}", audio_id);
        conn.query::<r2d2_mysql::mysql::Row, &str>(&query)?;
        Ok(())
    }
}
