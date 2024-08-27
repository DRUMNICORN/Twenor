// src/chunk.rs

use log::{debug, error, info, warn};
use r2d2_mysql::mysql::{Params, from_value};

use crate::metadata::get_metadata;
use crate::track::Track;
use crate::SAMPLE_RATE;
use crate::{db::DatabaseConfig, DB_PATH};

use rodio::{Decoder, Source};
use std::fs::File;
use std::io::BufReader;

pub struct Chunk {
    pub chunk_id: i32,
    pub track_id: i32,

    pub chunk_index: i32,

    pub chunk_start: f32,
    pub chunk_end: f32,

    pub chunk_values: Vec<i32>,
}

pub fn handle_chunked_track(track: Track, database_config: &DatabaseConfig) {
    debug!("Handle chunked track");
    let track_id = track.track_id;
    let user_id = track.user_id;
    let track_dir_path = format!("{}/{}/{}", DB_PATH, user_id, track_id);
    debug!("Track dir path: {}", track_dir_path);
    for entry in std::fs::read_dir(track_dir_path).unwrap() {
        debug!("Entry: {:?}", entry);
        let entry = entry.unwrap();
        let path = entry.path();
        let path_str = path.to_str().unwrap();
        insert_chunk_values(track_id, user_id, path_str, database_config);
        debug!("Path: {}", path_str);
    }
}

fn clear_chunks(track_id: i32, user_id: i32, database_config: &DatabaseConfig) {
    let mut conn = database_config
        .db_pool
        .get()
        .expect("Failed to get database connection");

    let query = "DELETE FROM TRACK_CHUNK WHERE track_id = ? AND user_id = ?";
    let params: Params = Params::from((track_id, user_id));
    conn.prep_exec(query, params)
        .expect("Failed to execute query");
    info!("Cleared chunks for track {} and user {}", track_id, user_id);
}

pub fn clear_chunks_by_track_id(track_id: i32, database_config: &DatabaseConfig) {
    let mut conn = database_config
        .db_pool
        .get()
        .expect("Failed to get database connection");

    let query = "DELETE FROM TRACK_CHUNK WHERE track_id = ?";
    let params: Params = Params::from((track_id,));
    conn.prep_exec(query, params)
        .expect("Failed to execute query");
    info!("Cleared chunks for user {}", track_id);
}

fn write_chunks_to_database(
    chunks: Vec<Chunk>,
    track_id: i32,
    user_id: i32,
    database_config: &DatabaseConfig,
) {
    clear_chunks(track_id, user_id, database_config);

    let mut conn = database_config
        .db_pool
        .get()
        .expect("Failed to get database connection");

    for chunk in chunks {
        let query = "INSERT INTO TRACK_CHUNK (track_id, user_id, chunk_start, chunk_end, chunk_index, chunk_values) VALUES (?, ?, ?, ?, ?, ?)";
        let serilized_values = serde_json::to_string(&chunk.chunk_values).unwrap();
        let values_string = serilized_values.as_str();
        if chunk.chunk_values.len() == 0 {
            continue;
        }
        if chunk.chunk_values[0] == 0 && chunk.chunk_values.len() == 1 {
            continue;
        }
        let params: Params = Params::from((
            track_id,
            user_id,
            chunk.chunk_start,
            chunk.chunk_end,
            chunk.chunk_index,
            values_string,
        ));
        conn.prep_exec(query, params)
            .expect("Failed to execute query");
    }
}

fn insert_chunk_values(
    track_id: i32,
    user_id: i32,
    track_path: &str,
    database_config: &DatabaseConfig,
) {
    let metadata = get_metadata(track_id, user_id, database_config);
    let bpm = metadata.bpm;

    let chunks = read_audio_chunks(track_path, bpm, SAMPLE_RATE);
    if chunks.len() == 0 {
        warn!("No chunks found");
        return;
    }
    info!("Found {} chunks", chunks.len());
    write_chunks_to_database(chunks, track_id, user_id, database_config);
}

fn read_audio_chunks(track_path: &str, bpm: f32, target_sr: u32) -> Vec<Chunk> {
    info!("Reading audio file from {}", track_path);
    
    let (extension, track_id) = get_file_info(track_path);
    let source = read_file(track_path);
    info!("Audio file read");
    
    let (sr, channels) = validate_audio_properties(&source, target_sr);
    
    info!("Sample rate: {}", sr);
    info!("Channels: {}", channels);
    
    let chunks = process_audio_chunks(source, bpm, track_id);
    chunks
}

fn get_file_info(track_path: &str) -> (String, i32) {
    let extension = track_path.split('.').last().unwrap_or("").to_lowercase();
    let parent_dir = std::path::Path::new(track_path).parent().unwrap();
    let track_id = parent_dir.file_name().unwrap().to_str().unwrap_or("0").parse::<i32>().unwrap_or(0);
    
    match extension.as_str() {
        "mp3" => {}, // Add other acceptable extensions here
        _ => {
            error!("File is not a wav, flac, or mp3 file");
        }
    }
    
    (extension, track_id)
}

fn read_file(track_path: &str) -> Decoder<BufReader<File>> {
    let file = BufReader::new(File::open(track_path).unwrap());
    let source = Decoder::new(file).unwrap();
    source
}

fn validate_audio_properties(source: &Decoder<BufReader<File>>, target_sr: u32) -> (u32, u16) {
    let sr = source.sample_rate();
    let channels = source.channels();

    if sr != target_sr {
        warn!("Sample rate is not {} Hz", target_sr);
    }
    
    if channels != 1 {
        warn!("Audio is not mono");
    }
    
    (sr, channels)
}

