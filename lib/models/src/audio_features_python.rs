use serde::{Deserialize, Serialize};

use crate::{AudioFeatures, DatabaseConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioFeaturesPython {
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

impl AudioFeaturesPython {
    pub fn into_features(self, features_id: i32, audio_id: i32, chunk_id: i32) -> AudioFeatures {
        AudioFeatures::new(
            features_id,
            audio_id,
            chunk_id,
            self.danceability,
            self.valence,
            self.energy,
            self.tempo,
            self.loudness,
            self.speechiness,
            self.instrumentalness,
            self.liveness,
            self.acousticness,
            self.key,
            self.mode,
            self.duration,
            self.time_signature,
        )
    }

    pub fn from_features(features: &AudioFeatures) -> Self {
        Self {
            danceability: features.get("danceability"),
            valence: features.get("valence"),
            energy: features.get("energy"),
            tempo: features.get("tempo"),
            loudness: features.get("loudness"),
            speechiness: features.get("speechiness"),
            instrumentalness: features.get("instrumentalness"),
            liveness: features.get("liveness"),
            acousticness: features.get("acousticness"),
            key: features.get("key"),
            mode: features.get("mode"),
            duration: features.get("duration"),
            time_signature: features.get("time_signature"),
        }
    }

    pub fn default() -> Self {
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

    pub fn get(&self, name: &str) -> f64 {
        match name {
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
            _ => {
                log::info!("Feature {} not found", name);
                0.0
            }
        }
    }
    pub fn set(&mut self, name: &str, value: f64) {
        match name {
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
            _ => {
                log::debug!("Feature {} not found", name);
            }
        }
    }

    fn into_avg(features: Vec<AudioFeaturesPython>) -> AudioFeaturesPython {
        let len = features.len() as f64;
        let mut feature_final = AudioFeaturesPython::default();
        for feature in features {
            feature_final.acousticness += feature.acousticness;
            feature_final.danceability += feature.danceability;
            feature_final.energy += feature.energy;
            feature_final.instrumentalness += feature.instrumentalness;
            feature_final.liveness += feature.liveness;
            feature_final.loudness += feature.loudness;
            feature_final.speechiness += feature.speechiness;
            feature_final.tempo += feature.tempo;
            feature_final.valence += feature.valence;
            feature_final.key += feature.key;
            feature_final.mode += feature.mode;
            feature_final.time_signature += feature.time_signature;
            feature_final.duration += feature.duration;
        }

        feature_final.acousticness /= len as f64;
        feature_final.danceability /= len as f64;
        feature_final.energy /= len as f64;
        feature_final.instrumentalness /= len as f64;
        feature_final.liveness /= len as f64;
        feature_final.loudness /= len as f64;
        feature_final.speechiness /= len as f64;
        feature_final.tempo /= len as f64;
        feature_final.valence /= len as f64;
        feature_final.key /= len as f64;
        feature_final.mode /= len as f64;
        feature_final.time_signature /= len as f64;
        feature_final.duration /= len as f64;

        feature_final
    }
    
    pub fn get_avg(audio_id: i32, scene_chunks_ids: Vec<i32>, database_config: &DatabaseConfig) -> std::result::Result<AudioFeaturesPython, Box<dyn std::error::Error>> {
        let mut feature_list: Vec<AudioFeaturesPython> = Vec::new();
        for chunk_id in scene_chunks_ids {
            let features = AudioFeatures::by_chunk_id(chunk_id, audio_id, database_config)?;
            feature_list.push(AudioFeaturesPython::from_features(&features));
        }

        let feature_final = AudioFeaturesPython::into_avg(feature_list);

        Ok(feature_final)
    }

    pub fn into_list (&self) -> Vec<f64> {
        vec![
            self.danceability,
            self.valence,
            self.energy,
            self.tempo,
            self.loudness,
            self.speechiness,
            self.instrumentalness,
            self.liveness,
            self.acousticness,
            self.key,
            self.mode,
            self.duration,
            self.time_signature,
        ]
    }
}
