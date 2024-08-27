#![allow(dead_code)]
use std::sync::{Arc, Mutex};

use rfd::FileDialog;

use tauri::AppHandle;

use xml_library::Library;

pub mod emitter;
pub mod listener;
use listener::listen;
use listener::Data;
use twenor_log::Filename;
use twenor_log::Log;
static LOG: Log = Log::new(Filename::Listener);

// =================================================================================================
pub fn init_listeners(a: &mut tauri::App, l: Arc<Mutex<Library>>) {
    listen("request-library", a, l.clone(), on_request_library);
    listen("request-node", a, l.clone(), on_request_node);
    listen("request-nodes", a, l.clone(), on_request_nodes);
    listen("request-track", a, l.clone(), on_request_track);
    listen("request-config", a, l.clone(), on_request_config);
    listen("request-reload", a, l.clone(), on_request_reload);
    listen("update-config", a, l.clone(), on_update_config);
    listen("add-track", a, l.clone(), on_add_track);
    listen("add-node", a, l.clone(), on_add_node);
    listen("add-node-to-track", a, l.clone(), on_add_node_to_track);
    listen("add-track-with-node", a, l.clone(), on_add_track_with_node);
    listen("open-file-dialog", a, l.clone(), on_open_file_dialog);
    listen("move-node", a, l.clone(), move_node);
    LOG.info("Listeners initialized");
}

fn on_request_library(library: &mut Library, app_handle: &AppHandle, _data: Data) {
    emitter::library(app_handle, library)
}

fn on_request_node(library: &mut Library, app_handle: &AppHandle, data: Data) {
    let node_path = data.unwrap_node_path();
    let node_with_tracks = library.get_playlist_with_tracks(node_path.clone());
    LOG.debug(&format!("Node with tracks: {:?}", node_with_tracks));
    if !node_with_tracks.is_ok() {
        LOG.error("Node not found");
        emitter::error(app_handle, "Node not found");
        return;
    }
    emitter::node_with_tracks(app_handle, &node_with_tracks.unwrap());
}

fn on_request_nodes(library: &mut Library, app_handle: &AppHandle, _data: Data) {
    let nodes = library.get_nodes();
    emitter::nodes(app_handle, &nodes);
}

fn on_request_track(library: &mut Library, app_handle: &AppHandle, data: Data) {
    let track_path = data.unwrap_track_path();
    let track = match library.get_track(track_path.clone()) {
        Ok(track) => track,
        Err(_) => library.add_track(track_path).unwrap(),
    };

    emitter::track_details(app_handle, track);
}

fn on_request_config(library: &mut Library, app_handle: &AppHandle, _data: Data) {
    emitter::config(app_handle, library);
}

fn on_request_reload(library: &mut Library, app_handle: &AppHandle, _data: Data) {
    match library.reload() {
        Ok(_) => {
            emitter::library(app_handle, library);
            // emitter::config(app_handle, library);
            emitter::nodes(app_handle, &library.get_nodes());
            emitter::success(app_handle, "library reloaded");
        }
        Err(err) => {
            LOG.error(&format!("library not reloaded: {}", err));
            emitter::error(app_handle, "library not reloaded");
        }
    }
}

fn on_update_config(library: &mut Library, app_handle: &AppHandle, data: Data) {
    let (key, value) = data.unwrap_config();
    library.update_config(key, value);

    // ====== Done ======
    emitter::config(app_handle, library);
    match library.save() {
        Ok(_) => LOG.debug("library saved"),
        Err(err) => LOG.error(&format!("library not saved: {}", err)),
    }
}

fn on_add_track(library: &mut Library, app_handle: &AppHandle, data: Data) {
    let track_path = data.unwrap_track_path();
    let track = library.add_track(track_path);
    match track {
        Ok(track) => {
            emitter::track_details(app_handle, track);
            match library.save() {
                Ok(_) => LOG.debug("library saved"),
                Err(err) => LOG.error(&format!("library not saved: {}", err)),
            }
        }
        Err(err) => {
            LOG.error(&format!("Track not added: {}", err));
            emitter::error(app_handle, "Track not added");
        }
    }
}

