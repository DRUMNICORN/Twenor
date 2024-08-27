// models.rs

use serde::{Deserialize, Serialize};

// Define the SceneRequest type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Correlation {
   pub correlation_id: i32,
   pub track_id: i32,
   pub correlation_values: Vec<f64>,
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
}