use std::collections::HashMap;
use std::error::Error;


// Function to get duration in minutes
pub fn get_duration(y: &[f32], sr: usize) -> f32 {
    let duration_in_seconds = y.len() as f32 / sr as f32;
    let duration_in_minutes = duration_in_seconds / 60.0;
    // Round duration to two decimal places
    (duration_in_minutes * 100.0).round() / 100.0
}

// Function to get BPM
pub fn get_bpm(y: &[f32], sr: usize) -> usize {
    let onset_env = get_onset_strength(y);
    let tempo = get_tempo(onset_env, sr);
    tempo.ceil() as usize
}

// Function to get key
pub fn get_key(_y: &[f32], _sr: usize) -> &'static str {
    "C Major"
}

// Function to get file size
pub fn get_file_size(file_path: &str) -> Result<f32, Box<dyn Error>> {
    let file_size = std::fs::metadata(file_path)?.len();
    Ok((file_size as f32) / (1024.0 * 1024.0))
}

// Function to extract chunk size
pub fn extract_chunk_size(bpm: usize, sr: usize, beats_per_bar: usize) -> usize {
    let seconds_per_minute = 60.0;
    let samples_per_beat = sr as f32 * seconds_per_minute / bpm as f32;
    let samples_per_bar = samples_per_beat * beats_per_bar as f32;
    let divisible_chunk_size = (samples_per_bar / 2.0).floor() as usize;
    divisible_chunk_size
}

// Function to gather track information
pub fn gather_track_information(y: &[f32], sr: usize, track_file_path: &str, topic: &str, beats_per_bar: usize, artstyle: &str) -> HashMap<String, String> {
    let mut metadata: HashMap<String, String> = HashMap::new();
    
    metadata.insert("topic".to_string(), topic.to_string());
    metadata.insert("key".to_string(), get_key(y, sr).to_string());
    if let Ok(size_in_mb) = get_file_size(track_file_path) {
        metadata.insert("size_in_mb".to_string(), format!("{:.2}", size_in_mb));
    }
    metadata.insert("bpm".to_string(), get_bpm(y, sr).to_string());
    metadata.insert("sample_rate_in_hz".to_string(), sr.to_string());
    metadata.insert("duration_in_minutes".to_string(), get_duration(y, sr).to_string());
    metadata.insert("chunk_size".to_string(), extract_chunk_size(get_bpm(y, sr), sr, beats_per_bar).to_string());
    metadata.insert("beats_per_bar".to_string(), beats_per_bar.to_string());
    metadata.insert("artstyle".to_string(), artstyle.to_string());

    metadata
}

// Helper function to calculate onset strength
fn get_onset_strength(y: &[f32]) -> Vec<f32> {
    let onset_strength = y.to_vec();
    onset_strength
}

// Helper function to calculate tempo
fn get_tempo(onset_env: Vec<f32>, sr: usize) -> f32 {
    let frame_size = 2048;
    let onset_env_length = onset_env.len();
    let _hop_size = onset_env_length / (onset_env_length / frame_size);

    // Calculate autocorrelation
    let mut autocorrelation = vec![0.0; onset_env_length];
    for lag in 0..onset_env_length {
        let mut sum = 0.0;
        for i in 0..onset_env_length - lag {
            sum += onset_env[i] * onset_env[i + lag];
        }
        autocorrelation[lag] = sum;
    }

    // Find the tempo from the first peak of the autocorrelation
    let mut tempo = 0.0;
    let mut max_value = 0.0;
    for i in (60 * sr / 240)..(240 * sr / 60) {
        if autocorrelation[i] > max_value {
            max_value = autocorrelation[i];
            tempo = sr as f32 / i as f32;
        }
    }

    tempo
}

// Function to split audio file into chunks with an offset
pub fn split_audio_file(y: &[f32], chunk_size: usize) -> Vec<Vec<f32>> {
    let mut audio_chunks = Vec::new();
    let num_chunks = y.len() / chunk_size;
    
    for i in 0..num_chunks {
        let chunk = &y[i * chunk_size..(i + 1) * chunk_size];
        audio_chunks.push(chunk.to_vec());
    }
    
    if y.len() % chunk_size != 0 {
        let last_chunk = &y[num_chunks * chunk_size..];
        let padding = vec![0.0; chunk_size - last_chunk.len()];
        let mut chunk = last_chunk.to_vec();
        chunk.extend(padding);
        audio_chunks.push(chunk);
    }
    
    audio_chunks
}
