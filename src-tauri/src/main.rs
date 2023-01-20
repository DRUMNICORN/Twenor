#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use xml_library;
use xml_library::config::Config;
use xml_library::Library;

use tauri_audio_stream;
use twenor_log::Filename;
use twenor_log::Log;

use std::sync::{Arc, Mutex};
use twenor_tauri_api;

static LOG: Log = Log::new(Filename::Main);
use serde::Serialize;

#[derive(Serialize)]
pub struct MousePos {
    pub x: i32,
    pub y: i32,
}

use tauri::Manager;

fn main() {
    let config = Config::new("../data/config.conf");
    let library = Library::new(config);

    LOG.info("Starting Tauri");
    tauri::Builder::default()
        .setup(move |app| {
            let main_window = app.get_window("main").unwrap();
            twenor_tauri_api::init_listeners(app, Arc::new(Mutex::new(library)));
            match main_window.show() {
                Ok(_) => LOG.info("Main window shown"),
                Err(e) => LOG.error(&format!("Error while showing main window: {}", e)),
            }

            Ok(())
        })
        .register_uri_scheme_protocol("stream", move |_app, request| {
            tauri_audio_stream::handle_request(request)
        })
        // .invoke_handler(twenor_tauri_api::get_invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
