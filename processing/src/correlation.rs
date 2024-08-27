use log::info;
use r2d2_mysql::mysql::{params, Params};
use serde::{Serialize, Deserialize};

use crate::{featutes::get_features, db::DatabaseConfig, correlation};


#[derive(Debug, Serialize, Deserialize)]
pub struct Correlation {
    correlation_id: i32,
    track_id: i32,
    correlation_values: Vec<f64>,
}

impl Correlation {
    pub fn map(&mut self) {
        // map correlation values to a range between 0 and 1
        let mut min = 0.0;
        let mut max = 0.0;

        for value in &self.correlation_values {
            if *value < min {
                min = *value;
            }
            if *value > max {
                max = *value;
            }
        }

        let range = max - min;

        for value in &mut self.correlation_values {
            *value = (*value - min) / range;
        }
    }

    pub fn get_corr_list(&self) -> Vec<f64> {
        self.correlation_values.clone()
    }
}

pub fn handle_correlation_state(track_id: i32, database_config: &DatabaseConfig) {
    // load all features from the database
    let features = get_features(track_id, database_config);

    // check freaures
    if features.len() == 0 {
        info!("No features found for track {}", track_id);
        return;
    }    


    // now calculate the correlation state
    let mut similar_audio_snippets: Vec<f64> = Vec::new();
    let mut chunk_ids: Vec<i32> = Vec::new();
    for audio_feature in &features {
        let feature_values: Vec<f64> = vec![
            audio_feature.danceability,
            audio_feature.valence,
            audio_feature.energy,
            audio_feature.tempo,
            audio_feature.loudness,
            audio_feature.speechiness,
            audio_feature.instrumentalness,
            audio_feature.liveness,
            audio_feature.acousticness,
            audio_feature.key,
            audio_feature.mode,
            audio_feature.duration,
            audio_feature.time_signature,
        ];

        let min = *feature_values.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let max = *feature_values.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

        // Normalize the feature values
        let normalized_feature_values: Vec<f64> = feature_values
            .iter()
            .map(|x| (x - min) / (max - min))
            .collect();

        // Compute similarity score
        let mut similarity_score: f64 = 0.0;
        if normalized_feature_values.len() != feature_values.len() {
            info!("Normalized feature values and feature values have different lengths");
            continue;
        }

        for (i, _val) in normalized_feature_values.iter().enumerate() {
            let feature_value = feature_values[i];
            let normalized_feature_value = normalized_feature_values[i];

            let diff = feature_value - normalized_feature_value;
            similarity_score += diff;
        }

        info!("Similarity score: {}", similarity_score);

        if similarity_score.is_nan() {
            info!("Similarity score is NaN");
            similarity_score = 0.0;
        }

        if similarity_score.is_infinite() {
            info!("Similarity score is infinite");
            similarity_score = 0.0;
        }

        // Append the similarity score to the list
        chunk_ids.push(audio_feature.chunk_id);
        similar_audio_snippets.push(similarity_score);
    }

    info!("Calculated correlation state for track_id: {}", track_id);
    info!("Similar audio snippets: {:?}", similar_audio_snippets);

    let mut correlation = Correlation {
        correlation_id: -1,
        track_id: track_id,
        correlation_values: similar_audio_snippets,
    };

    clear_correlation(track_id, database_config);
    correlation.map();
    save_correlation(correlation, database_config);
    info!("Saved correlation state for track_id: {}", track_id);
}

pub fn save_correlation(correlation: Correlation, database_config: &DatabaseConfig) {
    let mut conn = database_config.db_pool.get().unwrap();

    // Check if correlation for track_id already exists
    let existing_correlation = get_correlation(correlation.track_id, database_config);
    if existing_correlation.correlation_id != -1 {
        // Remove existing correlation for track_id
        let delete_query = format!("DELETE FROM TRACK_CORRELATION WHERE correlation_id = {}", existing_correlation.correlation_id);
        conn.query(delete_query).expect("Failed to delete existing correlation");
    }

    let query = "INSERT INTO TRACK_CORRELATION (track_id, correlation_values) VALUES (?, ?)";
    let serialized_values = serde_json::to_string(&correlation.correlation_values).unwrap();
    let values_string = serialized_values.as_str();
    info!("values_string: {}", values_string);
    let params: Params = Params::from((correlation.track_id, values_string));
    conn.prep_exec(query, params)
        .expect("Failed to execute query");
}

pub fn get_correlation(track_id: i32, database_config: &DatabaseConfig) -> Correlation {
    let mut conn = database_config.db_pool.get().unwrap();

    let query = format!("SELECT * FROM TRACK_CORRELATION WHERE track_id = {}", track_id);
    let correlation = conn.query(query).unwrap();

    let mut correlation_id = -1;
    let mut correlation_values: Vec<f64> = Vec::new();


    for row in correlation {
        let row = row.unwrap();
        correlation_id = row.get("correlation_id").unwrap();
        let correlation_values_string: String = row.get("correlation_values").unwrap();
        // use serde_json::from_str to convert string of values to Vec<f64>
        let values: Vec<f64> = serde_json::from_str(&correlation_values_string).unwrap();
        correlation_values = values;
    }

    Correlation {
        correlation_id,
        track_id,
        correlation_values,
    }
}

pub fn clear_correlation(track_id: i32, database_config: &DatabaseConfig) {
    let mut conn = database_config.db_pool.get().unwrap();

    let query = format!("DELETE FROM TRACK_CORRELATION WHERE track_id = {}", track_id);
    conn.query(query).unwrap();
}