use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AudioInfo {
    name: String,
    size: usize,
    mime_type: String,
    owner: String
}

impl AudioInfo {
    pub fn from_path( audio_path: &str,
    ) -> std::result::Result<(String, i32, i32), Box<dyn std::error::Error>> {
        let extension = audio_path.split('.').last().unwrap_or("").to_lowercase();
        let parent_path = std::path::Path::new(audio_path).parent().ok_or_else(|| {
            log::error!("Failed to get parent directory");
            "Failed to get parent directory"
        })?;
        let user_id = parent_path.parent().ok_or_else(|| {
            log::error!("Failed to get parent directory");
            "Failed to get parent directory"
        })?;
        let user_id = user_id.file_name().ok_or_else(|| {
            log::error!("Failed to get file name");
            "Failed to get file name"
        })?;
        let user_id = user_id.to_str().unwrap_or("0").parse::<i32>().unwrap_or(0);

        let audio_id = parent_path
            .file_name()
            .ok_or_else(|| {
                log::error!("Failed to get file name");
                "Failed to get file name"
            })?
            .to_str()
            .unwrap_or("0")
            .parse::<i32>()
            .unwrap_or(0);

        match extension.as_str() {
            "mp3" => {} // Add other acceptable extensions here
            _ => {
                log::error!("File is not a wav, flac, or mp3 file");
            }
        }

        Ok((extension, audio_id, user_id))
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_mime_type(&self) -> String {
        self.mime_type.clone()
    }

    pub fn get_owner(&self) -> String {
        self.owner.clone()
    }
}
