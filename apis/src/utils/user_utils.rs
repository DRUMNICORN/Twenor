use std::io::Result;
use std::path::Path;
use serde::Serialize;

#[derive(Serialize)]
pub struct Waveform {
    pub buffer: Vec<u16>,
}

pub fn _get_audios(user_id: &String) -> std::io::Result<Vec<String>> {
    let user_path = Path::new(".db/audio").join(&user_id);
    let mut audios = Vec::new();
    for entry in std::fs::read_dir(user_path)? {
        let entry = entry?;
        let path = entry.path();
        let path_str = path.to_str().unwrap().to_string();
        audios.push(path_str);
    }
    Ok(audios)
}

pub fn check_user_storage(user_id: &String) -> Result<()> {
    let user_path = Path::new(".db/audio").join(&user_id);
    if !user_path.exists() {
        std::fs::create_dir_all(user_path)?;
    }
    Ok(())
}