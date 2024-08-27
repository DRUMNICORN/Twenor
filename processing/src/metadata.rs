// src/metadata.rs

use r2d2_mysql::mysql::Params;
use serde::{Deserialize, Serialize};

use crate::db::DatabaseConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub metadata_id: i32,
    pub track_id: i32,
    pub user_id: i32,
    pub bpm: f32,
    pub offset: f32,
}

impl Metadata{
    // makes iter (key, value)
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, String)> {
        vec![
            ("metadata_id", self.metadata_id.to_string()),
            ("track_id", self.track_id.to_string()),
            ("user_id", self.user_id.to_string()),
            ("bpm", self.bpm.to_string()),
            ("offset", self.offset.to_string()),
        ]
        .into_iter()
    }
}

pub fn get_metadata(track_id: i32, user_id: i32, database_config: &DatabaseConfig) -> Metadata {
    let mut conn = database_config
    .db_pool
    .get()
    .expect("Failed to get database connection");

    let query = "SELECT * FROM TRACK_METADATA WHERE track_id = ? AND user_id = ?";
    let params: Params = Params::from((track_id, user_id));
    let result = conn.prep_exec(query, params).expect("Failed to execute query");

    let mut metadata: Metadata = Metadata {
        metadata_id: -1,
        track_id: -1,
        user_id: -1,
        bpm: 0.0,
        offset: 0.0,
    };
    
    for row in result {
        let row = row.unwrap();
        metadata = Metadata {
            metadata_id: row.get("metadata_id").unwrap(),
            track_id: row.get("track_id").unwrap(),
            user_id: row.get("user_id").unwrap(),
            bpm: row.get("bpm").unwrap(),
            offset: row.get("offset").unwrap(),
        };
    }

    metadata
}

pub fn get_metadata_by_track_id(track_id: i32, database_config: &DatabaseConfig) -> Metadata {
    let mut conn = database_config
    .db_pool
    .get()
    .expect("Failed to get database connection");

    let query = "SELECT * FROM TRACK_METADATA WHERE track_id = ?";
    let params: Params = (track_id,).into();
    let result = conn.prep_exec(query, params).expect("Failed to execute query");

    let mut metadata: Metadata = Metadata {
        metadata_id: -1,
        track_id: -1,
        user_id: -1,
        bpm: 0.0,
        offset: 0.0,
    };
    
    for row in result {
        let row = row.unwrap();
        metadata = Metadata {
            metadata_id: row.get("metadata_id").unwrap(),
            track_id: row.get("track_id").unwrap(),
            user_id: row.get("user_id").unwrap(),
            bpm: row.get("bpm").unwrap(),
            offset: row.get("offset").unwrap(),
        };
    }

    metadata
}

