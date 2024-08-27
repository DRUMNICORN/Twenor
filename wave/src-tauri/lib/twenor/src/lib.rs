pub mod config;
use config::Config;

pub mod datatypes;
use datatypes::state_to_hashmap;
use datatypes::Data;
use datatypes::Property;
use datatypes::StateParameterTypes;

pub mod emitter;
pub mod listener;
use listener::listen;
use recordbox::node::NodeWithTracks;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

extern crate tauri;
use tauri::AppHandle;

extern crate serde;

use recordbox::dj::DjPlaylists;
use recordbox::node::Node;
use recordbox::node::NodeSimplified;
use recordbox::track::TrackDetails;

use anyhow::anyhow as ah;
use anyhow::Result;

extern crate log;
use log::Filename;
use log::Log;
static LOG: Log = Log::new(Filename::Api);

// =================================================================================================
pub fn init_listeners(a: &mut tauri::App, l: Arc<Mutex<Library>>) {
    listen(
        "REQUEST_RELOAD",
        a,
        l.clone(),
        |library: &mut Library, app_handle: &AppHandle, _data: Data| {
            LOG.info("Received request to reload library");
            match library.reload() {
                Ok(_) => {
                    emitter::send_response_json(
                        app_handle,
                        "RECEIVE_NODES",
                        &library.get_nodes_simplified(),
                    );
                }
                Err(err) => {
                    LOG.error(&format!("library not reloaded: {}", err));
                    emitter::error(app_handle, "library not reloaded");
                }
            }
        },
    );

    listen(
        "GET_NODES",
        a,
        l.clone(),
        |library: &mut Library, app_handle: &AppHandle, _data: Data| {
            LOG.info("Received request to get nodes");
            emitter::send_response_json(
                app_handle,
                "RECEIVE_NODES",
                &library.get_nodes_simplified(),
            );
        },
    );

    listen(
        "GET_XML_PATH",
        a,
        l.clone(),
        |library: &mut Library, app_handle: &AppHandle, _data: Data| {
            LOG.info("Received request to get xml path");
            let path = library.get_xml_path();
            emitter::responde(app_handle, "RECEIVE_XML_PATH", &path);
        },
    );

    listen(
        "SAVE_STATE",
        a,
        l.clone(),
        |library: &mut Library, app_handle: &AppHandle, data: Data| {
            LOG.info("Received request to save state");
            let state = data.unwrap_state();
            match library.save_state(state) {
                Ok(_) => emitter::responde(app_handle, "STATE_SAVED", &""),
                Err(e) => emitter::error(app_handle, &format!("Error saving state: {}", e)),
            }
        },
    );

    listen(
        "GET_STATE",
        a,
        l.clone(),
        |library: &mut Library, app_handle: &AppHandle, _data: Data| {
            LOG.info("Received request to get state");
            let state = library.get_state();
            LOG.debug(&format!("State: {:?}", state));
            emitter::send_response_json(app_handle, "RECEIVE_STATE", &state);
        },
    );

    listen(
        "REQUEST_TRACKS",
        a,
        l.clone(),
        |library: &mut Library, app_handle: &AppHandle, data: Data| {
            LOG.info("Received request to get tracks");
            let node_path = data.unwrap_string();
            let tracks = match library.get_tracks(&node_path) {
                Ok(tracks) => tracks,
                Err(e) => {
                    LOG.error(&format!("Error getting tracks: {}", e));
                    Vec::new()
                }
            };
            emitter::send_response_json(app_handle, "RECEIVE_TRACKS", &tracks);
        },
    );

    LOG.info("Listeners initialized");
}

#[derive(Debug, PartialEq, Clone)]
pub struct Library {
    pub config: Config,
    pub dj_playlists: DjPlaylists,
}

impl Library {
    pub fn new(config: Config) -> Self {
        let xml_path = match config.get("xml_path") {
            Some(xml_path) => xml_path.clone().to_display(),
            None => "library.xml".to_string(),
        };

        let library = Library {
            config,
            dj_playlists: match DjPlaylists::from_file(&xml_path) {
                Ok(dj_playlists) => dj_playlists,
                Err(e) => {
                    LOG.error(&format!("Error loading dj_playlists from xml_path: {}", e));
                    DjPlaylists::new()
                }
            },
        };

        library
    }

