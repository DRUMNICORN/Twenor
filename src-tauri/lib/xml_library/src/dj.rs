use std::fs::read_to_string;

use anyhow::{Result, Ok};
use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;

// import Product, Collection, Playlists
use crate::collection::Collection;
use crate::node::Node;
use crate::playlist::Playlists;
use crate::product::Product;
use crate::track::TrackDetails;

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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct DjPlaylists {
    #[serde(rename = "PRODUCT")]
    pub product: Product,
    #[serde(rename = "COLLECTION")]
    pub collection: Collection,
    #[serde(rename = "PLAYLISTS")]
    pub playlists: Playlists,
}

impl DjPlaylists {
    pub fn new() -> DjPlaylists {
        DjPlaylists {
            product: Product {
                name: String::from(""),
                version: String::from(""),
                company: String::from(""),
            },
            collection: Collection {
                entries: String::from(""),
                track: Vec::new(),
            },
            playlists: Playlists { node: Vec::new() },
        }
    }

    pub fn default() -> DjPlaylists {
        DjPlaylists {
            product: Product {
                name: String::from("rekordbox"),
                version: String::from("0.0.0"),
                company: String::from(""),
            },
            collection: Collection {
                entries: String::from("0"),
                track: Vec::new(),
            },
            playlists: Playlists { node: Vec::new() },
        }
    }

    pub fn save_to_file(&self, _path: &str) -> Result<()> {

      Ok(())
    }

    pub fn load_from_file(&mut self, path: &str) -> Result<()> {
        // read file

        let xml = read_to_string(path)?;
        let dj_playlists: DjPlaylists = from_str(&xml)?;
        
        self.product = dj_playlists.product;
        self.collection = dj_playlists.collection;
        self.playlists = dj_playlists.playlists;
        Ok(())
    }
}

// Path: src-tauri\lib\recordbox_xml_parser\src\dj_playlists.rs
