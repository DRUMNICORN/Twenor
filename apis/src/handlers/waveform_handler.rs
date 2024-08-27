use std::path::Path;
use crate::models::user::AuthorizationHeaderToken;
use crate::utils::user_utils::check_user_storage;
use crate::utils::user_utils::Waveform;
use std::fs;
use std::fs::File;

use hound::WavReader;

use minimp3::Decoder;

fn load_audio_data(file_path: &str) -> std::io::Result<Waveform> {
    let extension = Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    match extension.to_lowercase().as_str() {
        "wav" => load_wav(file_path),
        "flac" => load_flac(file_path),
        "mp3" => load_mp3(file_path),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unsupported audio format",
        )),
    }
}

fn load_wav(file_path: &str) -> std::io::Result<Waveform> {
    let mut reader = WavReader::open(file_path).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    })?;

    let samples: Vec<f32> = match reader.spec().sample_format {
        hound::SampleFormat::Float => reader.samples().map(|s| s.unwrap()).collect(),
        hound::SampleFormat::Int => reader
            .samples::<i16>()
            .map(|s| s.unwrap() as f32 / std::i16::MAX as f32)
            .collect(),
    };

    let buffer: Vec<u16> = samples
        .iter()
        .map(|s| (*s * std::u16::MAX as f32) as u16)
        .collect();

    Ok(Waveform { buffer })
}


fn load_flac(_file_path: &str) -> std::io::Result<Waveform> {
    // Implement FLAC loading logic using the FLAC decoder library of your choice
    unimplemented!()
}

fn load_mp3(file_path: &str) -> std::io::Result<Waveform> {
    let file = File::open(file_path)?;
    let mut decoder = Decoder::new(file);

    let mut samples = Vec::new();
    loop {
        match decoder.next_frame() {
            Ok(frame) => {
                let frame_samples: Vec<f32> = frame.data.iter().map(|&sample| sample as f32).collect();
                samples.extend(frame_samples);
            }
            Err(minimp3::Error::Eof) => break,
            Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
        }
    }
    let buffer: Vec<u16> = samples.iter().map(|s| (*s * std::u16::MAX as f32) as u16).collect();

    Ok(Waveform { buffer })
}



pub fn get_waveform(token: AuthorizationHeaderToken) -> std::io::Result<String> {
    check_user_storage(&token.id)?;

    let user_path = Path::new(".db/audio").join(&token.id);

    // Search for .temp file in user's directory
    let mut temp_file_path = String::new();
    
    for entry in fs::read_dir(user_path)? {
        let entry = entry?;
        let path = entry.path();
        let path_str = path.to_str().unwrap().to_string();
        if path_str.ends_with(".temp") {
            temp_file_path = path_str;
            break;
        }
    }

    if temp_file_path.is_empty() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Temp file not found"));
    }
    
    let temp_file_path = Path::new(&temp_file_path);
    
    let file_path = temp_file_path.with_extension(""); // Removes the .temp extension
    
    let audio_data = load_audio_data(file_path.to_str().unwrap())?;
    let json = serde_json::to_string(&audio_data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    Ok(json)
}