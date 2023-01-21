use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use dj::DjPlaylists;
use dj::NodeWithTracks;
use node::Node;
use track::TrackDetails;

extern crate anyhow;
extern crate serde;
extern crate serde_xml_rs;
extern crate tauri;

use anyhow::anyhow as ah;
use anyhow::Result;

mod collection;
mod playlist;
mod product;

pub mod config;
pub mod dj;
pub mod node;
pub mod track;

#[derive(Debug, PartialEq, Clone)]
pub struct Library {
    pub config: config::Config,
    pub dj_playlists: dj::DjPlaylists,
}

impl Library {
    pub fn new(config: config::Config) -> Self {
        let mut library = Library {
            config: config,
            dj_playlists: DjPlaylists::new(),
        };

        let xml_path = library.config.get("xml_path");
        match xml_path {
            Some(xml_path) => match library.dj_playlists.load_from_file(xml_path) {
                Ok(_) => library,
                Err(e) => {
                    println!("Error loading dj_playlists from xml_path: {}", e);
                    library
                }
            },
            None => {
                // create new dj_playlists
                library.dj_playlists = DjPlaylists::new();

                let xml_path = library.config.generate_xml_path();
                match library.dj_playlists.save_to_file(&xml_path) {
                    Ok(_) => {
                        library.config.set("xml_path", &xml_path);
                        library.config.save();
                        library
                    }
                    Err(e) => {
                        println!("Error saving dj_playlists to xml_path: {}", e);
                        library
                    }
                }
            }
        }
    }

    pub fn get_xml_path(&self) -> String {
        match self.config.get("xml_path") {
            Some(xml_path) => xml_path,
            None => "library.xml",
        }
        .to_string()
    }

    pub fn get_node_by_path(&mut self, path: String) -> Result<&mut Node> {
        for node in self.dj_playlists.playlists.node.iter_mut() {
            let node = node.get_node_by_path(path.clone());
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
            match node
                .unwrap()
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
            match node
                .unwrap()
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

    pub fn get_config(&self) -> HashMap<String, String> {
        self.config.to_hashmap()
    }

    pub fn save(&self) -> Result<()> {
        println!("Saving library to file: {:?}", self.config.get("path"));
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
        let source_node = arcmut_lib.get_node_by_path(source_path.clone());
        if source_node.is_err() {
            return Err(ah!("Source node not found"));
        }
        let source_node = source_node.unwrap();

        let arcmut_lib = arcmut_lib_locked.clone();
        let mut arcmut_lib = arcmut_lib.lock().unwrap();
        let destination_node = arcmut_lib.get_node_by_path(destination_path.clone());
        if destination_node.is_err() {
            return Err(ah!("Destination node not found"));
        }
        let destination_node = destination_node.unwrap();

        let arcmut_lib = arcmut_lib_locked.clone();
        let mut arcmut_lib = arcmut_lib.lock().unwrap();
        let source_node_parent = arcmut_lib.get_node_by_path(source_parent_path.clone());
        if source_node_parent.is_err() {
            return Err(ah!("Source node parent not found"));
        }
        let source_node_parent = source_node_parent.unwrap();

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

    pub fn reload(&mut self) -> Result<()> {
        // should load from xml file
        // and replace the current library

        // let xml_string = std::fs::read_to_string(&self.xml_path)?;
        // let dj_playlists: DjPlaylists = serde_xml_rs::from_str(&xml_string)?;
        // self.dj_playlists = dj_playlists;
        Ok(())
    }

    pub fn update_config(&mut self, key: String, value: String) -> Result<()> {
        self.config.set(&key, &value);
        self.config.save();
        Ok(())
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
        let node = arcmut_lib.get_node_by_path(node_path.clone());
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

        let node_parent = arcmut_lib.get_node_by_path(node_parent_path.clone());

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
