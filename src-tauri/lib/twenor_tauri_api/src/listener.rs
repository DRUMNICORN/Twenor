#![allow(dead_code)]

use serde::Deserialize;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager};

use xml_library::Library;

use twenor_log::Filename;
use twenor_log::Log;
static LOG: Log = Log::new(Filename::Listener);

#[derive(Clone, Deserialize)]
pub struct FileFilter {
    name: String,
    extensions: Vec<String>,
}

#[derive(Deserialize)]
pub enum Data {
    TrackId {
        track_id: String,
    },
    TrackPath {
        track_path: String,
    },
    NodePathAndTrackId {
        node_path: String,
        track_id: String,
    },
    NodePath {
        node_path: String,
    },
    NodePathAndTrackPath {
        node_path: String,
        track_path: String,
    },
    ReorderedId {
        source_id: usize,
        target_id: usize,
    },
    NodePathAndReorderedId {
        node_path: String,
        source_id: usize,
        target_id: usize,
    },
    KeyAndValue {
        key: String,
        value: String,
    },
    NodeName {
        name: String,
    },
    SourcePathAndTargetPath {
        source_path: String,
        target_path: String,
    },
    XmlPath {
        xml_path: String,
    },
    FileDialog {
        title: String,
        filters: Vec<FileFilter>,
    },
    None,
}

impl Data {
    pub fn unwrap_track_id(&self) -> String {
        match self {
            Data::TrackId { track_id } => track_id.to_string(),
            Data::NodePathAndTrackId { track_id, .. } => track_id.to_string(),
            _ => panic!("Invalid data type"),
        }
    }

    pub fn unwrap_track_path(&self) -> String {
        match self {
            Data::TrackPath { track_path } => track_path.to_string(),
            Data::NodePathAndTrackPath { track_path, .. } => track_path.to_string(),
            _ => panic!("Invalid data type"),
        }
    }

    pub fn unwrap_node_path(&self) -> String {
        match self {
            Data::NodePath { node_path } => node_path.to_string(),
            Data::NodePathAndTrackId { node_path, .. } => node_path.to_string(),
            Data::NodePathAndTrackPath { node_path, .. } => node_path.to_string(),
            _ => panic!("Invalid data type"),
        }
    }

    pub fn unwrap_node_track_id(&self) -> (String, String) {
        match self {
            Data::NodePathAndTrackId {
                node_path,
                track_id,
            } => (node_path.to_string(), track_id.to_string()),
            _ => panic!("Invalid data type"),
        }
    }

    pub fn unwrap_node_track_path(&self) -> (String, String) {
        match self {
            Data::NodePathAndTrackPath {
                node_path,
                track_path,
            } => (node_path.to_string(), track_path.to_string()),
            _ => panic!("Invalid data type"),
        }
    }
    // fn unwrap_node_path_reorder_id(&self) -> (
    pub fn unwrap_reorder_id(&self) -> (usize, usize) {
        match self {
            Data::ReorderedId {
                source_id,
                target_id,
            } => (*source_id, *target_id),
            _ => panic!("Invalid data type"),
        }
    }

    pub fn unwrap_node_path_reorder_id(&self) -> (String, usize, usize) {
        match self {
            Data::NodePathAndReorderedId {
                node_path,
                source_id,
                target_id,
            } => (node_path.to_string(), *source_id, *target_id),
            _ => panic!("Invalid data type"),
        }
    }

    pub fn unwrap_config(&self) -> (String, String) {
        match self {
            Data::KeyAndValue { key, value } => (key.to_string(), value.to_string()),
            _ => panic!("Invalid data type"),
        }
    }

    pub fn unwarp_node_path_and_node_path(&self) -> (String, String) {
        match self {
            Data::SourcePathAndTargetPath {
                source_path,
                target_path,
            } => (source_path.to_string(), target_path.to_string()),
            _ => panic!("Invalid data type"),
        }
    }

    pub fn unwrap_xml_path(&self) -> String {
        match self {
            Data::XmlPath { xml_path } => xml_path.to_string(),
            _ => panic!("Invalid data type"),
        }
    }

    pub fn unwrap_file_dialog(&self) -> (String, Vec<FileFilter>) {
        match self {
            Data::FileDialog { title, filters } => (title.to_string(), filters.to_vec()),
            _ => panic!("Invalid data type"),
        }
    }
}

pub fn listen(
    event: &str,
    app: &mut tauri::App,
    library: Arc<Mutex<Library>>,
    callback: impl Fn(&mut Library, &AppHandle, Data) + std::marker::Send + std::marker::Sync + 'static,
) {
    let event_name = event.to_string();
    let app_handle = app.app_handle();
    app.listen_global(event, move |event| {
        LOG.debug(&format!("Received event: {}", event_name));
        let data = match event.payload() {
            Some(data_str) => {
                let data_json: Result<Data, serde_json::Error> = serde_json::from_str(&data_str);
                match data_json {
                    Ok(data) => data,
                    Err(e) => {
                        LOG.error(&format!("Error parsing event data: {}", e));
                        return;
                    }
                }
            }
            None => Data::None,
        };

        let lib = library.lock();
        match lib {
            Ok(mut lib) => callback(&mut lib, &app_handle, data),
            Err(e) => LOG.error(&format!("Error locking library: {}", e)),
        }
    });
}
