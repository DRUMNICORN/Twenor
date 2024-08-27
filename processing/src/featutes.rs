use log::{error, info};
use r2d2_mysql::mysql::{from_row, from_value, Params, Row};
use serde::{Deserialize, Serialize};

use crate::state::TrackState;
use crate::track::update_track_state;
use crate::{chunk::load_chunks, db::DatabaseConfig, track::get_track};

pub fn handle_features_state(track_id: i32, database_config: &DatabaseConfig) {
    let track = get_track(track_id, database_config);
    let track_id = track.track_id;
    let user_id = track.user_id;

    // Load chunks from the database
    let chunks: Vec<crate::chunk::Chunk> = load_chunks(track_id, user_id, database_config);
    let mut generated_chunks = 0;
    let mut current_generared_chunks = 0;

    // Loop through each chunk and compute features
    for chunk in &chunks {
        if check_if_chunk_exists(chunk.chunk_id, track_id, database_config) {
            generated_chunks += 1;
            continue;
        }

        if current_generared_chunks >= 10 {
            info!(
                "Generated chunks: {}, Total chunks: {}/{}",
                current_generared_chunks,
                generated_chunks,
                chunks.len()
            );
            break;
        }

        let data: Vec<f64> = chunk.chunk_values.iter().map(|&x| x as f64).collect();
        let data: &[f64] = &data;

        let chunks_count = chunks.len() as f64;
        let chunk_index = chunk.chunk_index as f64;
        let chunk_percentage = chunk_index / chunks_count;
        let chunk_percentage = (chunk_percentage * 100.0).round();

        info!(
            "Chunk: {}/{} ({}%)",
            (chunk_index + 1.0), chunks_count, chunk_percentage
        );
        let chunk_features = compute_with_python(data);
        save_features(chunk.chunk_id, track_id, chunk_features, database_config);
        generated_chunks += 1;
        current_generared_chunks += 1;
    }

    // Now 'features' contains the computed feature values for each chunk
    log::info!(
        "Generated chunks: {}, Total chunks: {}/{}",
        current_generared_chunks,
        generated_chunks,
        chunks.len()
    );
    if generated_chunks == chunks.len() {
        // Update the track state to 'corrolation'
        log::info!("All chunks have been generated");

        // mapping features values from 0-1
        log::info!("Mapping features values from 0-1");
        let mut features = get_features(track_id, database_config);
        clear_features(track_id, database_config);
        features = mapping_features(features);
        for feature in features.iter() {
            let chunk_id = feature.chunk_id;
            let feature = feature.ref_into();
            save_features(chunk_id, track_id, feature, database_config);
        }
        log::info!("Features values mapped from 0-1");

        update_track_state(track_id, TrackState::Corrolation, database_config);
    } else {
        log::info!("Not all chunks have been generated, waiting for the rest");
        update_track_state(track_id, TrackState::Features, database_config);
    }
}

fn mapping_features(features: Vec<AudioFeaturesDB>) -> Vec<AudioFeaturesDB> {
    let mut features = features;
    let mut max_val_hashmap: HashMap<&str, f64> = HashMap::new();
    let mut min_val_hashmap: HashMap<&str, f64> = HashMap::new();
    let keys = AudioFeatures::names();
    for key in keys.iter() {
        max_val_hashmap.insert(key, 0.0);
        min_val_hashmap.insert(key, 100000.0);
    }

    for feature in features.iter() {
        for key in keys.iter() {
            let value = feature.get(key);
            if value > *(max_val_hashmap.get(key).unwrap_or(&0.0)) {
                max_val_hashmap.insert(key, value);
            }
            if value < *(min_val_hashmap.get(key).unwrap_or(&100000.0)) {
                min_val_hashmap.insert(key, value);
            }
        }
    }

    for feature in features.iter_mut() {
        for key in keys.iter() {
            let value = feature.get(key);
            let max_val = max_val_hashmap.get(key).unwrap_or(&0.0);
            let min_val = min_val_hashmap.get(key).unwrap_or(&100000.0);
            let new_value = (value - min_val) / (max_val - min_val);
            if new_value.is_nan() {
                println!("NAN: {} = 1", key);
                feature.set(key, 1.0);
                continue;
            }
            // round by 10th decimal
            let new_value = (new_value * 1000.0).round() / 1000.0;
            feature.set(key, new_value);
        }
    }

    features
}

