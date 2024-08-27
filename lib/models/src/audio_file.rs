use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::path::{Path, PathBuf};
use std::fs::{self};
use std::io;
use rocket_contrib::json::Json;

use crate::{DatabaseConfig, AudioInfo, Audio, UserAuthorizationToken, AudioView};

pub struct AudioFile {
    user_id: i32,
    audio_id: i32,
}

impl AudioFile {
    pub fn new(user_id: i32, audio_id: i32) -> Self {
        Self { user_id, audio_id }
    }

    pub fn prepare_folder(&self) -> io::Result<PathBuf> {
        let user_path = Path::new(".db").join(self.user_id.to_string()).join(self.audio_id.to_string());
        let audio_folder = user_path;

        if !audio_folder.exists() {
            fs::create_dir_all(&audio_folder)?;
        }

        Ok(audio_folder)
    }

    pub fn get_file_name(&self) -> io::Result<String> {
        let audio_folder = self.prepare_folder()?;
        let paths = fs::read_dir(audio_folder)?;
        for path in paths {
            let path = path?;
            let path = path.path();
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            return Ok(file_name);
        }
        Err(io::Error::new(io::ErrorKind::Other, "File not found"))
    }

    pub fn upload_buffer(&self, buffer: Vec<u8>, database_config: &DatabaseConfig) -> std::result::Result<String, Box<dyn std::error::Error>> {
        log::debug!("upload_buffer: user_id: {}, audio_id: {}", self.user_id, self.audio_id);
        let audio_folder = self.prepare_folder()?;

        log::debug!("upload_buffer: saving file");
        let paths = fs::read_dir(audio_folder)?;
        for path in paths {
            let path = path?;
            let path = path.path();
            self.save_file(&path, &buffer)?;
        }

        log::debug!("upload_buffer: updating audio");
        let mut audio = Audio::by_id(self.audio_id, &database_config)?;
        audio.update_audio_loaded(true, &database_config)?;
        Ok("File uploaded".to_string())
    }

    pub fn prepare_upload(
        &self,
        file_info: Json<AudioInfo>,
        database_config: &DatabaseConfig,
    ) -> std::result::Result<String, Box<dyn std::error::Error>> {
        let audio_id = self.generate_audio_id(file_info.get_size().clone().to_string(), self.user_id.to_string());
        let mut audio = Audio::new(audio_id, self.user_id);
        audio.insert(database_config)?;
        audio.update_audio_size(file_info.get_size(), database_config)?;

        self.prepare_file(file_info.get_name())?;
        let res = format!("{}", audio_id);
        Ok(res)
    }

    fn save_file(&self, file_path: &Path, buffer: &[u8]) -> io::Result<String> {
        let file_path = Path::new(file_path);
        fs::write(file_path, buffer)?;
        Ok("File saved".to_string())
    }

    fn generate_audio_id(&self, audio_size: String, user_id: String) -> i32 {
        let mut hasher = DefaultHasher::new();
        let mut audio_id = String::new();
        audio_id.push_str(&audio_size);
        audio_id.push_str(&user_id);
        hasher.write(audio_id.as_bytes());
        hasher.finish() as i32
    }

    fn prepare_file(&self, audio_name: String) -> io::Result<()> {
        let audio_folder = self.prepare_folder()?;
        let file_path = audio_folder.join(audio_name);
        self.save_file(&file_path, &[])?;
        Ok(())
    }


    pub fn get_audios(
        token: UserAuthorizationToken,
        database_config: &DatabaseConfig,
    ) -> std::io::Result<String> {
        let token = token.get_id();
        let audio_views = AudioView::list_by_token(token, database_config)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

        log::info!("audio_views: {:?}", audio_views);
        let json = serde_json::to_string(&audio_views)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        Ok(json)
    }

    // audio_handler::delete_audio(token, audio_id)
    pub fn delete_audio(audio_id: i32, database_config: &DatabaseConfig) -> std::io::Result<String> {
    match Audio::by_id(audio_id, database_config) {
            Ok(audio) => match audio.delete(database_config) {
                Ok(_) => {
                    let user_path = Path::new(".db").join(audio.get_user_id().to_string());
                    let audio_path = user_path.join(audio_id.to_string());
                    if audio_path.exists() {
                        fs::remove_file(audio_path)?;
                        log::info!("Audio file not found in storage");
                    }
                }
                Err(_) => {
                    log::info!("audio not found");
                }
            },
            Err(_) => {
                log::info!("audio not found");
            }
        };
        Ok("{}".to_string())
    }

}
