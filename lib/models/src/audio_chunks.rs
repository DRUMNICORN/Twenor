use std::{fs::File, io::BufReader};

use claxon::FlacReader;
use hound::WavReader;
use r2d2_mysql::mysql::{self, Params};
use r2d2_mysql::mysql::{prelude::*, Row};
use rodio::{Decoder, Source};

use crate::{AudioChunk, AudioFormat, AudioInfo, AudioMetadata, DatabaseConfig, SAMPLE_RATE, AudioState};
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct AudioChunks {
    audio_path: String,
    audio_id: i32,
    user_id: i32,
    chunks: Vec<AudioChunk>,
}

impl FromRow for AudioChunks {
    fn from_row_opt(row: Row) -> std::result::Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        let chunk_id: i32 = match row.get("chunk_id") {
            Some(chunk_id) => chunk_id,
            None => return Err(mysql::FromRowError(row)),
        };
        let audio_id: i32 = match row.get("audio_id") {
            Some(audio_id) => audio_id,
            None => return Err(mysql::FromRowError(row)),
        };
        let chunk_start: f32 = match row.get("chunk_start") {
            Some(chunk_start) => chunk_start,
            None => return Err(mysql::FromRowError(row)),
        };
        let chunk_end: f32 = match row.get("chunk_end") {
            Some(chunk_end) => chunk_end,
            None => return Err(mysql::FromRowError(row)),
        };
        let chunk_index: i32 = match row.get("chunk_index") {
            Some(chunk_index) => chunk_index,
            None => return Err(mysql::FromRowError(row)),
        };
        let chunk_values: String = match row.get("chunk_values") {
            Some(chunk_values) => chunk_values,
            None => return Err(mysql::FromRowError(row)),
        };
        let chunk_values: Vec<i32> = match serde_json::from_str(&chunk_values) {
            Ok(chunk_values) => chunk_values,
            Err(_) => return Err(mysql::FromRowError(row)),
        };

        let audio_chunk = AudioChunk::new(chunk_id, audio_id, chunk_index, chunk_start, chunk_end, chunk_values);
        Ok(AudioChunks {
            audio_path: String::new(),
            audio_id: audio_id as i32,
            user_id: 0,
            chunks: vec![audio_chunk],
        })
    }
}

impl AudioChunks {
    pub fn new(
        audio_path: &str,
        audio_id: i32,
        user_id: i32,
        database_config: &DatabaseConfig,
    ) -> AudioChunks {
        let mut chunks = AudioChunks {
            audio_path: audio_path.to_string(),
            audio_id: audio_id,
            user_id: user_id,
            chunks: Vec::new(),
        };
        log::info!("Reading chunks from file...");
        match chunks.from_file(SAMPLE_RATE, database_config) {
            Ok(_) => {
                log::info!("Successfully read chunks from file");
            }
            Err(e) => {
                log::info!("Failed to read chunks from file: {}", e);
            }
        }
        chunks
    }

    pub fn len(&self) -> usize {
        self.chunks.len()
    }

    pub fn to_vec(&self) -> Vec<AudioChunk> {
        self.chunks.clone()
    }

    pub fn clear(
        audio_id: i32,
        database_config: &DatabaseConfig,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection()?;

        let query = "DELETE FROM AUDIO_CHUNK WHERE audio_id = ?";
        let params: Params = Params::from((audio_id,));
        conn.exec_drop(query, params)?;
        log::info!("Cleared chunks for audio {}", audio_id);
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.chunks.is_empty()
    }

    pub fn update(
        &mut self,
        audio_id: i32,
        user_id: i32,
        database_config: &DatabaseConfig,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        AudioChunks::clear(audio_id, database_config)?;
        self.audio_id = audio_id;
        self.user_id = user_id;
        self.insert(database_config)
    }

