extern crate anyhow;
extern crate serde;

use self::anyhow::anyhow as err;
use self::anyhow::Result;

use self::serde::{Deserialize, Serialize};
use crate::track::{Track, TrackDetails};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Collection {
    #[serde(rename = "Entries")]
    pub entries: String,
    #[serde(rename = "TRACK", default)]
    pub track: Vec<TrackDetails>,
}

impl Collection {
    pub fn new() -> Self {
        Collection {
            entries: String::new(),
            track: Vec::new(),
        }
    }

    pub fn get_entries(&self) -> &str {
        &self.entries
    }

    pub fn get_track(&self) -> &Vec<TrackDetails> {
        &self.track
    }

    pub fn set_entries(&mut self, entries: &str) {
        self.entries = entries.to_string();
    }

    pub fn get_tracks_by_keys(&self, track_keys: &Vec<Track>) -> Vec<TrackDetails> {
        let mut tracks: Vec<TrackDetails> = Vec::new();
        for track_key in track_keys {
            let track = self
                .track
                .iter()
                .find(|t| t.get_key() == track_key.get_key())
                .unwrap();
            tracks.push(track.clone());
        }
        tracks
    }

    pub fn get_track_by_key(&self, track_key: &Track) -> Result<TrackDetails> {
        match self
            .track
            .iter()
            .find(|t| t.get_key() == track_key.get_key())
        {
            Some(track) => Ok(track.clone()),
            None => Err(err!("Track not found")),
        }
    }
}

// Path: src-tauri\lib\recordbox_xml_parser\src\collection.rs
