use serde::Serialize;
use std::fs::{File, read_dir};
use std::path::Path;
use hound::WavReader;
use minimp3::Decoder;

use crate::{DatabaseConfig, AudioFile, Audio};

#[derive(Serialize)]
pub struct AudioBuffer {
    buffer: Vec<u16>,
}

impl AudioBuffer {
    fn float_vec_mult_float(vec: Vec<f32>, multiplier: f32) -> Vec<u16> {
        let mut new_vec = Vec::new();
        for i in vec {
            let new_val = i * multiplier;
            new_vec.push(new_val as u16);
        }
        new_vec
    }

    fn load_wav(file_path: &str) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        let mut reader = WavReader::open(file_path)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

        let spec = reader.spec();
        let samples: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Float => reader
                .samples::<f32>()
                .map(|s| s.unwrap())
                .collect::<Vec<f32>>(),
            hound::SampleFormat::Int => reader
                .samples::<i16>()
                .map(|s| s.unwrap() as f32 / std::i16::MAX as f32)
                .collect::<Vec<f32>>(),
        };

        let buffer: Vec<u16> = match spec.sample_format {
            hound::SampleFormat::Float => Self::float_vec_mult_float(samples, std::u16::MAX as f32),
            hound::SampleFormat::Int => samples
                .iter()
                .map(|s| (*s * std::u16::MAX as f32) as u16)
                .collect(),
        };

        Ok(AudioBuffer { buffer })
    }

    fn load_flac(_file_path: &str) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        // Implement FLAC loading logic using the FLAC decoder library of your choice
        let audio_buffer = AudioBuffer { buffer: vec![] };
        Ok(audio_buffer)
    }

    fn load_mp3(file_path: &str) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let mut decoder = Decoder::new(file);

        let mut samples = Vec::new();
        loop {
            match decoder.next_frame() {
                Ok(frame) => {
                    let frame_samples: Vec<f32> =
                        frame.data.iter().map(|&sample| sample as f32).collect();
                    samples.extend(frame_samples);
                }
                Err(minimp3::Error::Eof) => break,
                Err(err) => return Err(format!("Error: {}", err).into()),
            }
            }
        let buffer: Vec<u16> = samples
            .iter()
            .map(|s| (*s * std::u16::MAX as f32) as u16)
            .collect();

        Ok(AudioBuffer { buffer })
    }

    pub fn load_audio_data(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let extension = match Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())
        {
            Some(extension) => extension,
            None => {
                return Err((std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "File path has no extension",
                ))
                .into())
            }
        };

        match extension.to_lowercase().as_str() {
            "wav" => Self::load_wav(file_path),
            "flac" => Self::load_flac(file_path),
            "mp3" => Self::load_mp3(file_path),
            _ => Err(
                (std::io::Error::new(std::io::ErrorKind::Other, "Unsupported file extension")).into(),
            ),
        }
    }
    
    pub fn download_file(
        id: i32,
        audio_id: i32,
        database_config: &DatabaseConfig
    ) -> std::result::Result<std::fs::File, Box<dyn std::error::Error>> {
        let audio = Audio::by_id(audio_id, database_config)?;
        if audio.get_user_id() != id {
            return Err("User not authorized".into());
        }

        let audio_file = AudioFile::new(id, audio_id);
        let audio_folder = audio_file.prepare_folder()?;
        let paths = read_dir(audio_folder)?;
        for path in paths {
            let path = path?;
            let path = path.path();
            let file = File::open(path)?;
            return Ok(file);
        }
        Err("File not found".into())
    } 
           
}
