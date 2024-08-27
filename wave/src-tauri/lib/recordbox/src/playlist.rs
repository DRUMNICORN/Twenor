extern crate serde;

use self::serde::{Deserialize, Serialize};
use crate::node::Node;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Playlists {
    // multiple nodes
    #[serde(rename = "NODE", default)]
    pub node: Vec<Node>,
}

impl Playlists {
    pub fn new() -> Playlists {
        let mut playlists = Playlists { node: Vec::new() };

        playlists.add_node(Node::root());
        playlists
    }

    pub fn add_node(&mut self, node: Node) {
        self.node.push(node);
    }

    pub fn get_root_parent(&self) -> Node {
        // create a copy of all nodes within a now node
        let mut nodes = self.node.clone();

        // get the first node
        let mut node = nodes.remove(0);

        // loop through all nodes
        while !nodes.is_empty() {
            // get the first node
            let next_node = nodes.remove(0);

            // check if the node is a root node
            if next_node.is_root() {
                // if it is a root node, set the node to the next node
                node = next_node;
            } else {
                // if it is not a root node, add the next node to the node
                node.add_node(next_node);
            }
        }

        // return the node

        node
    }
}

// Path: src-tauri\lib\recordbox_xml_parser\src\playlist.rs
