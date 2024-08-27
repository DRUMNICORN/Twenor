use std::cmp::min;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use tauri::http::HttpRange;
use tauri::http::Request;
use tauri::http::Response;
use tauri::http::ResponseBuilder;

use log::Filename;
use log::Log;
use log::LogLevel;
static LOG: Log = Log::new_with_level(Filename::Stream, LogLevel::Error);

pub enum AcceptedFileType {
    MP3,
    OGG,
    FLAC,
    WAV,
}

impl From<&str> for AcceptedFileType {
    fn from(s: &str) -> Self {
        match s {
            "mp3" => AcceptedFileType::MP3,
            "ogg" => AcceptedFileType::OGG,
            "flac" => AcceptedFileType::FLAC,
            "wav" => AcceptedFileType::WAV,
            _ => panic!("Unsupported file type"),
        }
    }
}

impl From<AcceptedFileType> for &str {
    fn from(s: AcceptedFileType) -> Self {
        match s {
            AcceptedFileType::MP3 => "audio/mpeg",
            AcceptedFileType::OGG => "audio/ogg",
            AcceptedFileType::FLAC => "audio/flac",
            AcceptedFileType::WAV => "audio/wav",
        }
    }
}

fn get_track_type(extension: &String) -> String {
    match extension.as_str() {
        "mp3" => "audio/mpeg".to_string(),
        "ogg" => "audio/ogg".to_string(),
        "flac" => "audio/flac".to_string(),
        "wav" => "audio/wav".to_string(),
        _ => "audio/mpeg".to_string(),
    }
}

fn generate_response(
    track_src: &String,
    request: &Request,
    track_type: &String,
) -> Result<Response, Box<dyn std::error::Error>> {
    LOG.debug(&format!("Generating response for {}", track_src));
    let mut response = ResponseBuilder::new();
    let mut content = std::fs::File::open(&track_src)?;
    let mut buf = Vec::new();
    let mut status_code = 200;
    if let Some(range) = request.headers().get("range") {
        let file_size = content.metadata().unwrap().len();
        let range = HttpRange::parse(range.to_str().unwrap(), file_size).unwrap();
        let first_range = range.first();
        if let Some(range) = first_range {
            let mut real_length = range.length;

            if range.length > file_size / 3 {
                real_length = min(file_size - range.start, 1024 * 400);
            }

            let last_byte = range.start + real_length - 1;
            status_code = 206;
            response = response
                .header("Connection", "Keep-Alive")
                .header("Accept-Ranges", "bytes")
                .header("Content-Length", real_length)
                .header("Access-Control-Allow-Origin", "*")
                .header(
                    "Content-Range",
                    format!("bytes {}-{}/{}", range.start, last_byte, file_size),
                );
            // FIXME: Add ENode support (caching on the webview)
            content.seek(SeekFrom::Start(range.start))?;
            content.take(real_length).read_to_end(&mut buf)?;
        } else {
            content.read_to_end(&mut buf)?;
        }
    }

    response.mimetype(&track_type).status(status_code).body(buf)
}

fn extract_parts(request: &Request) -> (String, String) {
    #[cfg(target_os = "windows")]
    let path = request.uri().strip_prefix("stream://localhost/").unwrap();

    #[cfg(not(target_os = "windows"))]
    let path = request.uri().strip_prefix("stream://").unwrap();

    let path = percent_encoding::percent_decode(path.as_bytes())
        .decode_utf8_lossy()
        .to_string();

    let (path, extension) = path.split_once('.').unwrap();
    (format!("{}.{}", path, extension), extension.to_string())
}

pub fn handle_request(request: &Request) -> Result<Response, Box<dyn std::error::Error>> {
    let (track_src, extension) = extract_parts(request);
    let response = generate_response(&track_src, request, &get_track_type(&extension));
    return Ok(response?);
}
