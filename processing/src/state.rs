// src/state.rs

use r2d2_mysql::mysql::Params;
use serde::{Deserialize, Serialize};

use crate::{db::DatabaseConfig, chunk::clear_chunks_by_track_id, featutes::clear_features, scenes::clear_scene_arrangment_state};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrackState {
    Queued,

    Loading,
    
    Features,
    
    Corrolation,
    
    Arranging,

    Writing,

    Prompting,

    Rendering,
    
    Done,
}

impl TrackState {
    pub fn as_str(&self) -> &str {
        match self {
            TrackState::Queued => "queued",
            TrackState::Loading => "loading",
            TrackState::Features => "features",
            TrackState::Corrolation => "corrolation",
            TrackState::Arranging => "arranging",
            TrackState::Writing => "writing",
            TrackState::Prompting => "prompting",
            TrackState::Rendering => "rendering",
            TrackState::Done => "done",
        }
    }

    pub fn is_equal(&self, other: &TrackState) -> bool {
        self.as_str() == other.as_str()
    }

    pub fn get_next_state(&self) -> TrackState {
        match *self {
            TrackState::Queued => TrackState::Loading,
            TrackState::Loading => TrackState::Features,
            TrackState::Features => TrackState::Corrolation,
            TrackState::Corrolation => TrackState::Arranging,
            TrackState::Arranging => TrackState::Writing,
            TrackState::Writing => TrackState::Prompting,
            TrackState::Prompting => TrackState::Rendering,
            TrackState::Rendering => TrackState::Done,
            TrackState::Done => panic!("Invalid track state"),
        }
    }
}


pub fn handle_removing_state(track_id: i32, database_config: &DatabaseConfig) {
    clear_chunks_by_track_id(track_id, database_config);
    clear_features(track_id, database_config);
    clear_scene_arrangment_state(track_id, database_config);
    
}