fn on_add_node(library: &mut Library, app_handle: &AppHandle, data: Data) {
    println!("on_add_node");
    let node_path = data.unwrap_node_path();
    LOG.debug(&format!("Adding node: {}", node_path));
    let node = library.add_node(node_path);
    if !node.is_ok() {
        LOG.error("Node not found");
        emitter::error(app_handle, "Node not found");
        return;
    }

    // ====== Done ======
    emitter::nodes(app_handle, &library.get_nodes());
    match library.save() {
        Ok(_) => LOG.debug("library saved"),
        Err(err) => LOG.error(&format!("library not saved: {}", err)),
    }
}

fn on_add_node_to_track(library: &mut Library, app_handle: &AppHandle, data: Data) {
    let (track_path, node_path) = data.unwrap_node_track_path();
    let track = library.add_node_to_track(track_path, node_path.clone());
    if !track.is_ok() {
        LOG.error("Track not found");
        emitter::error(app_handle, "Track not found");
        return;
    }
    emitter::track_details(app_handle, track.unwrap());

    let node_with_tracks = library.get_playlist_with_tracks(node_path);
    if !node_with_tracks.is_ok() {
        LOG.error("Node not found");
        emitter::error(app_handle, "Node not found");
        return;
    }
    emitter::node_with_tracks(app_handle, &node_with_tracks.unwrap());

    match library.save() {
        Ok(_) => LOG.debug("library saved"),
        Err(err) => LOG.error(&format!("library not saved: {}", err)),
    }
}

fn on_add_track_with_node(library: &mut Library, app_handle: &AppHandle, data: Data) {
    LOG.debug("== Adding track with node ==");
    // arcmutex from library
    let lib_mutex = Arc::new(Mutex::new(library));

    let (node_path, track_path) = data.unwrap_node_track_path();
    let node_path = node_path.split('/').last().unwrap();

    // add_track
    {
        let mut locked = lib_mutex.lock().unwrap();
        let track = locked.add_track(track_path.clone());
        if !track.is_ok() {
            LOG.error("Track not added");
            emitter::error(app_handle, "Track not added");
            return;
        }

        LOG.debug(&format!(
            "Adding node to track: {} - {}",
            node_path,
            track_path.clone().to_owned()
        ));
    }

    let mut locked = lib_mutex.lock().unwrap();
    let track = locked.add_node_to_track(track_path, node_path.clone().to_owned());
    if !track.is_ok() {
        LOG.error("Track not found");
        emitter::error(app_handle, "Track not found");
        return;
    }

    emitter::track_details(app_handle, track.unwrap());

    let node_with_tracks = locked.get_playlist_with_tracks(node_path.clone().to_owned());
    if !node_with_tracks.is_ok() {
        LOG.error("Node not found");
        emitter::error(app_handle, "Node not found");
        return;
    }
    emitter::node_with_tracks(app_handle, &node_with_tracks.unwrap());

    match locked.save() {
        Ok(_) => LOG.debug("library saved"),
        Err(err) => LOG.error(&format!("library not saved: {}", err)),
    }
}

fn move_node(library: &mut Library, app_handle: &AppHandle, data: Data) {
    let (source_path, target_path) = data.unwarp_node_path_and_node_path();
    LOG.debug(&format!("Moving node: {} to {}", source_path, target_path));
    match library.move_node(source_path, target_path) {
        Ok(_) => {
            emitter::nodes(app_handle, &library.get_nodes());
            library.save().unwrap();
        }
        Err(err) => {
            LOG.error(&format!("Node not moved: {}", err));
            emitter::error(app_handle, "Node not moved");
        }
    }

    // ====== Done ======
    emitter::nodes(app_handle, &library.get_nodes());
    match library.save() {
        Ok(_) => LOG.debug("library saved"),
        Err(err) => LOG.error(&format!("library not saved: {}", err)),
    }
}

// Utils

fn on_open_file_dialog(_library: &mut Library, app_handle: &AppHandle, _data: Data) {
    LOG.debug("== Open file dialog ==");
    let files = FileDialog::new()
        .add_filter("All", &["*"])
        .set_directory("/")
        .pick_file();

    match files {
        Some(file) => {
            LOG.debug(&format!("File: {}", file.display()));
        }
        None => {
            LOG.debug("No file selected");
            emitter::error(app_handle, "No file selected");
        }
    }
}