    pub fn insert(
        &self,
        database_config: &DatabaseConfig,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        AudioChunks::clear(self.audio_id, database_config)?;

        let mut conn = database_config.get_connection()?;

        for chunk in &self.chunks {
            let query = "INSERT INTO AUDIO_CHUNK (audio_id, user_id, chunk_start, chunk_end, chunk_index, chunk_values) VALUES (?, ?, ?, ?, ?, ?)";
            let values = chunk.get_chunk_values();
            let serialized_values = serde_json::to_string(&values)?;
            let values_string = serialized_values.as_str();
            if values.len() == 0 {
                continue;
            }
            if values[0] == 0 && values.len() == 1 {
                continue;
            }

            let (chunk_start, chunk_end, chunk_index) = chunk.get_start_end_index();

            let params: Params = Params::from((
                self.audio_id,
                self.user_id,
                chunk_start,
                chunk_end,
                chunk_index,
                values_string,
            ));
            conn.exec_drop(query, params)?;
        }
        Ok(())
    }

    pub fn from_path(
        audio_path: &str,
        bpm: f32,
        database_config: &DatabaseConfig
    ) -> std::result::Result<AudioChunks, Box<dyn std::error::Error>> {
        log::info!("Reading audio file from {}", audio_path);
        let [audio_id, user_id] = match AudioInfo::from_path(audio_path) {
            Ok((_, audio_id, user_id)) => [audio_id,user_id],
            Err(e) => {
                return Err(format!("Failed to get audio id: {}", e).into());
            }
        };
        let source = match AudioChunks::buffer_from_file(audio_path) {
            Ok(source) => source,
            Err(e) => {
                return Err(format!("Failed to get audio source: {}", e).into());
            }
        };

        let chunks = AudioChunks::from_source(source, bpm, audio_id, user_id, database_config);
        Ok(chunks)
    }

    fn buffer_from_file(
        audio_path: &str,
    ) -> std::result::Result<Decoder<BufReader<File>>, Box<dyn std::error::Error>> {
        let file = BufReader::new(File::open(audio_path)?);
        let source = Decoder::new(file)?;
        Ok(source)
    }

    pub fn from_source(source: Decoder<BufReader<File>>, bpm: f32, audio_id: i32, user_id: i32, database_config: &DatabaseConfig) -> AudioChunks {
        let source_vec: Vec<_> = source.into_iter().collect();
        let len: i32 = source_vec.len() as i32;
        let mut buffer: Vec<i16> = Vec::with_capacity(len as usize);

      

        let start = Instant::now();
        for sample in &source_vec {
            buffer.push(*sample);
        }
        
        
        // for (sample, i) in source_vec.iter().zip(0..len) {
        //     let new_progress = (i as f32 / len as f32) / 2.0;
        //     buffer.push(*sample);
        // }


        let duration = start.elapsed();
        log::info!("Reading audio data took {} seconds", duration.as_secs_f32());

        let duration = buffer.len() as f32 / SAMPLE_RATE as f32;
        let chunks = AudioChunks::from_buffer(bpm, duration, buffer, audio_id, &database_config);

        AudioChunks {
            audio_path: String::new(),
            audio_id: audio_id,
            user_id: user_id,
            chunks: chunks,
        }
    }

    fn from_buffer(bpm: f32, duration: f32, buffer: Vec<i16>, audio_id: i32, database_config: &DatabaseConfig) -> Vec<AudioChunk> {
        if buffer.len() == 0 {
            log::error!("Buffer is empty");
            return Vec::new();
        }

        if duration == 0.0 {
            log::error!("Duration is 0");
            return Vec::new();
        }

        if bpm == 0.0 {
            log::error!("BPM is 0");
            return Vec::new();
        }

        if audio_id == 0 || audio_id == -1 {
            log::error!("Audio id is 0 or -1");
            return Vec::new();
        }

        log::info!("Starting chunk calculation for audio {}...", audio_id);
        log::info!("SETTINGS: BPM: {}, DURATION: {}", bpm, duration);
        log::info!("Buffer length: {}", buffer.len());
        log::info!(
            "Buffer duration: {}",
            buffer.len() as f32 / SAMPLE_RATE as f32
        );

        let mut chunks: Vec<AudioChunk> = Vec::new();
        let chunk_duration = 60.0 / bpm * (4.0 * 8.0);
        let chunk_size = (chunk_duration * (SAMPLE_RATE as f32)) as usize;
        let chunk_count = (buffer.len() / chunk_size)+1;


        log::info!("Expected chunk count: {}", chunk_count);
        // TODO: look in db for chunks and check if tey have same amouth
        let audio_chunks_in_db = AudioChunks::by_audio_id(audio_id, &database_config).unwrap_or(Vec::new());
        log::info!("Found {} chunks in db", audio_chunks_in_db.len());

        if audio_chunks_in_db.len() == chunk_count {
            log::info!("Chunks in db match expected chunk count");
            return audio_chunks_in_db;
        }

        for (chunk_index, chunk_values) in buffer.chunks(chunk_size).enumerate() {
            let start = chunk_index as f32 * chunk_duration;
            let end = start + chunk_duration;
            let chunk_values = chunk_values.iter().map(|&v| v as i32).collect::<Vec<i32>>();
            // check if chunk is empty or has no values other than 0
            if chunk_values.iter().all(|&v| v == 0) {
                log::info!("Chunk {} is empty", chunk_index);
                continue;
            }
            let audio_chunk = AudioChunk::new(-1, audio_id, chunk_index as i32, start, end, chunk_values);
            chunks.push(audio_chunk);
        }
        log::info!("Calculated {} chunks", chunks.len());
        chunks
    }

