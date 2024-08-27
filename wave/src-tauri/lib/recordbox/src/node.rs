extern crate anyhow;
extern crate serde;
extern crate serde_json;

use crate::collection::Collection;
use crate::track::Track;
use crate::track::TrackDetails;

use self::serde::{Deserialize, Serialize};

use self::anyhow::anyhow as err;
use self::anyhow::Result;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NodeSimplified {
    #[serde(rename = "Type")]
    pub node_type: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "NODE", default)]
    pub node: Vec<NodeSimplified>,
    #[serde(rename = "PATH", default)]
    pub path: String,
}

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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NodeWithTracks {
    #[serde(rename = "Type")]
    pub node_type: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Count", default)]
    pub count: String,
    #[serde(rename = "NODE", default)]
    pub node: Vec<Node>,
    #[serde(rename = "TRACK", default)]
    pub track: Vec<TrackDetails>,
}

impl NodeWithTracks {
    pub fn new(node: Node, tracks: Vec<TrackDetails>) -> Self {
        NodeWithTracks {
            node_type: node.node_type,
            name: node.name,
            count: node.count,
            node: node.node,
            track: tracks,
        }
    }
}

impl Node {
    pub fn is_root(&self) -> bool {
        self.node_type == "ROOT" || self.name == "ROOT"
    }

    pub fn to_simplified(&self) -> NodeSimplified {
        NodeSimplified {
            node_type: self.node_type.clone(),
            name: self.name.clone(),
            node: self.node.iter().map(|node| node.to_simplified()).collect(),
            path: self.path.clone(),
        }
    }

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

    pub fn locate_node_path(&mut self, parent: &str) {
        if parent == "" {
            self.path = self.name.clone();
        } else {
            self.path = format!("{}/{}", parent, self.name);
        }
        for node in self.node.iter_mut() {
            node.locate_node_path(&self.path);
        }
    }

    pub fn locate_own_path(&mut self, root_node: &Node) -> Result<String> {
        // this function will recall itself until it finds the playlist

        // find the node with the name of the first element
        let node = root_node.node.iter().find(|node| node.name == self.name);

        // if the node is not found, return an error
        if node.is_none() {
            return Err(err!("Node not found"));
        }

        // if the path has only one element, return the node with tracks
        if self.node.len() == 0 {
            return Ok(format!(
                "{}/{}",
                match node {
                    Some(node) => node.path.clone(),
                    None => return Err(err!("Node not found")),
                },
                self.name
            ));
        }

        // if the path has more than one element, remove the first element and recall the function
        let mut node = node.unwrap().clone();
        let mut node_to_find = self.clone();
        node_to_find.node.remove(0);
        node.locate_own_path(&node_to_find)
    }

    pub fn get_tracks_from_collection(&self, collection: &Collection) -> Result<Vec<TrackDetails>> {
        let mut tracks = Vec::new();

        for track in self.track.iter() {
            match collection.get_track_by_key(&track) {
                Ok(track) => tracks.push(track),
                Err(err) => return Err(err),
            }
        }

        Ok((tracks))
    }

    pub fn get_tracks_from_collection_with_sub_tracks(
        &self,
        collection: &Collection,
    ) -> Result<Vec<TrackDetails>> {
        let mut tracks = Vec::new();

        for track in self.track.iter() {
            match collection.get_track_by_key(&track) {
                Ok(track) => tracks.push(track),
                Err(err) => return Err(err),
            }
        }

        for node in self.node.iter() {
            match node.get_tracks_from_collection_with_sub_tracks(collection) {
                Ok(node_tracks) => tracks.extend(node_tracks),
                Err(err) => return Err(err),
            }
        }

        Ok((tracks))
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
                match node {
                    Some(node) => node.clone(),
                    None => return Err(err!("Node not found")),
                },
                collection.get_tracks_by_keys(
                    (match node {
                        Some(node) => node.clone(),
                        None => return Err(err!("Node not found")),
                    })
                    .get_track(),
                ),
            ));
        }

        // if the path has more than one element, remove the first element and recall the function
        let mut path = path.to_vec();
        path.remove(0);
        let path = path.join("/");
        node.unwrap().get_node_with_tracks(collection, path)
    }

    pub fn get_node_by_path_mut(&mut self, path: String) -> Result<&mut Node> {
        // this function will recall itself until it finds the playlist

        if path == "" {
            return Ok(self);
        }

        for node in self.node.iter_mut() {
            let inner_node = node.get_node_by_path_mut(path.clone());
            if inner_node.is_ok() {
                return inner_node;
            }
        }

        Err(err!("Node not found"))
    }

    pub fn get_node_by_path(&self, path: String) -> Result<&Node> {
        println!("Checking node: {} with {}", self.path, path);

        // check if path is same as current node
        if path == self.path {
            println!("It's the same node");
            return Ok(self);
        }

        println!("It's not the same node");
        // check if path is a child of current node
        for node in self.node.iter() {
            println!(
                "Checking node: {} with {} - starts with: {}",
                node.path,
                path,
                node.path.starts_with(&path)
            );
            if path.starts_with(&node.path) {
                println!("It's a child node");
                return node.get_node_by_path(path);
            }
        }

        Err(err!("Node not found"))
    }

    // NodeWithTracks
    pub fn get_node_with_tracks_by_path(
        &self,
        collection: Collection,
        path: String,
    ) -> Result<NodeWithTracks> {
        // this function will recall itself until it finds the playlist

        if path == "" {
            return Ok(NodeWithTracks::new(
                self.clone(),
                collection.get_tracks_by_keys(self.get_track()),
            ));
        }

        for node in self.node.iter() {
            let inner_node = node.get_node_with_tracks_by_path(collection.clone(), path.clone());
            if inner_node.is_ok() {
                return inner_node;
            }
        }

        Err(err!("Node not found"))
    }
}

// Path: src-tauri\lib\recordbox_xml_parser\src\node.rs

// how to FromStr from Node

use std::str::FromStr;

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let node: Node = serde_xml_rs::from_str(s)?;
        Ok(node)
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        serde_xml_rs::to_string(self).unwrap()
    }
}