fn process_audio_chunks(source: Decoder<BufReader<File>>, bpm: f32, track_id: i32) -> Vec<Chunk> {
    let mut buffer: Vec<i16> = Vec::new();

    // read audio data into buffer
    for sample in source {
        let sample = sample;
        buffer.push(sample);
    }

    let duration = buffer.len() as f32 / SAMPLE_RATE as f32;
    let chunks = generate_chunks(bpm, duration, buffer, track_id);
    chunks
}
/**
 * pub struct Chunk {
    pub chunk_id: i32,
    pub track_id: i32,

    pub chunk_index: i32,

    pub chunk_start: f32,
    pub chunk_end: f32,

    pub chunk_values: Vec<i32>,
}

 */

pub fn get_chunks(track_id: i32, database_config: &DatabaseConfig) -> Vec<Chunk> {
    let mut conn = database_config
        .db_pool
        .get()
        .expect("Failed to get database connection");

    let query = "SELECT * FROM TRACK_CHUNK WHERE track_id = ?";
    let params: Params = (track_id,).into();

    let mut chunks: Vec<Chunk> = Vec::new();

    for row in conn.prep_exec(query, params).expect("Failed to execute query") {
        if row.is_err() {
            println!("Error reading row");
            continue;
        }

        let row = row.unwrap();
        let chunk_id: i32 = from_value(row.get("chunk_id").unwrap());
        let track_id: i32 = from_value(row.get("track_id").unwrap());
        let chunk_index: i32 = from_value(row.get("chunk_index").unwrap());
        let chunk_start: f32 = from_value(row.get("chunk_start").unwrap());
        let chunk_end: f32 = from_value(row.get("chunk_end").unwrap());
        let chunk_values: String = from_value(row.get("chunk_values").unwrap());
        let chunk_values: Vec<i32> = serde_json::from_str(&chunk_values).unwrap();

        let chunk = Chunk {
            chunk_id,
            track_id,
            chunk_index,
            chunk_start,
            chunk_end,
            chunk_values,
        };

        chunks.push(chunk);
    }

    chunks
}

// Calculate the number of chunks based on bpm and duration
pub fn generate_chunks(bpm: f32, duration: f32, buffer: Vec<i16>, track_id: i32) -> Vec<Chunk> {
    if buffer.len() == 0 {
        error!("Buffer is empty");
        return Vec::new();
    }

    if duration == 0.0 {
        error!("Duration is 0");
        return Vec::new();
    }

    if bpm == 0.0 {
        error!("BPM is 0");
        return Vec::new();
    }

    if track_id == 0 || track_id == -1 {
        error!("Track id is 0 or -1");
        return Vec::new();
    }

    info!("Starting chunk calculation for track {}...", track_id);
    info!("SETTINGS: BPM: {}, DURATION: {}", bpm, duration);
    info!("Buffer length: {}", buffer.len());
    info!(
        "Buffer duration: {}",
        buffer.len() as f32 / SAMPLE_RATE as f32
    );

    let mut chunks: Vec<Chunk> = Vec::new();
    let chunk_duration = 60.0 / bpm * (4.0 * 8.0);
    let chunk_size = (chunk_duration * (SAMPLE_RATE as f32)) as usize;
    let chunk_count = buffer.len() / chunk_size;

    debug!("Expected chunk count: {}", chunk_count);
    for (chunk_index, chunk_values) in buffer.chunks(chunk_size).enumerate() {
        let start = chunk_index as f32 * chunk_duration;
        let end = start + chunk_duration;
        let chunk_values = chunk_values.iter().map(|&v| v as i32).collect::<Vec<i32>>();
        // check if chunk is empty or has no values other than 0
        if chunk_values.iter().all(|&v| v == 0) {
            warn!("Chunk {} is empty", chunk_index);
            continue;
        }
        chunks.push(Chunk {
            chunk_id: -1,
            track_id: track_id,
            chunk_start: start,
            chunk_end: end,

            chunk_index: chunk_index as i32,

            chunk_values: chunk_values,
        });
    }
    info!("Calculated {} chunks", chunks.len());
    chunks
}

pub fn load_chunks(track_id: i32, user_id: i32, database_config: &DatabaseConfig) -> Vec<Chunk> {
    let mut conn = database_config
        .db_pool
        .get()
        .expect("Failed to get database connection");

    let query =
        "SELECT * FROM TRACK_CHUNK WHERE track_id = ? AND user_id = ? ORDER BY chunk_index ASC";
    let params: Params = Params::from((track_id, user_id));
    let mut result = conn
        .prep_exec(query, params)
        .expect("Failed to execute query");

    let mut chunks: Vec<Chunk> = Vec::new();

    while let Some(row) = result.next() {
        let row: r2d2_mysql::mysql::Row = row.expect("Failed to get row");
        let chunk_id: i32 = row.get("chunk_id").expect("Failed to get chunk_id");
        let track_id: i32 = row.get("track_id").expect("Failed to get track_id");
        let chunk_start: f32 = row.get("chunk_start").expect("Failed to get chunk_start");
        let chunk_end: f32 = row.get("chunk_end").expect("Failed to get chunk_end");
        let chunk_index: i32 = row.get("chunk_index").expect("Failed to get chunk_index");
        let chunk_values: String = row.get("chunk_values").expect("Failed to get chunk_values");
        let chunk_values: Vec<i32> = serde_json::from_str(&chunk_values).unwrap();

        chunks.push(Chunk {
            chunk_id: chunk_id,
            track_id: track_id,
            chunk_start: chunk_start,
            chunk_end: chunk_end,
            chunk_index: chunk_index,
            chunk_values: chunk_values,
        });
    }

    chunks
}
