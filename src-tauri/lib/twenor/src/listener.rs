#![allow(dead_code)]

use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager};

use log::Filename;
use log::Log;

use crate::datatypes::Data;
use crate::Library;

static LOG: Log = Log::new(Filename::Listener);

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