fn check_if_chunk_exists(chunk_id: i32, track_id: i32, database_config: &DatabaseConfig) -> bool {
    let mut conn = database_config
        .db_pool
        .get()
        .expect("Failed to get database connection");

    let query = "SELECT * FROM TRACK_FEATURES WHERE chunk_id = ? AND track_id = ?";

    let params: Params = (chunk_id, track_id).into();
    let result = conn
        .prep_exec(query, params)
        .expect("Failed to execute query");

    let mut exists = false;
    for _row in result {
        exists = true;
    }

    exists
}

pub fn clear_features(track_id: i32, database_config: &DatabaseConfig) {
    let mut conn = database_config
        .db_pool
        .get()
        .expect("Failed to get database connection");

    let query = "DELETE FROM TRACK_FEATURES WHERE track_id = ?";

    let params: Params = (track_id,).into();
    conn.prep_exec(query, params)
        .expect("Failed to execute query");
}

fn save_features(
    chunk_id: i32,
    track_id: i32,
    features: AudioFeatures,
    database_config: &DatabaseConfig,
) {
    // if already exists, update
    let mut conn = database_config
        .db_pool
        .get()
        .expect("Failed to get database connection");

    // Execute the INSERT query to insert the features dynamically from struct
    let mut query = "INSERT INTO TRACK_FEATURES (chunk_id, track_id,".to_string();
    for (i, feature) in AudioFeatures::names().iter().enumerate() {
        let name = format!("`{}`", feature);
        query.push_str(name.as_str());
        if i < AudioFeatures::names().len() - 1 {
            query.push_str(", ");
        }
    }
    query.push_str(") VALUES (?, ?,");
    for (i, _) in AudioFeatures::names().iter().enumerate() {
        query.push_str("?");
        if i < AudioFeatures::names().len() - 1 {
            query.push_str(", ");
        }
    }
    query.push_str(")");


    info!("Query: {}", query);
    let params: Params = AudioFeaturesDB::new(features, chunk_id, track_id).params();
    conn.prep_exec(query, params)
        .expect("Failed to execute query");
}

pub fn get_features(track_id: i32, database_config: &DatabaseConfig) -> Vec<AudioFeaturesDB> {
    let mut conn = database_config
        .db_pool
        .get()
        .expect("Failed to get database connection");

    let query = "SELECT * FROM TRACK_FEATURES WHERE track_id = ?";

    let params: Params = (track_id,).into();
    let result = conn
        .prep_exec(query, params)
        .expect("Failed to execute query");

    let mut features: Vec<AudioFeaturesDB> = Vec::new();
    for row in result {
        let row = row.expect("Failed to get row");

        let feature = AudioFeaturesDB {
            feature_id: from_value(row.get("feature_id").unwrap()),
            chunk_id: from_value(row.get("chunk_id").unwrap()),
            track_id: from_value(row.get("track_id").unwrap()),
            danceability: from_value(row.get("danceability").unwrap()),
            valence: from_value(row.get("valence").unwrap()),
            energy: from_value(row.get("energy").unwrap()),
            tempo: from_value(row.get("tempo").unwrap()),
            loudness: from_value(row.get("loudness").unwrap()),
            speechiness: from_value(row.get("speechiness").unwrap()),
            instrumentalness: from_value(row.get("instrumentalness").unwrap()),
            liveness: from_value(row.get("liveness").unwrap()),
            acousticness: from_value(row.get("acousticness").unwrap()),
            key: from_value(row.get("key").unwrap()),
            mode: from_value(row.get("mode").unwrap()),
            duration: from_value(row.get("duration").unwrap()),
            time_signature: from_value(row.get("time_signature").unwrap()),
        };

        features.push(feature);
    }

    features
}

