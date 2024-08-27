use std::collections::HashMap;
use std::path::Path;

use mp3_duration;

// use twenor_log::Filename;
// use crate::api::tools::log::Log;
// static LOG: Log = Log::new(Filename::TrackMetadataDuration);

pub fn read(track_path: &str) -> HashMap<String, String> {
    let track_extension = Path::new(track_path).extension().unwrap().to_str().unwrap();

    let duration = match track_extension {
        "mp3" => get_duration_mp3(track_path),
        _ => "00:00".to_string(),
    };

    let mut metadata = HashMap::new();
    metadata.insert("duration".to_string(), duration);
    metadata
}

pub fn get_duration_mp3(track_path: &str) -> String {
    // when track is mp3, get duration
    let path = Path::new(track_path);
    let duration = mp3_duration::from_path(path).unwrap().as_secs();
    let minutes = (duration / 60) % 60;
    let seconds = duration % 60;
    // set duration to 00:00 if duration is 0
    if duration == 0 {
        "00:00".to_string()
    } else {
        // LOG.debug(&format!("{}:{}", minutes, seconds));
        format!("{:02}:{:02}", minutes, seconds)
    }
}
