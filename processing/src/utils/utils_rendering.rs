use std::{
    env,
    error::Error,
    fs,
    io::Write,
    path::PathBuf
};

use base64_light;
use models::{SceneConfig, AudioMetadata};
use reqwest::Client;
use serde_json::Value;
use time::Instant;



fn save_response_json(
    response_json: &Value,
    user_id: i32,
    audio_id: i32,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    let folder_path = format!("{}/{}/{}", ".db", user_id, audio_id);
    fs::create_dir_all(&folder_path)?;

    let file_path = format!("{}/{}", folder_path, filename);
    let json_data = serde_json::to_string_pretty(response_json)?;
    fs::write(&file_path, json_data)?;

    Ok(())
}

pub async fn call_t2v_api(
    config: &SceneConfig,
    metadata: &AudioMetadata,
    index: usize,
) -> Result<(), Box<dyn Error>> {
    let automatic_api_url = env::var("AUTOMATIC1111_URL")?;
    let url = format!("{}/t2v/run", automatic_api_url);
    let client = Client::new();

    let response_result = client
        .post(&url)
        .query(&[
            ("prompt", &config.get_prompt()),
            ("n_prompt", &config.get_n_prompt()), /* ... other params */
        ])
        .timeout(std::time::Duration::from_secs(60 * 60 * 24 * 7))
        .send()
        .await?;

    let response = response_result.error_for_status()?;

    if response.status().is_success() {
        log::info!("API call successful");
        let response_data: Value = response.json().await?;
        // save in dir as json
        match save_response_json(&response_data, metadata.get_user_id(), metadata.get_user_id(), "response.json") {
            Ok(_) => {}
            Err(e) => {
                log::info!("Error saving response json: {}", e);
                return Err("Error saving response json".into());
            }
        }

        // Store the status URL separately
        // let status_url = response_data["status_url"]
        //     .as_str()
        //     .map(|url| url.parse::<reqwest::Url>()?);

        let status_url = match response_data["status_url"].as_str() {
            Some(url) => url.parse::<reqwest::Url>().ok(),
            None => {
                log::info!("No status_url found in response");
                None
            }
        };

        if let Some(status_url) = status_url {
            log::info!("Checking status...");
            let status_response = reqwest::get(status_url).await?;
            if status_response.status().is_success() {
                let status_data: Value = status_response.json().await?;
                log::info!("Status: {}", status_data["status"]);

                // Use both status_data and response_json as needed
            } else {
                log::info!(
                    "Status check failed with status code: {}",
                    status_response.status()
                );
                log::info!("{}", status_response.text().await?);
            }
        }

        // Store the video data separately
        log::info!("Saving video data...");
        // check length of mp4s array
        let mp4s_length = match response_data["mp4s"].as_array() {
            Some(mp4s) => mp4s.len(),
            None => {
                return Err("No video data found in response".into());
            }
        };

        log::info!("mp4s_length: {}", mp4s_length);
        if mp4s_length == 0 {
            return Err("No video data found in response".into());
        }
        let video_data = match response_data["mp4s"].get(0) {
            Some(video_data) => video_data,
            None => {
                return Err("No video data found in response".into());
            }
        };
        let folder_path = format!("{}/{}/{}", ".db", metadata.get_user_id(), metadata.get_audio_id());
        match save_video(video_data.to_string().as_str(), &folder_path, index) {
            Ok(_) => {
                log::info!("Video saved successfully");
                Ok(())
            }
            Err(e) => {
                log::info!("Error saving video: {}", e);
                log::info!(
                    "video_data (first 420 chars): {}",
                    video_data.to_string().chars().take(420).collect::<String>()
                );
                log::debug!("folder_path: {}", folder_path);
                log::debug!("metadata.user_id: {}", metadata.get_user_id());

                Err("Error saving video".into())
            }
        }
    } else {
        log::info!("API call failed with status code: {}", response.status());
        log::info!("{}", response.text().await?);
        Err("API call failed".into())
    }
}

fn save_video(
    video_data: &str,
    folder_path: &str,
    index: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Saving video...");

    {
        let folder_path = PathBuf::from(folder_path);
        if !folder_path.exists() {
            log::info!("Folder does not exist: {:?}", folder_path);
            fs::create_dir_all(&folder_path)?;
        }
    }

    let file_path = format!("{}/{}.mp4", folder_path, index);
    log::info!("Saving video: {}", file_path);

    let video_data = video_data.split(";base64,").last().ok_or_else(|| {
        log::info!("No video data found in response");
        "No video data found in response"
    })?;
    if video_data.is_empty() {
        log::info!("No video data found in response");
        return Err("No video data found in response".into());
    }

    log::info!("Decoding video data...");
    let video_data = base64_light::base64_decode(video_data);
    log::info!("Video data decoded");

    {
        let mut file = fs::File::create(&file_path)?;
        file.write_all(&video_data)?;

        log::info!("Video saved successfully");

        Ok(())
    }
}

pub fn calculate_eta(start_time: Instant, current_index: usize, total_videos: usize) -> f64 {
    let elapsed_time = Instant::now() - start_time;
    let time_per_video = elapsed_time.as_seconds_f64() / (current_index + 1) as f64;
    let remaining_videos = total_videos - (current_index + 1);
    let eta = time_per_video * remaining_videos as f64;
    eta
}
