use super::DatabaseConfig;
use r2d2_mysql::mysql::{prelude::Queryable, Params};
pub struct Audio {
    audio_id: i32,
    user_id: i32,
    audio_size: usize,
    audio_loaded: bool,
}

impl Audio {
        pub fn from_row(
        row: r2d2_mysql::mysql::Row,
    ) -> std::result::Result<Audio, Box<dyn std::error::Error>> {
        let audio_id = row
            .get("audio_id")
            .ok_or_else(|| "Failed to get audio_id")?;
        let user_id = row.get("user_id").ok_or_else(|| "Failed to get user_id")?;
        let audio_size = row
            .get("audio_size")
            .ok_or_else(|| "Failed to get audio_size")?;
        let audio_loaded = row
            .get("audio_loaded")
            .ok_or_else(|| "Failed to get audio_loaded")?;
        Ok(Audio {
            audio_id,
            user_id,
            audio_size,
            audio_loaded,
        })
    }

    pub fn new(audio_id: i32, user_id: i32) -> Self {
        Audio {
            audio_id,
            user_id,
            audio_size: 0,
            audio_loaded: false,
        }
    }

    pub fn update_audio_size(
        &mut self,
        audio_size: usize,
        database_config: &DatabaseConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.audio_size = audio_size;
        self.update(database_config)
    }

    pub fn update_audio_loaded(
        &mut self,
        audio_loaded: bool,
        database_config: &DatabaseConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.audio_loaded = audio_loaded;
        self.update(database_config)
    }

    pub fn update(
        &self,
        database_config: &DatabaseConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection()?;
        let query = "UPDATE AUDIO_LIST SET audio_size = ?, audio_loaded = ? WHERE audio_id = ?";
        let params: Params =
            Params::from((self.audio_size.clone(), self.audio_loaded, self.audio_id));
        conn.exec_first::<usize, &str, Params>(query, params)?;
        Ok(())
    }

    pub fn by_id(
        audio_id: i32,
        database_config: &DatabaseConfig,
    ) -> std::result::Result<Audio, Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection()?;

        let query = "SELECT * FROM AUDIO_LIST WHERE audio_id = ?";
        let params: Params = Params::from((audio_id,));
        let result = conn.exec_first::<r2d2_mysql::mysql::Row, &str, Params>(query, params)?;

        let row = match result {
            Some(row) => row,
            None => {
                return {
                    log::info!("Audio::by_id: no audio found");
                    Err("No audio found".into())
                }
            }
        };
        Audio::from_row(row)
    }



    pub fn insert(
        &self,
        database_config: &DatabaseConfig,
    ) -> Result<Option<&Audio>, Box<dyn std::error::Error>> {
        let select_query = "SELECT * FROM AUDIO_LIST WHERE user_id = ? AND audio_id = ?";
        let params: Params = Params::from((self.user_id, self.audio_id));
        let mut conn = database_config.get_connection()?;

        let result =
            conn.exec_first::<r2d2_mysql::mysql::Row, &str, Params>(select_query, params)?;
        if let Some(_) = result {
            return Ok(None);
        }

        let insert_query = "INSERT INTO AUDIO_LIST (user_id, audio_id, audio_size, audio_loaded) VALUES (?, ?, ?, ?)";
        let params: Params = Params::from((
            self.user_id,
            self.audio_id,
            self.audio_size,
            self.audio_loaded,
        ));
        conn.exec_first::<usize, &str, Params>(insert_query, params)?;

        Ok(Some(self.clone()))
    }
    
    pub fn delete(
        &self,
        database_config: &DatabaseConfig,
    ) -> Result<Option<&Audio>, Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection()?;
        
        // clear AUDIO_STATE
        let query = "DELETE FROM AUDIO_STATE WHERE audio_id = ?";
        let params: Params = Params::from((self.audio_id,));
        match conn.exec_first::<usize, &str, Params>(query, params) {
            Ok(_) => {},
            Err(e) => {
                log::info!("Audio::delete: AUDIO_STATE: {:?}", e);
            }
        }
        
        // clear SCRIPT_LIST
        let query = "DELETE FROM SCRIPT_LIST WHERE audio_id = ?";
        let params: Params = Params::from((self.audio_id,));
        match conn.exec_first::<usize, &str, Params>(query, params) {
            Ok(_) => {},
            Err(e) => {
                log::info!("Audio::delete: SCRIPT_LIST: {:?}", e);
            }
        }

        // clear SCENE_LIST
        let query = "DELETE FROM SCENE_LIST WHERE audio_id = ?";
        let params: Params = Params::from((self.audio_id,));
        match conn.exec_first::<usize, &str, Params>(query, params) {
            Ok(_) => {},
            Err(e) => {
                log::info!("Audio::delete: SCENE_LIST: {:?}", e);
            }
        }

        // clear AUDIO_FEATURES
        let query = "DELETE FROM AUDIO_FEATURES WHERE audio_id = ?";
        let params: Params = Params::from((self.audio_id,));
        match conn.exec_first::<usize, &str, Params>(query, params)  {
            Ok(_) => {},
            Err(e) => {
                log::info!("Audio::delete: AUDIO_FEATURES: {:?}", e);
            }
        }

        // clear AUDIO_CHUNK 
        let query = "DELETE FROM AUDIO_CHUNK WHERE audio_id = ?";
        let params: Params = Params::from((self.audio_id,));
        match conn.exec_first::<usize, &str, Params>(query, params) {
            Ok(_) => {},
            Err(e) => {
                log::info!("Audio::delete: AUDIO_CHUNK: {:?}", e);
            }
        }

        // clear AUDIO_CORRLATION
        let query = "DELETE FROM AUDIO_CORRLATION WHERE audio_id = ?";
        let params: Params = Params::from((self.audio_id,));
        match conn.exec_first::<usize, &str, Params>(query, params){
            Ok(_) => {},
            Err(e) => {
                log::info!("Audio::delete: AUDIO_CORRLATION: {:?}", e);
            }
        }

        // clear AUDIO_METADATA
        let query = "DELETE FROM AUDIO_METADATA WHERE audio_id = ?";
        let params: Params = Params::from((self.audio_id,));
        match conn.exec_first::<usize, &str, Params>(query, params) {
            Ok(_) => {},
            Err(e) => {
                log::info!("Audio::delete: AUDIO_METADATA: {:?}", e);
            }
        }

        // clear AUDIO_LIST
        let query = "DELETE FROM AUDIO_LIST WHERE audio_id = ?";
        let params: Params = Params::from((self.audio_id,));
        match conn.exec_first::<usize, &str, Params>(query, params){    
            Ok(_) => {},
            Err(e) => {
                log::info!("Audio::delete: AUDIO_LIST: {:?}", e);
            }
        }

        Ok(Some(self.clone()))
    }

    pub fn get_audio_id(&self) -> i32 {
        self.audio_id
    }

    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }

    pub fn is_loaded(&self) -> bool {
        self.audio_loaded
    }
}
