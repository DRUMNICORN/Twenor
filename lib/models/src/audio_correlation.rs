// models.rs

use r2d2_mysql::mysql::Params;
use serde::{Deserialize, Serialize};

use crate::DatabaseConfig;
use r2d2_mysql::mysql::prelude::Queryable;

// Define the SceneRequest type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioCorrelation {
   correlation_id: i32,
   audio_id: i32,
   correlation_values: Vec<f64>,
}

impl AudioCorrelation {
    pub fn new(
        correlation_id: i32,
        audio_id: i32,
        correlation_values: Vec<f64>,
    ) -> AudioCorrelation {
        let correlation = AudioCorrelation {
            correlation_id,
            audio_id,
            correlation_values,
        };
        correlation
    }

    pub fn get_id(&self) -> i32 {
        self.correlation_id
    }

    pub fn get_audio_id(&self) -> i32 {
        self.audio_id
    }

    pub fn get_correlation_values(&self) -> Vec<f64> {
        self.correlation_values.clone()
    }

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

    pub fn get_correlation(audio_id: i32, database_config: &DatabaseConfig) -> std::result::Result<Option<AudioCorrelation>, Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection()?;

        let query = format!("SELECT * FROM AUDIO_CORRELATION WHERE audio_id = {}", audio_id);
        let correlation = conn.query::<r2d2_mysql::mysql::Row, &str>(&query)?;
        let mut correlation_values: Vec<f64> = Vec::new();
        let mut correlation_id = 0;

        for row in correlation {
            correlation_id = row.get("correlation_id").ok_or_else(|| "Failed to get correlation_id")?;
            let values_string: String = row.get("correlation_values").ok_or_else(|| "Failed to get correlation_values")?;
            correlation_values = match serde_json::from_str(&values_string) {
                Ok(values) => values,
                Err(err) => {
                    log::error!("Failed to parse correlation values: {}", err);
                    return Ok(None);
                }
            }
        }


        Ok(Some(AudioCorrelation {
            correlation_id,
            audio_id,
            correlation_values,
        }))
    }

    pub fn clear(audio_id: i32, database_config: &DatabaseConfig) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection()?;

        let query = format!("DELETE FROM AUDIO_CORRELATION WHERE audio_id = {}", audio_id);
        conn.query::<r2d2_mysql::mysql::Row, &str>(&query)?;
        Ok(())
    }

    pub fn into_vec(&self) -> Vec<f64> {
        self.correlation_values.clone()
    }

    pub fn insert(&self, database_config: &DatabaseConfig) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection()?;
        let query = "INSERT INTO AUDIO_CORRELATION (audio_id, correlation_values) VALUES (?, ?)";
        let serialized_values = serde_json::to_string(&self.correlation_values)?;
        let values_string = serialized_values.as_str();
        log::info!("values_string: {}", values_string);
        let params: Params = Params::from((self.audio_id, values_string));
        conn.exec_first::<usize, &str, Params>(query, params)?;
        Ok(())
    }
}
