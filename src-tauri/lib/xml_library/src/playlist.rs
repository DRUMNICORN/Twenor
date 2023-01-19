
use serde::{Deserialize, Serialize};
use crate::node::Node;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Playlists {
    // multiple nodes
    #[serde(rename = "NODE")]
    pub node: Vec<Node>,
}

impl Playlists {
    pub fn new() -> Playlists {
      let mut playlists = Playlists {
        node: Vec::new(),
      };

      playlists.add_node(Node::root());
      playlists
    }

    pub fn add_node(&mut self, node: Node) {
        self.node.push(node);
    }
}


// Path: src-tauri\lib\recordbox_xml_parser\src\playlist.rs