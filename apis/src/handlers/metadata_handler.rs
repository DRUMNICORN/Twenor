use crate::models::{Metatrack, AppState, Metadata, MetatrackContructor, track_state::TrackState};

use super::{update_track_state, get_track_state};

pub fn get_metadata(user_id: i32, track_id: i32, state: &AppState) -> Result<String, Box<dyn std::error::Error>> {
    let metadata = Metadata::select_on_db(state, user_id, track_id)?;

    let metatrack = Metatrack {
        title: metadata.title,
        artist: metadata.artist,
        genre: metadata.genre,
        artstyle: metadata.artstyle,
        bpm: metadata.bpm.to_string(),
        offset: metadata.offset.to_string(),
        scale: metadata.scale,
        lyrics: metadata.lyrics,
    };

    let json = serde_json::to_string(&metatrack)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    Ok(json)
}


pub fn set_metadata(
    track_id: i32,
    user_id: i32,
    metatrack: &Metatrack,
    state: &AppState,
) -> Result<Option<String>, Box<dyn  std::error::Error>> {
    let conn = state.db_pool().get()?;
    let metadata = Metadata::select_on_db(state, user_id, track_id)?;

    let construct = MetatrackContructor {
        user_id,
        track_id,
        metadata_id: metadata.metadata_id,
        metatrack: metatrack.clone(),
    };

    if metadata.metadata_id == -1 {
        let conn = state.db_pool().get()?;
        let track_metadata = Metadata::from(construct);
        track_metadata.insert_on_db(conn, &track_metadata)?;
        Ok(Some("Metadata created".to_string()))
    } else {
        let conn = state.db_pool().get()?;
        let track_metadata = Metadata::from(construct);

        metadata.update_on_db(conn, &track_metadata)?;

        let current_state = get_track_state(track_id, state)?;
        let _ = update_track_state(track_id, user_id, state, TrackState::from_changed_metadata(metadata, track_metadata, current_state));
        
        Ok(Some("Metadata updated".to_string()))
    }
}