    pub fn from_file(
        &mut self,
        target_sr: u32,
        database_config: &DatabaseConfig,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        log::info!("Reading audio chunks from file...");
        let metadata = AudioMetadata::by_audio_id(self.audio_id, database_config)?;
        let bpm = metadata.get_bpm();
        log::info!("BPM: {}", bpm);

        let audio_type = AudioFormat::from_path(&self.audio_path);
        log::info!("Audio type: {:?}", audio_type);
        
        self.read_audio_chunks(bpm, target_sr)?;
        if self.chunks.len() == 0 {
            return Err("Failed to read audio chunks".into());
        }
        log::info!("Found {} chunks", self.chunks.len());
        self.insert(database_config)?;

        log::info!("Successfully read audio chunks from file");
        Ok(())
    }

    fn load_audio_buffer(
        &self,
        audio_type: AudioFormat,
        target_sr: u32,
    ) -> std::result::Result<Vec<i32>, Box<dyn std::error::Error>> {
        let mut buffer: Vec<i32> = Vec::new();
        let mut duration = 0.0;

        let audio_path = &self.audio_path;

        match audio_type {
            AudioFormat::Wav => {
                let mut reader = WavReader::open(audio_path)?;
                let spec = reader.spec();
                let samples = reader.samples::<i32>();
                let mut sample_count = 0;

                for sample in samples {
                    buffer.push(sample?);
                    sample_count += 1;
                }
                duration = sample_count as f32 / spec.sample_rate as f32;
            }
            AudioFormat::Flac => {
                let mut reader = FlacReader::open(audio_path)?;
                let spec = reader.streaminfo();
                let samples = reader.samples();
                for sample in samples {
                    buffer.push(sample?);
                }

                duration = buffer.len() as f32 / spec.sample_rate as f32;
            }
            AudioFormat::Mp3 => {
                let file = BufReader::new(File::open(audio_path)?);
                let source = Decoder::new(file)?;
                let sr: u32 = source.sample_rate();
                duration = match source.total_duration() {
                    Some(duration) => duration.as_secs_f32(),
                    None => 0.0,
                };

                for sample in source {
                    let sample = sample as i32;
                    buffer.push(sample);
                }

                if duration == 0.0 {
                    duration = buffer.len() as f32 / sr as f32;
                }
            }
            AudioFormat::Other => {
                log::info!("File is not a wav, flac, or mp3 file");
            }
        }

        let resampled_buffer = AudioChunks::resampled_from_buffer(&buffer, target_sr, duration)?;
        Ok(resampled_buffer)
    }

    fn resampled_from_buffer(
        buffer: &[i32],
        target_sr: u32,
        duration: f32,
    ) -> std::result::Result<Vec<i32>, Box<dyn std::error::Error>> {
        let conversion_buffer: Vec<f32> = buffer.iter().map(|&x| x as f32).collect();
        let source_sr: u32 = (buffer.len() as f32 / duration) as u32;
        let conversion_type = samplerate::ConverterType::SincBestQuality;
        let resampled_buffer =
            samplerate::convert(source_sr, target_sr, 1, conversion_type, &conversion_buffer)?;
        Ok(resampled_buffer.iter().map(|&x| x as i32).collect())
    }

