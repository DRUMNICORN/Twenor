#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    Wav,
    Flac,
    Mp3,
    Other,
}


impl AudioFormat {
    pub fn from_extension(extension: &str) -> AudioFormat {
        match extension {
            "wav" => AudioFormat::Wav,
            "flac" => AudioFormat::Flac,
            "mp3" => AudioFormat::Mp3,
            _ => AudioFormat::Other,
        }
    }

    pub fn from_path(path: &str) -> AudioFormat {
        let extension = path.split(".").last().unwrap_or("");
        AudioFormat::from_extension(extension)
    }
}
