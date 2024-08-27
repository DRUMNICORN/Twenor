use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::path::{Path, PathBuf};
use std::fs::{self};
use std::io;
use r2d2::PooledConnection;
use r2d2_mysql::mysql::Params;
use r2d2_mysql::MysqlConnectionManager;
use crate::models::AppState;
use crate::models::file_info::FileInfo;
use rocket_contrib::json::Json;

fn prepare_file_folder(user_id: i32, track_id: i32) -> io::Result<PathBuf> {
    let user_path = Path::new(".db").join(user_id.to_string()).join(track_id.to_string());
    let track_folder = user_path;
    
    if !track_folder.exists() {
        fs::create_dir_all(&track_folder)?;
    }
    
    Ok(track_folder)
}


pub fn upload_file_buffer(buffer: Vec<u8>, user_id: i32, track_id: i32) -> io::Result<String> {
    let track_folder = prepare_file_folder(user_id, track_id)?;

    let paths = fs::read_dir(track_folder)?;
    for path in paths {
        let path = path?;
        let path = path.path();
        save_file(&path, &buffer)?;
    }
    Ok("File uploaded".to_string())
}

pub fn upload_file(
    file_info: Json<FileInfo>,
    user_id: i32,
    state: rocket::State<'_, AppState>,
) -> std::io::Result<String>
{
    let track_id = generate_track_id(file_info.size.clone().to_string(), user_id.to_string());
    println!("Track id: {}", track_id);
    prepare_file(user_id, track_id, file_info.name.clone())?;
    let conn = state.db_pool().get().unwrap();
    insert_into_track_list(conn, user_id, track_id).unwrap();
    let res = format!("{}", track_id);
    let res = res.to_string();
    Ok(res)
}

fn save_file(file_path: &Path, buffer: &[u8]) -> io::Result<String> {
    // list all files in the directory
    let file_path = Path::new(file_path);
    fs::write(file_path, buffer)?;
    Ok("File saved".to_string())
}

pub fn generate_track_id(track_size: String, user_id: String) -> i32 {
    let mut hasher = DefaultHasher::new();
    let mut track_id = String::new();
    track_id.push_str(&track_size);
    track_id.push_str(&user_id);
    hasher.write(track_id.as_bytes());
    hasher.finish() as i32
}

pub fn prepare_file(user_id: i32, track_id: i32, track_name: String) -> io::Result<()> {
    let track_folder = prepare_file_folder(user_id, track_id)?;
    let file_path = track_folder.join(track_name);
    save_file(&file_path, &[])?;
    Ok(())
}


pub fn insert_into_track_list(
    mut conn: PooledConnection<MysqlConnectionManager>,
    user_id: i32,
    track_id: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let select_query = "SELECT * FROM TRACK_LIST WHERE user_id = ? AND track_id = ?";
    let params: Params = Params::from((user_id, track_id));
    let result: Option<(i32, i32)> = conn.first_exec(select_query, params)?;
    match result {
        Some(_) => {
            return Ok(());
        }
        None => (),
    }

    let insert_query = "INSERT INTO TRACK_LIST (user_id, track_id) VALUES (?, ?)";
    let params: Params = Params::from((user_id, track_id));
    let result: Option<(i32, i32)> = conn.first_exec(insert_query, params)?;

    match result {
        Some(_) => {
            return Ok(());
        }
        None => {
            return Ok(());
        }
    }
}
