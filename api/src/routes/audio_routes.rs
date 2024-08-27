use std::{path::Path, fs::read_dir};

use models::{UserAuthorizationToken, AudioBuffer, AudioFile};

use crate::app_state::AppState;

#[get("/audio")]
pub fn get_audio(token: UserAuthorizationToken) -> std::io::Result<String> {
    let user_path = Path::new(".db/audio").join(&token.get_id());

    // Search for .temp file in user's directory
    let mut temp_file_path = String::new();

    for entry in read_dir(user_path)? {
        let entry = entry?;
        let path = entry.path();
        let path_str = match path.to_str() {
            Some(path_str) => path_str,
            None => continue,
        };
        if path_str.ends_with(".temp") {
            temp_file_path = path_str.to_string();
            break;
        }
    }

    if temp_file_path.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Temp file not found",
        ));
    }

    let temp_file_path = Path::new(&temp_file_path);

    let file_path = temp_file_path.with_extension(""); // Removes the .temp extension

    let file_path = match file_path.to_str() {
        Some(file_path) => file_path,
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "File path is not valid UTF-8",
            ))
        }
    };

    let audio_data = match AudioBuffer::load_audio_data(file_path) {
        Ok(audio_data) => audio_data,
        Err(e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        }
    };

    let json = serde_json::to_string(&audio_data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    Ok(json)
}


#[get("/audios")]
pub fn get_audios(token: UserAuthorizationToken, 
    state: rocket::State<'_, AppState>
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    Ok(AudioFile::get_audios(token, &state.db_config)?)
}

#[delete("/audio/<audio_id>")]
pub fn delete_audio(
    audio_id: String,
    state: rocket::State<'_, AppState>,
) -> std::io::Result<String> {
    let db_config = &state.db_config;
    let audio_id = match audio_id.parse::<i32>() {
        Ok(audio_id) => audio_id,
        Err(e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        }
    };
    AudioFile::delete_audio(audio_id, db_config)
}