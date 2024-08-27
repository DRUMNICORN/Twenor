use std::collections::HashMap;

// mp3-metadata = "0.3.4"
use mp3_metadata::read_from_file;

// list of requested parameters

use audionodes::Node;

// use crate::log::Filename;
// use crate::log::Log;
// static LOG: Log = Log::new(Filename::TrackMetadataMP3Metadata);

// #[tauri::command]
pub fn get_metadata(path: &str) -> HashMap<String, String> {
    let metadata = read_from_file(path);

    match metadata {
        Ok(results) => {
            // results.frames.clear();
            let string = format!("{:?}", results);

            let nodes = Node::default().read_from_path(path).unwrap();

            // log the results
            // ln!("{}", format!("{:?}", nodes));
            

            // let mut metadata = HashMap::new();
            // metadata.insert("title".to_string(), nodes.title().unwrap_or("".to_string()));
            // metadata.insert("artist".to_string(), nodes.artist().unwrap_or("".to_string()));
            // metadata.insert("album".to_string(), nodes.album().unwrap_or("".to_string()));
            // metadata.insert("year".to_string(), nodes.year().unwrap_or("".to_string()));
            // metadata

            HashMap::new()
        }
        Err(_err) => HashMap::new(),
    }
}