pub struct AudioFeaturesDB {
    pub feature_id: i32,
    pub chunk_id: i32,
    pub track_id: i32,
    pub danceability: f64,
    pub valence: f64,
    pub energy: f64,
    pub tempo: f64,
    pub loudness: f64,
    pub speechiness: f64,
    pub instrumentalness: f64,
    pub liveness: f64,
    pub acousticness: f64,
    pub key: f64,
    pub mode: f64,
    pub duration: f64,
    pub time_signature: f64,
}

impl AudioFeaturesDB {
    fn new(features: AudioFeatures, chunk_id: i32, track_id: i32) -> Self {
        Self {
            feature_id: -1,
            chunk_id,
            track_id,
            danceability: features.danceability,
            valence: features.valence,
            energy: features.energy,
            tempo: features.tempo,
            loudness: features.loudness,
            speechiness: features.speechiness,
            instrumentalness: features.instrumentalness,
            liveness: features.liveness,
            acousticness: features.acousticness,
            key: features.key,
            mode: features.mode,
            duration: features.duration,
            time_signature: features.time_signature,
        }
    }

    pub fn get(&self, feature: &str) -> f64 {
        match feature {
            "danceability" => self.danceability,
            "valence" => self.valence,
            "energy" => self.energy,
            "tempo" => self.tempo,
            "loudness" => self.loudness,
            "speechiness" => self.speechiness,
            "instrumentalness" => self.instrumentalness,
            "liveness" => self.liveness,
            "acousticness" => self.acousticness,
            "key" => self.key,
            "mode" => self.mode,
            "duration" => self.duration,
            "time_signature" => self.time_signature,
            _ => panic!("Invalid feature"),
        }
    }

    pub fn set(&mut self, feature: &str, value: f64) {
        match feature {
            "danceability" => self.danceability = value,
            "valence" => self.valence = value,
            "energy" => self.energy = value,
            "tempo" => self.tempo = value,
            "loudness" => self.loudness = value,
            "speechiness" => self.speechiness = value,
            "instrumentalness" => self.instrumentalness = value,
            "liveness" => self.liveness = value,
            "acousticness" => self.acousticness = value,
            "key" => self.key = value,
            "mode" => self.mode = value,
            "duration" => self.duration = value,
            "time_signature" => self.time_signature = value,
            _ => panic!("Invalid feature"),
        }
    }

    pub fn default() -> Self {
        Self {
            feature_id: -1,
            chunk_id: -1,
            track_id: -1,
            danceability: 0.0,
            valence: 0.0,
            energy: 0.0,
            tempo: 0.0,
            loudness: 0.0,
            speechiness: 0.0,
            instrumentalness: 0.0,
            liveness: 0.0,
            acousticness: 0.0,
            key: 0.0,
            mode: 0.0,
            duration: 0.0,
            time_signature: 0.0,
        }
    }

    pub fn params(&self) -> Params {
        println!("Track Signature: {:?}", self.time_signature);
        Params::Positional(vec![
            self.chunk_id.into(),
            self.track_id.into(),
            self.danceability.into(),
            self.valence.into(),
            self.energy.into(),
            self.tempo.into(),
            self.loudness.into(),
            self.speechiness.into(),
            self.instrumentalness.into(),
            self.liveness.into(),
            self.acousticness.into(),
            self.key.into(),
            self.mode.into(),
            self.duration.into(),
            self.time_signature.into(),
        ])
    }