    pub fn generate(
        bpm: f32,
        duration: f32,
        buffer: Vec<i32>,
        audio_id: i32,
    ) -> std::result::Result<Vec<AudioChunk>, Box<dyn std::error::Error>> {
        if buffer.len() == 0 {
            log::error!("Buffer is empty");
            return Err("Buffer is empty".into());
        }

        if duration == 0.0 {
            log::error!("Duration is 0");
            return Err("Duration is 0".into());
        }

        if bpm == 0.0 {
            log::error!("BPM is 0");
            return Err("BPM is 0".into());
        }

        if audio_id == 0 || audio_id == -1 {
            log::error!("Audio id is 0 or -1");
            return Err("Audio id is 0 or -1".into());
        }

        log::info!("Starting chunk calculation for audio {}...", audio_id);
        log::info!("SETTINGS: BPM: {}, DURATION: {}", bpm, duration);
        log::info!("Buffer length: {}", buffer.len());
        log::info!(
            "Buffer duration: {}",
            buffer.len() as f32 / SAMPLE_RATE as f32
        );

        let mut chunks: Vec<AudioChunk> = Vec::new();
        let chunk_duration = 60.0 / bpm * (4.0 * 8.0);
        let chunk_size = (chunk_duration * (SAMPLE_RATE as f32)) as usize;
        // let chunk_count = buffer.len() / chunk_size;

        for (chunk_index, chunk_values) in buffer.chunks(chunk_size).enumerate() {
            let start = chunk_index as f32 * chunk_duration;
            let end = start + chunk_duration;
            let chunk_values = chunk_values.iter().map(|&v| v as i32).collect::<Vec<i32>>();
            // check if chunk is empty or has no values other than 0
            if chunk_values.iter().all(|&v| v == 0) {
                log::info!("Chunk {} is empty", chunk_index);
                continue;
            }
            let audio_chunk = AudioChunk::new(-1, audio_id, chunk_index as i32, start, end, chunk_values);
            chunks.push(audio_chunk);
        }
        log::info!("Calculated {} chunks", chunks.len());
        Ok(chunks)
    }

    pub fn read_audio_chunks(
        &mut self,
        bpm: f32,
        target_sr: u32,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if !std::path::Path::new(&self.audio_path).exists() {
            return Err("File does not exist".into());
        }

        let extension = self
            .audio_path
            .split('.')
            .last()
            .ok_or_else(|| "Failed to get extension")?;
        let extension = extension.to_lowercase();

        // get parent directory of audio_path
        let parent_dir = std::path::Path::new(&self.audio_path)
            .parent()
            .ok_or_else(|| "Failed to get parent directory")?;

        // get audio_id from parent directory
        let audio_id = parent_dir
            .file_name()
            .ok_or_else(|| "Failed to get file name")?
            .to_str()
            .ok_or_else(|| "Failed to convert file name to string")?
            .to_string();

        log::info!("Audio id: {}", audio_id);

        let audio_id: i32 = match audio_id.parse() {
            Ok(audio_id) => audio_id,
            Err(_) => {
                log::info!("Failed to parse audio id");
                return Err("Failed to parse audio id".into());
            }
        };

        let audio_type = AudioFormat::from_extension(&extension);
        let resampled_buffer = self.load_audio_buffer(audio_type, target_sr)?;

        let duration = resampled_buffer.len() as f32 / target_sr as f32;
        let chunks = AudioChunks::generate(bpm, duration, resampled_buffer, audio_id)?;
        self.chunks = chunks;

        Ok(())
    }

    pub fn by_audio_id(
        audio_id: i32,
        database_config: &DatabaseConfig,
    ) -> Result<Vec<AudioChunk>, Box<dyn std::error::Error>> {
        let mut conn = database_config.get_connection()?;
        let query = "SELECT * FROM AUDIO_CHUNK WHERE audio_id = ? ORDER BY chunk_index ASC";
        let params: Params = Params::from((audio_id,));
        let audio_chunks: Vec<AudioChunk> = conn
            .exec::<AudioChunks, _, _>(query, params)?
            .into_iter()
            .flat_map(|audio_chunks| audio_chunks.chunks)
            .collect();
        Ok(audio_chunks)
    }

    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }
}
