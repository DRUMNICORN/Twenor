use tauri::{AppHandle, Manager};

// use twenor_manager::dj::NodeWithTracks;
// use twenor_manager::node::Node;
// use twenor_manager::track::Track;
// use twenor_manager::Library;


use log::Filename;
use log::Log;
static LOG: Log = Log::new(Filename::Emitter);

pub fn responde(app: &AppHandle, method: &str, data: &str) {
    if let Err(e) = app.emit_all(method, data) {
        LOG.error(&format!("{} not sent: {}", method, e))
    } else {
        LOG.debug(&format!("{} sent", method))
    }
}

pub fn error(app: &AppHandle, message: &str) {
    LOG.error(message);
    match app.emit_all("error", message) {
        Ok(_) => LOG.debug("Error sent"),
        Err(e) => LOG.error(&format!("Error not sent: {}", e)),
    }
}

// just send data and emit "method" as event
pub fn send_response(app: &AppHandle, method: &str, data: &str) {
    if let Err(e) = app.emit_all(method, data) {
        LOG.error(&format!("{} not sent: {}", method, e))
    } else {
        LOG.debug(&format!("{} sent", method))
    }
}

pub fn send_response_json<T: serde::Serialize>(app: &AppHandle, method: &str, data: &T) {
    let data = serde_json::to_string(&data).unwrap();
    if let Err(e) = app.emit_all(method, data) {
        LOG.error(&format!("{} not sent: {}", method, e))
    } else {
        LOG.debug(&format!("{} sent", method))
    }
}
