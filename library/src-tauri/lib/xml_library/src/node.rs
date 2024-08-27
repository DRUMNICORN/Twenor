use crate::collection::Collection;
use crate::{dj::NodeWithTracks, track::Track};
use serde::{Deserialize, Serialize};

use anyhow::anyhow as err;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Node {
    #[serde(rename = "Type")]
    pub node_type: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Count", default)]
    pub count: String,
    #[serde(rename = "NODE", default)]
    pub node: Vec<Node>,
    #[serde(rename = "TRACK", default)]
    pub track: Vec<Track>,
    #[serde(rename = "PATH", default)]
    pub path: String,
}

impl Node {
    pub fn root() -> Node {
        Node {
            node_type: String::from("ROOT"),
            name: String::from("ROOT"),
            count: String::from("0"),
            node: Vec::new(),
            track: Vec::new(),
            path: String::from(""),
        }
    }

    pub fn fill_path(&mut self, path: String) {
        self.path = path.clone();
        for node in self.node.iter_mut() {
            node.fill_path(format!("{}/{}", path, node.name));
        }
    }

    pub fn get_node_type(&self) -> &str {
        &self.node_type
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_count(&self) -> &str {
        &self.count
    }

    pub fn get_node(&self) -> &Vec<Node> {
        &self.node
    }

    pub fn get_track(&self) -> &Vec<Track> {
        &self.track
    }

    pub fn set_node_type(&mut self, node_type: &str) {
        self.node_type = node_type.to_string();
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_count(&mut self, count: &str) {
        self.count = count.to_string();
    }

    pub fn set_node(&mut self, node: Vec<Node>) {
        self.node = node;
    }

    pub fn set_track(&mut self, track: Vec<Track>) {
        self.track = track;
    }

    pub fn add_node(&mut self, node: Node) {
        self.node.push(node);
    }

    pub fn add_track(&mut self, track: Track) {
        self.track.push(track);
    }

    pub fn get_node_with_tracks(
        &self,
        mut collection: Collection,
        path: String,
    ) -> Result<NodeWithTracks> {
        // this function will recall itself until it finds the playlist

        // split the path into a vector
        let path: Vec<&str> = path.split("/").collect();

        // get the first element of the path
        let first_element = path[0];

        // find the node with the name of the first element
        let node = self.node.iter().find(|node| node.name == first_element);

        // if the node is not found, return an error
        if node.is_none() {
            return Err(err!("Node not found"));
        }

        // if the path has only one element, return the node with tracks
        if path.len() == 1 {
            return Ok(NodeWithTracks::new(
                node.unwrap().clone(),
                collection.get_tracks_by_keys(node.unwrap().get_track()),
            ));
        }

        // if the path has more than one element, remove the first element and recall the function
        let mut path = path.to_vec();
        path.remove(0);
        let path = path.join("/");
        node.unwrap().get_node_with_tracks(collection, path)
    }

    pub fn get_node_by_path(&mut self, path: String) -> Result<&mut Node> {
        // this function will recall itself until it finds the playlist

        if path == "" {
            return Ok(self);
        }

        for node in self.node.iter_mut() {
            let inner_node = node.get_node_by_path(path.clone());
            if inner_node.is_ok() {
                return inner_node;
            }
        }

        Err(err!("Node not found"))
    }
}

// Path: src-tauri\lib\recordbox_xml_parser\src\node.rs
