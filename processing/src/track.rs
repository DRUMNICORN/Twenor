use log::{info, warn, debug};
// src/track.rs
use r2d2_mysql::mysql::Params;

use crate::featutes::clear_features;
use crate::{db::DatabaseConfig, chunk::handle_chunked_track};
use crate::state::TrackState;

pub fn search_track_with_state(state: TrackState, database_config: &DatabaseConfig) -> Option<i32> {
    // Connect to the database using the connection pool
    let mut conn = database_config
        .db_pool
        .get()
        .expect("Failed to get database connection");

    // Execute the SELECT query to search for a track with the given state
    let query = "SELECT track_id FROM TRACK_STATE WHERE state = ?";
    let params: Params = Params::from((state.as_str(),));
    let mut result = conn
        .prep_exec(query, params)
        .expect("Failed to execute query");

    // Check if a row is returned
    if let Some(row) = result.next() {
        // Extract the track_id from the row
        let row = row.expect("Failed to get row");
        let track_id: i32 = row.get("track_id").expect("Failed to get track_id");
        info!("_______________________________");
        info!("Found track with state {}: {}", state.as_str(), track_id);
        
        return Some(track_id);
    }

    debug!("No track with state {}", state.as_str());

    None
}


pub fn update_track_state(track_id: i32, state: TrackState, database_config: &DatabaseConfig) {
    // Connect to the database using the connection pool
    let mut conn = database_config
        .db_pool
        .get()
        .expect("Failed to get database connection");


    let current_state = get_track_state(track_id, database_config);

    if current_state.is_equal(&TrackState::Done) {
        warn!("Track {} is done, and this will overwrite", track_id);
    }

    if current_state.is_equal(&state) {
        warn!("Track {} is already in state {}", track_id, state.as_str());
    }

    // Execute the UPDATE query to update the state of the track
    let query = "UPDATE TRACK_STATE SET state = ? WHERE track_id = ?";
    let state_str = state.as_str();
    let params: Params = Params::from((state_str, track_id));
    conn.prep_exec(query, params)
        .expect("Failed to execute query");
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub state_id: i32,
    pub track_id: i32,
    pub user_id: i32,
    pub state: TrackState,
}

pub fn handle_queued_state(track_id: i32, database_config: &DatabaseConfig) {
    info!("Handling queued state");
    
    // Connect to the database using the connection pool
    let mut conn = database_config
        .db_pool
        .get()
        .expect("Failed to get database connection");
    

    // Execute the SELECT query to get the track
    let query = "SELECT * FROM TRACK_STATE WHERE track_id = ?";
    let params: Params = Params::from((track_id,));
    let mut result = conn
        .prep_exec(query, params)
        .expect("Failed to execute query");

    info!("Got track");
    // Check if a row is returned
    if let Some(row) = result.next() {
        // Extract the track from the row
        let row = row.expect("Failed to get row");
        let track: Track = Track {
            state_id: row.get("state_id").expect("Failed to get state_id"),
            track_id: row.get("track_id").expect("Failed to get track_id"),
            user_id: row.get("user_id").expect("Failed to get user_id"),
            state: TrackState::Queued,
        };
        info!("Track: {:?}", track);
        handle_chunked_track(track, database_config);
        info!("Done handling queued state");
        // Set the track state to the preparation state
        clear_features(track_id, database_config);
        update_track_state(track_id, TrackState::Corrolation, database_config);
        info!("Track {} set to state {}", track_id, TrackState::Corrolation.as_str());
    } else {
        panic!("No track with the given track_id");
    }
}

pub fn get_track(track_id: i32, database_config: &DatabaseConfig) -> Track {
    // Connect to the database using the connection pool
    let mut conn = database_config
        .db_pool
        .get()
        .expect("Failed to get database connection");

    // Execute the SELECT query to get the track
    let query = "SELECT * FROM TRACK_STATE WHERE track_id = ?";
    let params: Params = Params::from((track_id,));
    let mut result = conn
        .prep_exec(query, params)
        .expect("Failed to execute query");

    // Check if a row is returned
    if let Some(row) = result.next() {
        // Extract the track from the row
        let row = row.expect("Failed to get row");
        let track: Track = Track {
            state_id: row.get("state_id").expect("Failed to get state_id"),
            track_id: row.get("track_id").expect("Failed to get track_id"),
            user_id: row.get("user_id").expect("Failed to get user_id"),
            state: TrackState::Queued,
        };
        return track;
    } else {
        panic!("No track with the given track_id");
    }
}

fn get_track_state(track_id: i32, database_config: &DatabaseConfig) -> TrackState {
    // Connect to the database using the connection pool
    let track = get_track(track_id, database_config);
    track.state
}