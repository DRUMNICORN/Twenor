use crate::handlers::file_handler;
use crate::models::file_info::FileInfo;
use crate::models::user::AuthorizationHeaderToken;
use crate::models::AppState;
use file_handler::upload_file;
use rocket::Data;
use rocket_contrib::json::Json;

#[post("/file/<track_id>", data = "<data>")]
pub fn file(token: AuthorizationHeaderToken, data: Data, track_id: i32,
    state: rocket::State<'_, AppState>
) -> std::io::Result<String> {
    use std::io::Read;
    use std::thread;

    // Spawn a new thread to read the data
    let buffer = thread::spawn(move || {
        let mut buffer = Vec::new();
        let mut data_reader = data.open();
        data_reader.read_to_end(&mut buffer).unwrap();
        buffer
    })
    .join()
    .unwrap();

    let user_id = state.get_user_id_by_token(&token.id.clone());
    match user_id {
        Ok(Some(id)) => {
            return file_handler::upload_file_buffer(buffer, id, track_id);
        }
        Ok(None) => {
            return Ok(String::from("No user found"));
        }
        Err(e) => {
            let error_message = format!("Error: {}", e);
            return Ok(error_message);
        }
    }
}

#[post("/upload", data = "<file_info>", format = "json")]
pub fn upload(
    token: AuthorizationHeaderToken,
    file_info: Json<FileInfo>,
    state: rocket::State<'_, AppState>,
) -> std::io::Result<String> {
    let user_id = state.get_user_id_by_token(&token.id.clone());
    match user_id {
        Ok(Some(id)) => {
            println!("User id: {:?}", id);
            return upload_file(file_info, id, state)
        }
        Ok(None) => {
            println!("No user found");
            return Ok(String::from("No user found"));
        }
        Err(
            e,
        ) => {
            println!("Error: {}", e);
            let error_message = format!("Error: {}", e);
            return Ok(error_message);
        }
    }


}

