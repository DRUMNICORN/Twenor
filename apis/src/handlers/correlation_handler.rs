use r2d2_mysql::mysql::{params, Params};
use serde::{Serialize, Deserialize};

use crate::models::{AppState, Correlation};

pub fn get_correlation(track_id: i32, 
    state: rocket::State<'_, AppState>) -> Result<Correlation, Box<dyn std::error::Error>> 
{
    let mut conn = state.db_pool().get()?;
    
    let query = format!("SELECT * FROM TRACK_CORRELATION WHERE track_id = {}", track_id);

    let correlation = conn.query(query).unwrap();

    let mut correlation_id = -1;
    let mut correlation_values: Vec<f64> = Vec::new();

    for row in correlation {
        let row = row.unwrap();
        correlation_id = row.get("correlation_id").unwrap();
        let correlation_values_string: String = row.get("correlation_values").unwrap();
        let serde_values: Vec<f64> = serde_json::from_str(&correlation_values_string).unwrap();
        correlation_values = serde_values;
    }
    
    Ok(
    Correlation {
        correlation_id,
        track_id,
        correlation_values,
    })

}