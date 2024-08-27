
use r2d2_mysql::mysql::{Params, from_row};

use crate::models::{track_state::TrackState, AppState};


pub fn get_track_state(track_id: i32, state: &AppState) -> Result<TrackState, Box<dyn std::error::Error>> {
    let select_query = "SELECT state FROM TRACK_STATE WHERE track_id = ?";
    let select_params: Params = Params::from((track_id,));
    let mut conn = state.db_pool().get()?;
    let mut select_result = conn.prep_exec(select_query, select_params)?;

    let mut track_state = String::from("queued");
    if let Some(row) = select_result.next() {
        match row  {
            Ok(row) => {
                let state: String = from_row(row);
                 track_state = state;
            },
            Err(_) => track_state = TrackState::None.as_str().to_string(),
        };
    }

    let track_state = match track_state.as_str() {
        "queued" => TrackState::Queued,
        "loading" => TrackState::Loading,
        "features" => TrackState::Features,
        "corrolation" => TrackState::Corrolation,
        "writing" => TrackState::Writing,
        "rendering" => TrackState::Rendering,
        "done" => TrackState::Done,
        _ => TrackState::Queued,
    };

    Ok(track_state)
}

pub fn update_track_state(
    track_id: i32,
    user_id: i32,
    app_state: &AppState,
    track_state: TrackState,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    
    // Check if there is already a state written for the track
    let select_query = "SELECT state FROM TRACK_STATE WHERE track_id = ?";
    let cloned_track_id: String = track_id.to_string();
    let other_cloned_track_id = track_id.clone();
    let select_params: Params = Params::from((track_id,));
    let mut conn = app_state.db_pool().get()?;
    let mut select_result = conn.prep_exec(select_query, select_params)?;

    // If a row is returned, delete the existing state
    if let Some(_row) = select_result.next() {
        let mut conn = app_state.db_pool().get()?;
        let delete_query = "DELETE FROM TRACK_STATE WHERE track_id = ?";
        let delete_params: Params = Params::from((cloned_track_id,));
        conn.prep_exec(delete_query, delete_params)?;
    }
    let mut conn = app_state.db_pool().get()?;
    // Insert the new state
    let insert_query = "INSERT INTO TRACK_STATE (user_id, track_id, state) VALUES (?, ?, ?)";
    let insert_params: Params = Params::from((user_id, other_cloned_track_id, track_state.as_str()));
    conn.prep_exec(insert_query, insert_params)?;
    
    Ok("Track queued for rendering".to_string())
}
