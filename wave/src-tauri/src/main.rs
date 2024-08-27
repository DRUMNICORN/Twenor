#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use log::Filename;
use log::Log;
use tauri_audio_stream;

use std::sync::{Arc, Mutex};
use twenor;
use twenor::config::Config;

static LOG: Log = Log::new(Filename::Main);
use serde::Serialize;

#[derive(Serialize)]
pub struct MousePos {
    pub x: i32,
    pub y: i32,
}

fn main() {
    let config_path = "../data/config.json".to_string();
    let config = match Config::from_file(config_path.clone()) {
        Ok(config) => config,
        Err(e) => {
            LOG.error(&format!("Error while loading config: {}", e));
            Config::default(config_path.clone())
        }
    };

    let library = twenor::Library::new(config);

    LOG.info("Starting Tauri");
    tauri::Builder::default()
        .setup(move |app| {
            twenor::init_listeners(app, Arc::new(Mutex::new(library)));
            Ok(())
        })
        .register_uri_scheme_protocol("stream", move |_app, request| {
            tauri_audio_stream::handle_request(request)
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