    pub fn names() -> Vec<&'static str> {
        vec![
            "danceability",
            "valence",
            "energy",
            "tempo",
            "loudness",
            "speechiness",
            "instrumentalness",
            "liveness",
            "acousticness",
            "key",
            "mode",
            "duration",
            "time_signature",
        ]
    }

    pub fn into(self) -> AudioFeatures {
        AudioFeatures {
            danceability: self.danceability,
            valence: self.valence,
            energy: self.energy,
            tempo: self.tempo,
            loudness: self.loudness,
            speechiness: self.speechiness,
            instrumentalness: self.instrumentalness,
            liveness: self.liveness,
            acousticness: self.acousticness,
            key: self.key,
            mode: self.mode,
            duration: self.duration,
            time_signature: self.time_signature,
        }
    }

    pub fn ref_into(&self) -> AudioFeatures {
        AudioFeatures {
            danceability: self.danceability,
            valence: self.valence,
            energy: self.energy,
            tempo: self.tempo,
            loudness: self.loudness,
            speechiness: self.speechiness,
            instrumentalness: self.instrumentalness,
            liveness: self.liveness,
            acousticness: self.acousticness,
            key: self.key,
            mode: self.mode,
            duration: self.duration,
            time_signature: self.time_signature,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AudioFeatures {
    pub danceability: f64,
    pub valence: f64,
    pub energy: f64,
    pub tempo: f64,
    pub loudness: f64,
    pub speechiness: f64,
    pub instrumentalness: f64,
    pub liveness: f64,
    pub acousticness: f64,
    pub key: f64,
    pub mode: f64,
    pub duration: f64,
    pub time_signature: f64,
}

impl Default for AudioFeatures {
    fn default() -> Self {
        Self {
            danceability: 0.0,
            valence: 0.0,
            energy: 0.0,
            tempo: 0.0,
            loudness: 0.0,
            speechiness: 0.0,
            instrumentalness: 0.0,
            liveness: 0.0,
            acousticness: 0.0,
            key: 0.0,
            mode: 0.0,
            duration: 0.0,
            time_signature: 0.0,
        }
    }
}

impl AudioFeatures {
    pub fn get(&self, feature: &str) -> f64 {
        match feature {
            "danceability" => self.danceability,
            "valence" => self.valence,
            "energy" => self.energy,
            "tempo" => self.tempo,
            "loudness" => self.loudness,
            "speechiness" => self.speechiness,
            "instrumentalness" => self.instrumentalness,
            "liveness" => self.liveness,
            "acousticness" => self.acousticness,
            "key" => self.key,
            "mode" => self.mode,
            "duration" => self.duration,
            "time_signature" => self.time_signature,
            _ => panic!("Invalid feature"),
        }
    }

    pub fn names() -> Vec<&'static str> {
        vec![
            "danceability",
            "valence",
            "energy",
            "tempo",
            "loudness",
            "speechiness",
            "instrumentalness",
            "liveness",
            "acousticness",
            "key",
            "mode",
            "duration",
            "time_signature",
        ]
    }
}

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str;

pub fn get_features_by_chunk_id(chunk_id: i32, database_config: &DatabaseConfig) -> AudioFeaturesDB {
    let mut conn = database_config
        .db_pool
        .get()
        .expect("Failed to get database connection");

    let query = "SELECT * FROM TRACK_FEATURES WHERE chunk_id = ?";

    let params: Params = (chunk_id,).into();
    let result = conn
        .prep_exec(query, params)
        .expect("Failed to execute query");

    let mut feature = AudioFeaturesDB::default();
    for row in result {
        let row = row.expect("Failed to get row");

        feature = AudioFeaturesDB {
            feature_id: from_value(row.get("feature_id").unwrap()),
            chunk_id: from_value(row.get("chunk_id").unwrap()),
            track_id: from_value(row.get("track_id").unwrap()),
            danceability: from_value(row.get("danceability").unwrap()),
            valence: from_value(row.get("valence").unwrap()),
            energy: from_value(row.get("energy").unwrap()),
            tempo: from_value(row.get("tempo").unwrap()),
            loudness: from_value(row.get("loudness").unwrap()),
            speechiness: from_value(row.get("speechiness").unwrap()),
            instrumentalness: from_value(row.get("instrumentalness").unwrap()),
            liveness: from_value(row.get("liveness").unwrap()),
            acousticness: from_value(row.get("acousticness").unwrap()),
            key: from_value(row.get("key").unwrap()),
            mode: from_value(row.get("mode").unwrap()),
            duration: from_value(row.get("duration").unwrap()),
            time_signature: from_value(row.get("time_signature").unwrap()),
        };
    }

    feature
}

fn compute_with_python(data: &[f64]) -> AudioFeatures {
    info!("Computing features with Python script");
    // check if data is empty
    if data.is_empty() {
        info!("Data is empty, returning default features");
        return AudioFeatures::default();
    }

    // get length of data
    let len = data.len();
    info!("Data length: {}", len);

    // Convert the data to a string
    let data_str = data
        .iter()
        .map(|f| f.to_string())
        .collect::<Vec<String>>()
        .join(",");

    // Get the path to the Python script
    let python_script_path = "/home/roggen/Documents/GitHub/processing/py/compute.py";
    let python_script_path = PathBuf::from(python_script_path);

    // Create the buffer file path
    let buffer_file_path = python_script_path
        .parent()
        .unwrap()
        .join("buffers")
        .join("id.wav");

    // Create the buffers directory if it doesn't exist
    std::fs::create_dir_all(buffer_file_path.parent().unwrap())
        .expect("Failed to create buffers directory");

    // Save the buffer to the file
    std::fs::write(&buffer_file_path, data_str).expect("Failed to write buffer to file");
    info!("Buffer saved to file: {:?}", buffer_file_path);

    // Call the Python script
    let child = Command::new("python")
        .arg(python_script_path)
        .arg(buffer_file_path.to_str().unwrap()) // Pass the buffer file path as an argument to the Python script
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute Python script");

    let output = child.wait_with_output().unwrap();

    // Check if the Python script executed successfully
    if output.status.success() {
        let features: AudioFeatures = serde_json::from_slice(&output.stdout)
            .expect("Failed to parse JSON output");
        info!("Features computed with Python script: {:?}", features);
        features
    } else {
        error!(
            "Python script execution failed: {}",
            str::from_utf8(&output.stderr).unwrap()
        );
        AudioFeatures::default()
    }
}


pub fn get_features_by_id(feature_id: i32, database_config: &DatabaseConfig) -> AudioFeaturesDB {
    let mut conn = database_config
        .db_pool
        .get()
        .expect("Failed to get database connection");

    let query = "SELECT * FROM TRACK_FEATURES WHERE feature_id = ?";

    let params: Params = (feature_id,).into();
    let result = conn
        .prep_exec(query, params)
        .expect("Failed to execute query");

    let mut feature = AudioFeaturesDB::default();
    for row in result {
        let row = row.expect("Failed to get row");

        feature = AudioFeaturesDB {
            feature_id: from_value(row.get("feature_id").unwrap()),
            chunk_id: from_value(row.get("chunk_id").unwrap()),
            track_id: from_value(row.get("track_id").unwrap()),
            danceability: from_value(row.get("danceability").unwrap()),
            valence: from_value(row.get("valence").unwrap()),
            energy: from_value(row.get("energy").unwrap()),
            tempo: from_value(row.get("tempo").unwrap()),
            loudness: from_value(row.get("loudness").unwrap()),
            speechiness: from_value(row.get("speechiness").unwrap()),
            instrumentalness: from_value(row.get("instrumentalness").unwrap()),
            liveness: from_value(row.get("liveness").unwrap()),
            acousticness: from_value(row.get("acousticness").unwrap()),
            key: from_value(row.get("key").unwrap()),
            mode: from_value(row.get("mode").unwrap()),
            duration: from_value(row.get("duration").unwrap()),
            time_signature: from_value(row.get("time_signature").unwrap()),
        };
    }

    feature
}