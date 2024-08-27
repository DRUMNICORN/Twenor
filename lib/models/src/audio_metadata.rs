use r2d2_mysql::mysql::Params;
use serde::{Deserialize, Serialize};

use super::{Audio, DatabaseConfig};
use r2d2_mysql::mysql::prelude::Queryable;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioMetadata {
    // STATIC IDS
    metadata_id: i32,
    audio_id: i32,
    user_id: i32,

    // DYNAMIC IDS
    title: String,
    artist: String,
    scale: String,
    genre: String,
    artstyle: String,
    lyrics: String,

    // PREDEFINED IDS
    bpm: f32,
    offset: f32,
    
}

impl AudioMetadata {
    pub fn new(audio: &Audio, database_config: &DatabaseConfig) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        let mut meta_data = AudioMetadata {
                metadata_id: -1,
                audio_id: audio.get_audio_id(),
                user_id: audio.get_user_id(),
                title: "".to_string(),
                artist: "".to_string(),
                bpm: 0.0,
                offset: 0.0,
                scale: "".to_string(),
                genre: "".to_string(),
                artstyle: "".to_string(),
                lyrics: "".to_string(),

        };

        // search if exsist 
        meta_data.insert(database_config)?;        

        Ok(meta_data)
    }

    pub fn fill_in(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
        

        Ok(())
    }

    pub fn get_artstyle(&self) -> String {
        self.artstyle.clone()
    }

    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }

    pub fn get_audio_id(&self) -> i32 {
        self.audio_id
    }

    pub fn get_fps(self) -> f32 {
        let frames_per_beat = 16.0;
        frames_per_beat * self.bpm / 60.0
    }

    pub fn get_bpm(&self) -> f32 {
        self.bpm
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn is_filled(&self) -> bool {
        // self.title.len() > 0 &&
        // self.artist.len() > 0 &&
        log::info!("BPM: {}", self.bpm);
        log::info!("Offset: {}", self.offset);
        log::info!("is bpm > 0.0: {}", self.bpm > 0.0);
        self.bpm > 0.0 
        // self.offset > 0.0 &&
        // self.scale.len() > 0 &&
        // self.genre.len() > 0 &&
        // self.artstyle.len() > 0 &&
        // self.lyrics.len() > 0
    }

    pub fn contains_key(&self, key: &str) -> bool {
        match key {
            "title" => self.title.len() > 0,
            "artist" => self.artist.len() > 0,
            "bpm" => self.bpm > 0.0,
            "offset" => self.offset > 0.0,
            "scale" => self.scale.len() > 0,
            "genre" => self.genre.len() > 0,
            "artstyle" => self.artstyle.len() > 0,
            "lyrics" => self.lyrics.len() > 0,
            _ => false,
        }
    }

    pub fn insert(&mut self, database_config: &DatabaseConfig) -> std::result::Result<(), Box<dyn std::error::Error>> {
        log::info!("Inserting metadata for audio_id: {}", self.audio_id);
        let mut conn = database_config.get_connection().map_err(|err| format!("Failed to get database connection: {}", err))?;
        let query = "INSERT INTO AUDIO_METADATA (`audio_id`, `user_id`, `title`, `artist`, `bpm`, `offset`, `scale`, `genre`, `artstyle`, `lyrics`) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
        let params: Params = Params::from((
            self.audio_id,
            self.user_id,
            &self.title,
            &self.artist,
            self.bpm,
            self.offset,
            &self.scale,
            &self.genre,
            &self.artstyle,
            &self.lyrics,
        ));
        let result = match conn.exec_first::<usize, &str, Params>(query, params)? {
            Some(result) => result,
            None => {
                return Err("Failed to insert metadata".to_string().into());
            }
        };

        self.metadata_id = result as i32;
        let validation_metadata_id = AudioMetadata::by_audio_id(self.audio_id, database_config)?.metadata_id;


        let is_valid = validation_metadata_id == self.metadata_id;
        if is_valid {
            log::info!("Metadata inserted successfully");
            Ok(())
        } else {
            log::error!("Metadata inserted failed");
            Err("Metadata inserted failed".to_string().into())
        }
    }

    pub fn by_audio_id(audio_id: i32, database_config: &DatabaseConfig) -> Result<AudioMetadata, String> {
        let mut conn = database_config.get_connection().map_err(|err| format!("Failed to get database connection: {}", err))?;
        let query = "SELECT * FROM AUDIO_METADATA WHERE audio_id = ?";
        let params: Params = Params::from((audio_id,));
        let meta_data = conn.exec_first::<r2d2_mysql::mysql::Row, &str, Params>(query, params).map_err(|err| format!("Failed to execute query: {}", err))?;
        let row = match meta_data {
            Some(meta_data) => {
                meta_data
            },
            None => {
                return Err("Failed to get metadata".to_string());
            }
        };

        let meta_data = AudioMetadata {
            metadata_id: row.get("metadata_id").unwrap_or(-1),
            audio_id: row.get("audio_id").unwrap_or(-1),
            user_id: row.get("user_id").unwrap_or(-1),
            title: row.get("title").unwrap_or("".to_string()),
            artist: row.get("artist").unwrap_or("".to_string()),
            bpm: row.get("bpm").unwrap_or(0.0),
            offset: row.get("offset").unwrap_or(0.0),
            scale: row.get("scale").unwrap_or("".to_string()),
            genre: row.get("genre").unwrap_or("".to_string()),
            artstyle: row.get("artstyle").unwrap_or("".to_string()),
            lyrics: row.get("lyrics").unwrap_or("".to_string()),
        };
        Ok(meta_data)

    }

    pub fn update_metadata(&mut self, other_metadata: AudioMetadata, database_config: &DatabaseConfig) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection().map_err(|err| format!("Failed to get database connection: {}", err))?;

        self.title = other_metadata.title;
        self.artist = other_metadata.artist;
        self.bpm = other_metadata.bpm;
        self.offset = other_metadata.offset;
        self.scale = other_metadata.scale;
        self.genre = other_metadata.genre;
        self.artstyle = other_metadata.artstyle;
        self.lyrics = other_metadata.lyrics;

        let query = "UPDATE AUDIO_METADATA SET `title` = ?, `artist` = ?, `bpm` = ?, `offset` = ?, `scale` = ?, `genre` = ?, `artstyle` = ?, `lyrics` = ? WHERE `metadata_id` = ?";
        let params: Params = Params::from((
            &self.title,
            &self.artist,
            self.bpm,
            self.offset,
            &self.scale,
            &self.genre,
            &self.artstyle,
            &self.lyrics,

            self.metadata_id,
        ));
        conn.exec_first::<usize, &str, Params>(query, params)?;

        Ok(())
    }

    pub fn names(&self) -> Vec<String> {
        vec![
            "title".to_string(),
            "artist".to_string(),
            "bpm".to_string(),
            "offset".to_string(),
            "scale".to_string(),
            "genre".to_string(),
            "artstyle".to_string(),
            "lyrics".to_string(),
        ]
    }

    pub fn get(&self, key: &str) -> String {
        match key {
            "title" => self.title.clone(),
            "artist" => self.artist.clone(),
            "bpm" => self.bpm.to_string(),
            "offset" => self.offset.to_string(),
            "scale" => self.scale.clone(),
            "genre" => self.genre.clone(),
            "artstyle" => self.artstyle.clone(),
            "lyrics" => self.lyrics.clone(),
            _ => "".to_string(),
        }
    }

}
