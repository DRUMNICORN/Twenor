// models.rs

use r2d2_mysql::mysql::prelude::{FromRow, Queryable};
use std::collections::HashMap;

use r2d2_mysql::mysql::{Params, Row};
use serde::{Deserialize, Serialize};

use crate::{AudioFeatureList, AudioFeaturesPython, DatabaseConfig};

// Define the SceneRequest type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioFeatures {
    feature_id: i32,
    chunk_id: i32,
    audio_id: i32,
    danceability: f64,
    valence: f64,
    energy: f64,
    tempo: f64,
    loudness: f64,
    speechiness: f64,
    instrumentalness: f64,
    liveness: f64,
    acousticness: f64,
    key: f64,
    mode: f64,
    duration: f64,
    time_signature: f64,
}

impl FromRow for AudioFeatures {
    fn from_row_opt(row: Row) -> std::result::Result<Self, r2d2_mysql::mysql::FromRowError> {
        let mut features = Self::default();
        features.copy_values_from_row(&row);
        Ok(features)
    }
}

impl AudioFeatures {
    pub fn new(feature_id: i32, audio_id: i32, chunk_id: i32,
        danceability: f64,
        valence: f64,
        energy: f64,
        tempo: f64,
        loudness: f64,
        speechiness: f64,
        instrumentalness: f64,
        liveness: f64,
        acousticness: f64,
        key: f64,
        mode: f64,
        duration: f64,
        time_signature: f64,
    ) -> Self {
        Self {
            feature_id,
            chunk_id,
            audio_id,
            danceability,
            valence,
            energy,
            tempo,
            loudness,
            speechiness,
            instrumentalness,
            liveness,
            acousticness,
            key,
            mode,
            duration,
            time_signature,
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
            "chunk_id" => self.chunk_id as f64,
            "audio_id" => self.audio_id as f64,
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
            _ => (),
        }
    }

    pub fn params(&self) -> Params {
        Params::Positional(vec![
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

    pub fn parmas_with_ids(&self) -> Params {
        Params::Positional(vec![
            self.chunk_id.into(),
            self.audio_id.into(),
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
            "feature_id",
            "chunk_id",
            "audio_id",
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



    fn default() -> Self {
        Self {
            feature_id: -1,
            chunk_id: -1,
            audio_id: -1,
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

    fn copy_values_from_row(&mut self, row: &Row) -> Option<()> {
        for name in Self::names() {
            let value: f64 = match row.get(name) {
                Some(value) => value,
                _ => return None,
            };
            self.set(name, value);
        }
        Some(())
    }

    pub fn normalize(features: Vec<AudioFeatures>) -> Vec<AudioFeatures> {
        let mut features = features;
        let mut max_val_hashmap: HashMap<&str, f64> = HashMap::new();
        let mut min_val_hashmap: HashMap<&str, f64> = HashMap::new();
        let keys = AudioFeaturesPython::names();
        for key in keys.iter() {
            max_val_hashmap.insert(key, 0.0);
            min_val_hashmap.insert(key, 100000.0);
        }

        for feature in features.iter_mut() {
            let feature = AudioFeaturesPython::from_features(feature);

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

        log::info!("Max values: {:?}", max_val_hashmap);
        log::info!("Min values: {:?}", min_val_hashmap);
        
        for feature in features.iter_mut() {
            let mut feature_abs = AudioFeaturesPython::from_features(feature);

            for key in keys.iter() {
                log::info!("{} = {}", key, feature.get(key));
                let value = feature.get(key);
                let max_val = max_val_hashmap.get(key).unwrap_or(&0.0);
                let min_val = min_val_hashmap.get(key).unwrap_or(&100000.0);
                let new_value = (value - min_val) / (max_val - min_val);
                if new_value.is_nan() {
                    feature_abs.set(key, 1.0);
                    continue;
                }
                // round by 10th decimal
                let new_value = (new_value * 1000.0).round() / 1000.0;
                feature_abs.set(key, new_value);
            }

            *feature = feature_abs.into_features(feature.feature_id, feature.audio_id, feature.chunk_id);
        }

        features
    }

    pub fn clear(
        audio_id: i32,
        database_config: &DatabaseConfig,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection()?;
        let query = "DELETE FROM AUDIO_FEATURES WHERE audio_id = ?";
        let params: Params = (audio_id,).into();
        conn.exec_drop(query, params)?;
        Ok(())
    }

    pub fn save(
        &self,
        database_config: &DatabaseConfig,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let features = self.clone();
        log::info!("[audio_features] Saving features: {:?}", features);

        let mut conn = database_config.get_connection()?;

        let keys;
        let questions;
        let params;

        if features.audio_id == -1 || features.chunk_id == -1 {
            return Err("Invalid audio_id or chunk_id".into());
        } else {
            keys = format!("`{}`", AudioFeatures::names().join("`, `"));
            questions = format!(
                "?{}",
                ", ?".repeat(AudioFeatures::names().len() - 1)
            );
            params = features.parmas_with_ids();
        }

        log::info!("[audio_features] Keys: {:?}", keys);
        log::info!("[audio_features] Questions: {:?}", questions);
        let query = format!(
            "INSERT INTO AUDIO_FEATURES ({}) VALUES ({})",
            keys, questions
        );
        log::info!("[audio_features] Query: {:?}", query);
        conn.exec_drop(query, params)?;
        Ok(())
    }

    pub fn by_audio_id(
        audio_id: i32,
        database_config: &DatabaseConfig,
    ) -> std::result::Result<AudioFeatureList, Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection()?;
        let query = "SELECT * FROM AUDIO_FEATURES WHERE audio_id = ?";
        let params: Params = (audio_id,).into();
        let features: Vec<AudioFeatures> = conn.exec(query, params)?;

        let features_list = AudioFeatureList::new(audio_id, features);
        Ok(features_list)
    }

    pub fn by_chunk_id(
        chunk_id: i32,
        audio_id: i32,
        database_config: &DatabaseConfig,
    ) -> std::result::Result<AudioFeatures, Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection()?;

        let query = "SELECT * FROM AUDIO_FEATURES WHERE chunk_id = ?";

        let params: Params = (chunk_id,).into();
        let row_option = conn.exec_first(query, params)?;

        if let Some(row) = row_option {
            Ok(AudioFeatures::from_row(row))
        } else {
            return Err("No features found".into());
        }
    }

    pub fn by_features_id(
        feature_id: i32,
        database_config: &DatabaseConfig,
    ) -> std::result::Result<AudioFeatures, Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection()?;

        let query = "SELECT * FROM AUDIO_FEATURES WHERE feature_id = ?";
        let params: Params = (feature_id,).into();
        let row_option = conn.exec_first(query, params)?;

        if let Some(row) = row_option {
            Ok(AudioFeatures::from_row(row))
        } else {
            return Err("No features found".into());
        }
    }

}
