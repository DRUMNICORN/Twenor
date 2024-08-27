// src/chunk.rs

use r2d2_mysql::mysql::{Params, prelude::Queryable};
use super::DatabaseConfig;

#[derive(Debug, Clone)]
pub struct AudioChunk {
    chunk_id: i32,
    audio_id: i32,

    chunk_index : i32,

    chunk_start: f32,
    chunk_end: f32,

    chunk_values: Vec<i32>,
}

impl AudioChunk {
    pub fn is_exsisting(chunk_id: i32, audio_id: i32, database_config: &DatabaseConfig) -> std::result::Result<bool, Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection()?;
        let query = "SELECT COUNT(*) FROM AUDIO_CHUNK WHERE chunk_id = ? AND audio_id = ?";
        let params: Params = Params::from((chunk_id, audio_id));
        let result = conn.exec_first(query, params)?;
        let count: i32 = result.map(|x: i32| x).ok_or_else(|| 0).unwrap();
        Ok(count > 0)
    }

    pub fn new(chunk_id: i32, audio_id: i32, chunk_index: i32, chunk_start: f32, chunk_end: f32, chunk_values: Vec<i32>) -> Self {
        AudioChunk {
            chunk_id,
            audio_id,
            chunk_index,
            chunk_start,
            chunk_end,
            chunk_values,
        }
    }

    pub fn get_start_end_index(&self) -> (f32, f32, i32) {
        (self.chunk_start, self.chunk_end, self.chunk_index)
    }

    pub fn get_chunk_values(&self) -> &Vec<i32> {
        &self.chunk_values
    }

    pub fn get_chunk_id(&self) -> i32 {
        self.chunk_id
    }

    pub fn get_chunk_index(&self) -> i32 {
        self.chunk_index
    }

}