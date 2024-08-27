use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Track {
    #[serde(rename = "Key")]
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TrackDetails {
    #[serde(rename = "TrackID")]
    pub key: String,
    #[serde(rename = "Name", default)]
    pub name: String,
    #[serde(rename = "Artist", default)]
    pub artist: String,
    #[serde(rename = "Composer", default)]
    pub composer: String,
    #[serde(rename = "Album", default)]
    pub album: String,
    #[serde(rename = "Grouping", default)]
    pub grouping: String,
    #[serde(rename = "Genre", default)]
    pub genre: String,
    #[serde(rename = "Kind", default)]
    pub kind: String,
    #[serde(rename = "Size", default)]
    pub size: String,
    #[serde(rename = "TotalTime", default)]
    pub total_time: String,
    #[serde(rename = "DiscNumber", default)]
    pub disc_number: String,
    #[serde(rename = "TrackNumber", default)]
    pub track_number: String,
    #[serde(rename = "Year", default)]
    pub year: String,
    #[serde(rename = "AverageBpm", default)]
    pub average_bpm: String,
    #[serde(rename = "DateAdded", default)]
    pub date_added: String,
    #[serde(rename = "BitRate", default)]
    pub bit_rate: String,
    #[serde(rename = "SampleRate", default)]
    pub sample_rate: String,
    #[serde(rename = "Comments", default)]
    pub comments: String,
    #[serde(rename = "PlayCount", default)]
    pub play_count: String,
    #[serde(rename = "Rating", default)]
    pub rating: String,
    #[serde(rename = "Location", default)]
    pub location: String,
    #[serde(rename = "Remixer", default)]
    pub remixer: String,
    #[serde(rename = "Tonality", default)]
    pub tonality: String,
    #[serde(rename = "Label", default)]
    pub label: String,
    #[serde(rename = "Mix", default)]
    pub mix: String,
    #[serde(rename = "Colour", default)]
    pub colour: String,
    #[serde(rename = "TEMPO", default)]
    pub tempo: Vec<Tempo>,
    #[serde(rename = "POSITION_MARK", default)]
    pub position_mark: Vec<PositionMark>,
}

impl Default for TrackDetails {
    fn default() -> Self {
        Self {
            key: String::new(),
            name: String::new(),
            artist: String::new(),
            composer: String::new(),
            album: String::new(),
            grouping: String::new(),
            genre: String::new(),
            kind: String::new(),
            size: String::new(),
            total_time: String::new(),
            disc_number: String::new(),
            track_number: String::new(),
            year: String::new(),
            average_bpm: String::new(),
            date_added: String::new(),
            bit_rate: String::new(),
            sample_rate: String::new(),
            comments: String::new(),
            play_count: String::new(),
            rating: String::new(),
            location: String::new(),
            remixer: String::new(),
            tonality: String::new(),
            label: String::new(),
            mix: String::new(),
            colour: String::new(),
            tempo: Vec::new(),
            position_mark: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Tempo {
    #[serde(rename = "Inizio")]
    pub inizio: String,
    #[serde(rename = "Bpm")]
    pub bpm: String,
    #[serde(rename = "Metro")]
    pub metro: String,
    #[serde(rename = "Battito")]
    pub battito: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PositionMark {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Type")]
    pub mark_type: String,
    #[serde(rename = "Start")]
    pub start: String,
    #[serde(rename = "Num")]
    pub num: String,
    #[serde(rename = "Red")]
    pub red: String,
    #[serde(rename = "Green")]
    pub green: String,
    #[serde(rename = "Blue")]
    pub blue: String,
}

impl Track {
    pub fn new(key: String) -> Track {
        Track { key }
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_track_details(&self) -> TrackDetails {
        TrackDetails::new(self.key.clone())
    }

    pub fn set_track_details(&mut self, track_details: TrackDetails) {
        self.key = track_details.key;
    }
}

impl TrackDetails {
    pub fn new(key: String) -> TrackDetails {
        TrackDetails {
            key,
            name: String::new(),
            artist: String::new(),
            composer: String::new(),
            album: String::new(),
            grouping: String::new(),
            genre: String::new(),
            kind: String::new(),
            size: String::new(),
            total_time: String::new(),
            disc_number: String::new(),
            track_number: String::new(),
            year: String::new(),
            average_bpm: String::new(),
            date_added: String::new(),
            bit_rate: String::new(),
            sample_rate: String::new(),
            comments: String::new(),
            play_count: String::new(),
            rating: String::new(),
            location: String::new(),
            remixer: String::new(),
            tonality: String::new(),
            label: String::new(),
            mix: String::new(),
            colour: String::new(),
            tempo: Vec::new(),
            position_mark: Vec::new(),
        }
    }

    pub fn to_track(&self) -> Track {
        Track::new(self.key.clone())
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_artist(&self) -> &str {
        &self.artist
    }

    pub fn set_artist(&mut self, artist: String) {
        self.artist = artist;
    }
}

// Path: src-tauri\lib\recordbox_xml_parser\src\track.rs
