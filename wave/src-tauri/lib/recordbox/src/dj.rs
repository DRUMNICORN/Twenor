extern crate anyhow;
extern crate serde;
extern crate serde_xml_rs;

use std::fs::read_to_string;

use self::anyhow::Result;
use self::serde::{Deserialize, Serialize};
use self::serde_xml_rs::from_str;

// import Product, Collection, Playlists
use crate::collection::Collection;
use crate::node::Node;
use crate::playlist::Playlists;
use crate::product::Product;
use crate::track::TrackDetails;

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
        // load default
        DjPlaylists::default()
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

    pub fn to_file(&self, path: &str) -> Result<()> {
        // if file DON'T exists create it
        if !std::path::Path::new(path).exists() {
            std::fs::File::create(path)?;

            // write to file
            let xml = self.to_string()?;
            std::fs::write(path, xml)?;
        } else {
            println!("File already exists");
        }

        Ok(())
    }

    pub fn from_file(path: &str) -> Result<DjPlaylists> {
        // read file

        if !path.ends_with(".xml") {
            return Err(anyhow::anyhow!("File is not xml"));
        }

        // path exists
        if !std::path::Path::new(path).exists() {
            // return Err(anyhow::anyhow!("File does not exist"));
            let dj_playlists = DjPlaylists::default();
            // save to file
            dj_playlists.to_file(path)?;
            return Ok(dj_playlists);
        }

        let xml = read_to_string(path)?;
        let mut dj_playlists: DjPlaylists = from_str(&xml)?;

        // locate all paths
        dj_playlists.locate_each_node_path();

        Ok(dj_playlists)
    }

    pub fn to_string(&self) -> Result<String> {
        let xml = serde_xml_rs::to_string(self)?;
        Ok(xml)
    }

    pub fn locate_each_node_path(&mut self) {
        // locate each node path
        for node in &mut self.playlists.node {
            node.locate_node_path("");
        }
    }
}

// Path: src-tauri\lib\recordbox_xml_parser\src\dj_playlists.rs
