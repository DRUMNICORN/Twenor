use serde::{Serialize, Deserialize};

use crate::AudioFeatures;


#[derive(Serialize, Deserialize, Debug)]
pub struct AudioFeatureList {
    audio_id: i32,
    features: Vec<AudioFeatures>,
}

impl AudioFeatureList {
    pub fn new(audio_id: i32, features: Vec<AudioFeatures>) -> AudioFeatureList {
        AudioFeatureList { audio_id, features }
    }

    pub fn insert(&mut self, feature: AudioFeatures) {
        self.features.push(feature);
    }

    pub fn to_vec(&self) -> Vec<AudioFeatures> {
        self.features.clone()
    }
}