    pub fn get_tracks(&self, node_path: &str) -> Result<Vec<TrackDetails>> {
        // find the node in the dj_playlists

        LOG.debug(&format!("node_path: {}", node_path));

        let node = self.dj_playlists.playlists.get_root_parent();
        LOG.debug(&format!("node name: {}", node.name));

        let node = match node.get_node_by_path(node_path.to_string()) {
            Ok(node) => node,
            Err(e) => {
                LOG.error(&format!("Error getting node by path: {}", e));
                return Err(ah!(e));
            }
        };

        let tracks = node.get_tracks_from_collection_with_sub_tracks(&self.dj_playlists.collection);

        match tracks {
            Ok(tracks) => Ok(tracks),
            Err(e) => {
                LOG.error(&format!("Error getting tracks from node: {}", e));
                Err(ah!(e))
            }
        }
    }

    pub fn get_xml_path(&self) -> String {
        match self.config.get("xml_path") {
            Some(xml_path) => xml_path.to_display(),
            None => "library.xml".to_string(),
        }
    }

    pub fn get_state(&self) -> Vec<Property> {
        let mut state: Vec<Property> = Vec::new();

        let config_hashmap = self.config.to_hashmap();
        for (key, value) in config_hashmap.iter() {
            state.push(Property {
                key: key.to_string(),
                value: value.clone(),
            });
        }

        state
    }

    pub fn save_state(&mut self, state: Vec<Property>) -> Result<()> {
        // save the state to the config
        let state_hashmap = state_to_hashmap(state);
        for (key, value) in state_hashmap.iter() {
            self.config.set(key, value.clone());
        }

        match self.config.save() {
            Ok(_) => LOG.info("Saved config"),
            Err(e) => LOG.error(&format!("Error saving config: {}", e)),
        }

        // save the dj_playlists to the xml file

        let xml_path = self.get_xml_path();

        // load the dj_playlists from the xml file

        if !xml_path.is_empty() {
            LOG.info(&format!("Loading dj_playlists from xml_path: {}", xml_path));
            self.dj_playlists = match DjPlaylists::from_file(&xml_path) {
                Ok(dj_playlists) => {
                    LOG.info(&format!("Loaded dj_playlists from xml_path: {}", xml_path));
                    dj_playlists
                }
                Err(e) => {
                    LOG.error(&format!("Error loading dj_playlists from xml_path: {}", e));
                    DjPlaylists::new()
                }
            };
        }

        LOG.info(&format!("_Saving dj_playlists to xml_path: {}", xml_path));
        Ok(())
        // match self.dj_playlists.to_file(&xml_path) {
        //     Ok(_) => {
        //         LOG.info(&format!("Saved dj_playlists to xml_path: {}", xml_path));
        //         Ok(())
        //     }
        //     Err(e) => {
        //         LOG.error(&format!("Error saving dj_playlists to xml_path: {}", e));
        //         Err(e)
        //     }
        // }

        // save the collection to the xml file

        // let collection_xml_path = self.config.get("collection_xml_path");
        // match collection_xml_path {
        //     Some(collection_xml_path) => {
        //         self.dj_playlists.collection.save_to_file(collection_xml_path)
        //     }
        //     None => Err(ah!("collection_xml_path not set")),
        // }
    }

    pub fn get_node_by_path_mut(&mut self, path: String) -> Result<&mut Node> {
        for node in self.dj_playlists.playlists.node.iter_mut() {
            let node = node.get_node_by_path_mut(path.clone());
            if node.is_ok() {
                return node;
            }
        }
        Err(ah!("Node not found"))
    }

    pub fn get_playlist_with_tracks(&self, path: String) -> Result<NodeWithTracks> {
        let collection_clone = self.dj_playlists.collection.clone();
        let path: Vec<&str> = path.split("/").collect();
        let first_element = path[0];
        let node = self
            .dj_playlists
            .playlists
            .node
            .iter()
            .find(|node| node.name == first_element);
        if node.is_none() {
            return Err(ah!("Node not found"));
        }
        if path.len() == 1 {
            match (match node {
                Some(node) => node,
                None => return Err(ah!("Node not found")),
            })
            .get_node_with_tracks(collection_clone, path.join("/"))
            {
                Ok(node) => Ok(node),
                Err(e) => Err(e),
            }
        } else {
            let mut new_path = String::new();
            for i in 1..path.len() {
                new_path.push_str(path[i]);
                if i != path.len() - 1 {
                    new_path.push_str("/");
                }
            }
            match (match node {
                Some(node) => node,
                None => return Err(ah!("Node not found")),
            })
            .get_node_with_tracks(collection_clone, new_path)
            {
                Ok(node) => Ok(node),
                Err(e) => Err(e),
            }
        }
    }

