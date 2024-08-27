use r2d2_mysql::mysql::{params, Params};
use serde::{Serialize, Deserialize};

use crate::models::{AppState, Features, FeaturesPackage};

pub fn get_features(track_id: i32, 
    state: rocket::State<'_, AppState>) -> Result<FeaturesPackage, Box<dyn std::error::Error>> 
{
    let mut conn = state.db_pool().get()?;

    // we will select all features for a given track_id and put them into a FeaturesPackage

    let query = format!("SELECT * FROM TRACK_FEATURES WHERE track_id = {}", track_id);
    let features = conn.query(query).unwrap();
    let mut package = FeaturesPackage::new(track_id, Vec::new());

    // now loop all the rows and add them to the package
    for row in features {
        let row = row.unwrap();
        let f = Features::from_row(row);
        package.insert(f)
    }

    Ok(package)
}
