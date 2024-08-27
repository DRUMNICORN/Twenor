// models.rs

use r2d2_mysql::mysql::Row;
use serde::{Deserialize, Serialize};

/**
 * 
CREATE TABLE `TRACK_FEATURES` (
  `feature_id` int(11) DEFAULT NULL AUTO_INCREMENT,
  `track_id` int(11) DEFAULT NULL,
  `chunk_id` int(11) DEFAULT NULL,

  `danceability` float DEFAULT NULL,
  `valence` float DEFAULT NULL,
  `energy` float DEFAULT NULL,
  `tempo` float DEFAULT NULL,
  `loudness` float DEFAULT NULL,
  `speechiness` float DEFAULT NULL,
  `instrumentalness` float DEFAULT NULL,
  `liveness` float DEFAULT NULL,
  `acousticness` float DEFAULT NULL,
  `key` float DEFAULT NULL,
  `mode` float DEFAULT NULL,
  `duration` float DEFAULT NULL,
  `time_signature` float DEFAULT NULL,

  PRIMARY KEY (`feature_id`),
  KEY `track_id` (`track_id`),
  key `chunk_id` (`chunk_id`),

  CONSTRAINT `TRACK_FEATURES_ibfk_1` FOREIGN KEY (`track_id`) REFERENCES `TRACK_LIST` (`track_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `TRACK_FEATURES_ibfk_2` FOREIGN KEY (`chunk_id`) REFERENCES `TRACK_CHUNK` (`chunk_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;


 */

#[derive(Serialize, Deserialize, Debug)]
pub struct FeaturesPackage {
    track_id: i32,
    features: Vec<Features>,
}

impl FeaturesPackage {
    pub fn new(track_id: i32, features: Vec<Features>) -> FeaturesPackage {
        FeaturesPackage {
            track_id,
            features,
        }
    }

    pub fn insert(&mut self, feature: Features) {
        self.features.push(feature);
    }
    
}

// Define the SceneRequest type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Features {
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

impl Features {
   pub fn from_row(row: Row) -> Self {
       Self {
           danceability: row.get("danceability").unwrap(),
           valence: row.get("valence").unwrap(),
           energy: row.get("energy").unwrap(),
           tempo: row.get("tempo").unwrap(),
           loudness: row.get("loudness").unwrap(),
           speechiness: row.get("speechiness").unwrap(),
           instrumentalness: row.get("instrumentalness").unwrap(),
           liveness: row.get("liveness").unwrap(),
           acousticness: row.get("acousticness").unwrap(),
           key: row.get("key").unwrap(),
           mode: row.get("mode").unwrap(),
           duration: row.get("duration").unwrap(),
           time_signature: row.get("time_signature").unwrap(),
       }
   }
}