    pub fn get_nodes(&self) -> Vec<&Node> {
        self.dj_playlists.playlists.node.iter().collect()
    }

    pub fn get_nodes_simplified(&self) -> Vec<NodeSimplified> {
        self.dj_playlists
            .playlists
            .node
            .iter()
            .map(|node| node.to_simplified())
            .collect()
    }

    pub fn get_config(&self) -> HashMap<String, StateParameterTypes> {
        self.config.to_hashmap()
    }

    pub fn save(&self) -> Result<()> {
        LOG.info(&format!(
            "Saving dj_playlists to xml_path: {}",
            self.get_xml_path()
        ));
        Ok(())
    }

    pub fn move_node(&mut self, source_path: String, destination_path: String) -> Result<()> {
        let source_parent_path = source_path
            .split("/")
            .take(source_path.split("/").count() - 1)
            .collect::<Vec<&str>>()
            .join("/");

        // locate source node and destination node
        // if source node is not found, return error
        // if destination node is not found, return error
        // if source node is found, and destination node is found, move source node to destination node

        let arcmut_lib_locked = Arc::new(Mutex::new(self.clone()));

        let arcmut_lib = arcmut_lib_locked.clone();
        let mut arcmut_lib = arcmut_lib.lock().unwrap();
        let source_node = arcmut_lib.get_node_by_path_mut(source_path.clone());
        if source_node.is_err() {
            return Err(ah!("Source node not found"));
        }
        let source_node = match source_node {
            Ok(source_node) => source_node,
            Err(e) => return Err(e),
        };

        let arcmut_lib = arcmut_lib_locked.clone();
        let mut arcmut_lib = arcmut_lib.lock().unwrap();
        let destination_node = arcmut_lib.get_node_by_path_mut(destination_path.clone());
        if destination_node.is_err() {
            return Err(ah!("Destination node not found"));
        }
        let destination_node = destination_node.unwrap();

        let arcmut_lib = arcmut_lib_locked.clone();
        let mut arcmut_lib = arcmut_lib.lock().unwrap();
        let source_node_parent = arcmut_lib.get_node_by_path_mut(source_parent_path.clone());
        if source_node_parent.is_err() {
            return Err(ah!("Source node parent not found"));
        }
        let source_node_parent = match source_node_parent {
            Ok(source_node_parent) => source_node_parent,
            Err(e) => return Err(e),
        };

        // add source node to destination node

        destination_node.add_node(source_node.clone());

        // remove source node from source node parent

        let mut new_source_node_parent_node = Vec::new();

        for node in source_node_parent.node.iter() {
            if node.name != source_node.name {
                new_source_node_parent_node.push(node.clone());
            }
        }

        source_node_parent.node = new_source_node_parent_node;

        // save library

        self.save()?;

        return Ok(());
    }

    pub fn get_track(&self, track_path: String) -> Result<&TrackDetails> {
        // find track in collection

        let track_path = track_path.replace("\\", "/");
        let track_path = track_path.replace("//", "/");
        // let track_name = track_path.split("/").last().unwrap().to_string();

        let track = self
            .dj_playlists
            .collection
            .track
            .iter()
            .find(|track| track.location == track_path);

        if track.is_none() {
            return Err(ah!("Track not found"));
        }
        Ok(track.unwrap())
    }

    pub fn add_track(&mut self, track_path: String) -> Result<&TrackDetails> {
        // should add  a track to collection

        let track_path = track_path.replace("\\", "/");
        let track_path = track_path.replace("//", "/");
        let track_name = track_path.split("/").last().unwrap().to_string();

        let new_track = TrackDetails {
            name: track_name,
            location: track_path.to_string(),
            ..Default::default()
        };

        self.dj_playlists.collection.track.push(new_track.clone());

        self.save()?;

        Ok(self
            .dj_playlists
            .collection
            .track
            .iter()
            .find(|track| track.location == track_path)
            .unwrap())
    }

