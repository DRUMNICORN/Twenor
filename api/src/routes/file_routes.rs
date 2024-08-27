use crate::app_state::AppState;
use std::io::Read;
use std::thread;
use crate::models::audio_info::AudioInfo;
use crate::models::User;
use models::{AudioFile, AudioBuffer, AudioState, Audio};
use rocket::Data;
use rocket_contrib::json::Json;

#[get("/file/<audio_id>")]
pub fn get_file(
    audio_id: i32,
    state: rocket::State<'_, AppState>,
) -> std::io::Result<String> {
    log::info!("get_file: audio_id: {}", audio_id);
    let database_config = &state.db_config;

    log::info!("get_file: looking up user");
    let user_id = match User::by_audio_id(&audio_id, database_config) {
        Ok(Some(user)) => user.id(),
        Ok(None) => {
            log::error!("get_file: no user found");
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No user found",
            ));
        }
        Err(e) => {
            let error_message = format!("Error: {}", e);
            log::error!("get_file: {}", error_message);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                error_message,
            ));
        }
    };
    // find the file name by look 
    let audio_file = AudioFile::new(audio_id, user_id);
    audio_file.get_file_name()
}

#[post("/file/<audio_id>", data = "<data>")]
pub fn post_file(
    data: Data,
    audio_id: i32,
    state: rocket::State<'_, AppState>,
) -> std::result::Result<String, std::io::Error> {
    log::info!("post_file: audio_id: {}", audio_id);

    // skip uploading if file already exists
    let audio = Audio::by_id(audio_id, &state.db_config);
    match audio {
        Ok(audio) => {
            if audio.is_loaded() {
                log::info!("post_file: audio already loaded");
                // check if file state is error
                let audio_state = match AudioState::by_id(audio_id, &state.db_config) {
                    Ok(state) => state,
                    Err(e) => {
                        log::error!("post_file: error getting audio state: {}", e);
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Error getting audio state",
                        ));
                    }
                };
                if !audio_state.is_equal(&AudioState::Error) {
                    log::info!("post_file: audio state is not error - skipping");
                    return Ok(audio_id.to_string());
                }else{
                    match AudioState::reset_state(audio_id,  &state.db_config) {
                        Ok(_) => (),
                        Err(e) => {
                            log::error!("post_file: error resetting audio state: {}", e);
                            return Ok(audio_id.to_string());
                        }
                    }
                    log::info!("post_file: audio state is error");
                }
            }
        }
        Err(e) => {
            log::error!("post_file: error getting audio: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error getting audio",
            ));
        }
    }
    
    log::info!("post_file: reading data");
    let buffer = match thread::spawn(move || {
        let mut buffer = Vec::new();
        
        log::info!("post_file: opening data");
        let mut data_reader = data.open();

        log::info!("post_file: reading data");
        let res = match data_reader.read_to_end(&mut buffer) {
            Ok(res) => {
                log::info!("post_file: read {} bytes", res);
                Ok(buffer)
            }
            Err(e) => {
                log::error!("post_file: error reading data: {}", e);
                Err(e)
            }
        };
        match res {
            Ok(buffer) => {
                log::info!("post_file: read {} bytes", buffer.len());
                Ok(buffer)
            },
            Err(e) => {
                log::error!("post_file: error reading data: {}", e);
                Err(e)
            }
        }
    })
    .join()
    {
        Ok(res) => match { res } {
            Ok(buffer) => {
                log::info!("post_file: read {} bytes", buffer.len());
                buffer
            },
            Err(e) => {
                log::error!("post_file: error reading data: {}", e);
                return Err(e);
            }
        },
        Err(_e) => {
            log::error!("post_file: error reading data");
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error reading data",
            ));
        }
    };

    let database_config = &state.db_config;

    log::info!("post_file: looking up user");
    let user_id = match User::by_audio_id(&audio_id, database_config) {
        Ok(Some(user)) => {
            log::info!("post_file: found user");
            Ok(Some(user.id()))
        },
        Ok(None) => {
            log::error!("post_file: no user found");
            Ok(None)
        }
        Err(e) => Err(format!("Error on user lookup: {}", e)),
    };
    match user_id {
        Ok(Some(id)) => {
            log::debug!("post_file: uploading file");
            let audio_file = AudioFile::new(audio_id, id);
            match audio_file.upload_buffer(buffer, database_config) {
                Ok(_) => {
                    log::info!("post_file: uploaded file");
                    return Ok(audio_id.to_string());
                }
                Err(e) => {
                    log::error!("post_file: error uploading file: {}", e);
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Error on file upload: {}", e),
                    ));
                }
            }
        }
        Ok(None) => {
            log::error!("post_file: no user found");
            return Ok(String::from("No user found"));
        }
        Err(e) => {
            let error_message = format!("Error on user lookup: {}", e);        
            log::error!("post_file: {}", error_message);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                error_message,
            ));
        }
    }
}

#[post("/upload", data = "<file_info>", format = "json")]
pub fn post_upload(
    file_info: Json<AudioInfo>,
    state: rocket::State<'_, AppState>,
) -> std::io::Result<String> {
    log::info!("post_upload: file_info: {:?}", file_info);
    let database_config = &state.db_config;

    log::debug!("post_upload: looking up user");
    let user_id = match User::from_user_name(&file_info.get_owner(), database_config) {
        Ok(Some(user)) => Ok(Some(user.id())),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    };

    match user_id {
        Ok(Some(id)) => {
            let db_config = &state.db_config;
            let audio_file = AudioFile::new(0, id);
            return match audio_file.prepare_upload(file_info, db_config) {
                Ok(res) => {
                    log::info!("post_upload: prepared upload");
                    Ok(res.to_string())
                },
                Err(e) => {
                    let error_message = format!("Error: {}", e);
                    log::error!("post_upload: {}", error_message);
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        error_message,
                    ));
                }
            };
        }
        Ok(None) => {
            log::error!("post_upload: no user found");
            return Ok(String::from("No user found"));
        }
        Err(e) => {
            let error_message = format!("Error: {}", e);
            log::error!("post_upload: {}", error_message);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                error_message,
            ));
        }
    }
}

#[get("/download/<audio_id>")]
pub fn get_download(
    audio_id: i32,
    state: rocket::State<'_, AppState>,
) -> std::io::Result<std::fs::File> {
    log::info!("get_download: audio_id: {}", audio_id);
    let database_config = &state.db_config;

    log::info!("get_download: looking up user");
    let user_id = match User::by_audio_id(&audio_id, database_config) {
        Ok(Some(user)) => Ok(Some(user.id())),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    };

    match user_id {
        Ok(Some(id)) => {
            log::info!("get_download: downloading file");
            return match AudioBuffer::download_file(id, audio_id, database_config) {
                Ok(buffer) => {
                    log::info!("get_download: downloaded file");
                    Ok(buffer)
                }
                Err(e) => {
                    let error_message = format!("Error: {}", e);
                    log::error!("get_download: {}", error_message);
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        error_message,
                    ));
                }
            };
        }
        Ok(None) => {
            log::error!("get_download: no user found");
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No user found",
            ));
        }
        Err(e) => {
            let error_message = format!("Error: {}", e);
            log::error!("get_download: {}", error_message);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                error_message,
            ));
        }
    }
}