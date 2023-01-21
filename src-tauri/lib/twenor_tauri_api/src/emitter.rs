use tauri::{AppHandle, Manager};

use xml_library::dj::NodeWithTracks;
use xml_library::node::Node;
use xml_library::track::Track;
use xml_library::Library;

use twenor_log::Filename;
use twenor_log::Log;
use xml_library::track::TrackDetails;
static LOG: Log = Log::new(Filename::Emitter);

pub fn responde(app: &AppHandle, method: &str, data: &str) {
    if let Err(e) = app.emit_all(method, data) {
        LOG.error(&format!("{} not sent: {}", method, e))
    } else {
        LOG.debug(&format!("{} sent", method))
    }
}

// pub fn library(_app: &AppHandle, _library: &mut Library) {
//     // let data = serde_json::to_string(&library).unwrap();
//     // if let Err(e) = app.emit_all("receive-library", data) {
//     //     LOG.error(&format!("Library not sent: {}", e))
//     // } else {
//     LOG.warn("Library will never send")
//     // }
// }

// pub fn file_dialog(app: &AppHandle, path: &str) {
//     let data = serde_json::to_string(&path).unwrap();
//     if let Err(e) = app.emit_all("receive-file-dialog", data) {
//         LOG.error(&format!("File dialog not sent: {}", e))
//     } else {
//         LOG.debug("File dialog sent")
//     }
// }

// pub fn node(app: &AppHandle, node: &Node) {
//     let data = serde_json::to_string(&node).unwrap();
//     if let Err(e) = app.emit_all("receive-node", data) {
//         LOG.error(&format!("Node not sent: {}", e))
//     } else {
//         LOG.debug("Node sent")
//     }
// }

// pub fn node_with_tracks(app: &AppHandle, node: &NodeWithTracks) {
//     let data = serde_json::to_string(&node).unwrap();
//     if let Err(e) = app.emit_all("receive-node", data) {
//         LOG.error(&format!("Node not sent: {}", e))
//     } else {
//         LOG.debug("Node sent")
//     }
// }

// pub fn track(app: &AppHandle, track: &Track) {
//     let data = serde_json::to_string(&track).unwrap();
//     if let Err(e) = app.emit_all("receive-track", data) {
//         LOG.error(&format!("Track not sent: {}", e))
//     } else {
//         LOG.debug("Track sent")
//     }
// }

// pub fn track_details(app: &AppHandle, track: &TrackDetails) {
//     let data = serde_json::to_string(&track).unwrap();
//     if let Err(e) = app.emit_all("receive-track-details", data) {
//         LOG.error(&format!("Track details not sent: {}", e))
//     } else {
//         LOG.debug("Track details sent")
//     }
// }

// pub fn nodes(app: &AppHandle, nodes: &Vec<&Node>) {
//     let data = serde_json::to_string(&nodes).unwrap();
//     if let Err(e) = app.emit_all("receive-nodes", data) {
//         LOG.error(&format!("Nodes not sent: {}", e))
//     } else {
//         LOG.debug("Nodes sent")
//     }
// }

// pub fn config(app: &AppHandle, library: &Library) {
//     let data = serde_json::to_string(&library.get_config()).unwrap();
//     if let Err(e) = app.emit_all("receive-config", data) {
//         LOG.error(&format!("Settings not sent: {}", e))
//     } else {
//         LOG.debug("Config sent")
//     }
// }

pub fn error(app: &AppHandle, message: &str) {
    LOG.error(message);
    app.emit_all("error", message).unwrap();
}

// pub fn info(app: &AppHandle, message: &str) {
//     LOG.info(message);
//     app.emit_all("info", message).unwrap();
// }

// pub fn debug(app: &AppHandle, message: &str) {
//     LOG.debug(message);
//     app.emit_all("debug", message).unwrap();
// }

// pub fn warn(app: &AppHandle, message: &str) {
//     LOG.warn(message);
//     app.emit_all("warn", message).unwrap();
// }

// pub fn success(app: &AppHandle, message: &str) {
//     LOG.debug(message);
//     app.emit_all("success", message).unwrap();
// }

// just send data and emit "method" as event
pub fn send_response(app: &AppHandle, method: &str, data: &str) {
    if let Err(e) = app.emit_all(method, data) {
        LOG.error(&format!("{} not sent: {}", method, e))
    } else {
        LOG.debug(&format!("{} sent", method))
    }
}