    // should load from xml file
    // and replace the current library
    pub fn reload(&mut self) -> Result<()> {
        // reload config

        // match self.get_config().get("config_path") {
        //     Some(path) => {
        //         let path = path.replace("\\", "/");
        //         let path = path.replace("//", "/");

        //         let config = match Config::new(path) {
        //             Ok(config) => config,
        //             Err(e) => return Err(ah!(e)),
        //         };
        //     }
        //     None => return Err(ah!("Config path not found")),
        // }

        // reload dj_playlists
        // let xml_path = self.get_xml_path();
        // let xml_path = xml_path.replace("\\", "/");
        // let xml_path = xml_path.replace("//", "/");

        // let mut dj_playlists = DjPlaylists::new();

        // match dj_playlists.load_from_file(&xml_path) {
        //     Ok(dj_playlists) => dj_playlists,
        //     Err(e) => return Err(ah!(e)),
        // };
        let config: Config = match self.get_config().get("config_path") {
            Some(path) => {
                let path = path.to_display();
                let path = path.replace("\\", "/");
                let path = path.replace("//", "/");

                match Config::from_file(path) {
                    Ok(config) => config,
                    Err(e) => return Err(ah!(e)),
                }
            }
            None => return Err(ah!("Config path not found")),
        };

        let dj_playlists: DjPlaylists = match self.get_config().get("xml_path") {
            Some(path) => {
                let path = path.to_display();
                let path = path.replace("\\", "/");
                let path = path.replace("//", "/");

                match DjPlaylists::from_file(&path) {
                    Ok(dj_playlists) => dj_playlists,
                    Err(_) => DjPlaylists::new(),
                }
            }
            None => return Err(ah!("Xml path not found")),
        };

        self.config = config;
        self.dj_playlists = dj_playlists;

        Ok(())
    }

    pub fn update_config(&mut self, key: String, value: StateParameterTypes) -> Result<()> {
        self.config.set(&key, value);
        match self.config.save() {
            Ok(_) => Ok(()),
            Err(e) => Err(ah!(e)),
        }
    }

    pub fn add_node_to_track(
        &mut self,
        track_path: String,
        node_path: String,
    ) -> Result<&TrackDetails> {
        self.add_track_to_node(track_path, node_path)
    }

    pub fn add_track_to_node(
        &mut self,
        track_path: String,
        node_path: String,
    ) -> Result<&TrackDetails> {
        // get node
        let arcmut_lib_locked = Arc::new(Mutex::new(self.clone()));
        let arcmut_lib = arcmut_lib_locked.clone();
        let mut arcmut_lib = arcmut_lib.lock().unwrap();
        let node = arcmut_lib.get_node_by_path_mut(node_path.clone());
        if node.is_err() {
            return Err(ah!("Node not found"));
        }
        let node = node.unwrap();

        // get track
        let track = self.get_track(track_path.clone());
        if track.is_err() {
            return Err(ah!("Track not found"));
        }
        let track = track.unwrap();

        // add track to node
        node.add_track(track.to_track().clone());

        {
            let arcmut_lib_locked = Arc::new(Mutex::new(self.clone()));
            let arcmut_lib = arcmut_lib_locked.clone();
            let mut arcmut_lib = arcmut_lib.lock().unwrap();

            // add track to collection
            arcmut_lib.add_track(track_path.clone())?;
        }
        // save library
        self.save()?;
        return Ok(track);
    }

    pub fn add_node(&mut self, node_path: String) -> Result<()> {
        // find node parent

        let node_parent_path = node_path
            .split("/")
            .take(node_path.split("/").count() - 1)
            .collect::<Vec<&str>>()
            .join("/");

        let arcmut_lib_locked = Arc::new(Mutex::new(self.clone()));

        let arcmut_lib = arcmut_lib_locked.clone();
        let mut arcmut_lib = arcmut_lib.lock().unwrap();

        let node_parent = arcmut_lib.get_node_by_path_mut(node_parent_path.clone());

        if node_parent.is_err() {
            return Err(ah!("Node parent not found"));
        }

        let node_parent = node_parent.unwrap();
        let node_name = node_path.split("/").last().unwrap();
        let new_node = Node {
            name: node_name.to_string(),
            node: Vec::new(),
            track: Vec::new(),
            node_type: "1".to_string(),
            count: "".to_string(),
            path: node_path.clone(),
        };

        node_parent.add_node(new_node);
        self.save()?;
        Ok(())
    }
}

// Path: src-tauri\lib\recordbox_xml_parser\src\lib.rs
