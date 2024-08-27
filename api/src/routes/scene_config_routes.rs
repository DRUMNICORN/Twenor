
pub async fn call_t2v_api(config: &SceneConfig, metadata: &AudioMetadata) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let automatic_api_url = env::var("AUTOMATIC1111_URL")?;

    let url = format!("{}/t2v/run", automatic_api_url);
    let headers = reqwest::header::HeaderMap::new();
    let client = reqwest::Client::new();

    log::info!("Calling API: {}", url);
    log::info!("Config: {:?}", config);
    log::info!("Headers: {:?}", headers);

    let response_result = client.post(url)
        .headers(headers)
        .query(&[
            ("prompt", &config.prompt),
            ("n_prompt", &config.n_prompt),
            ("steps", &config.steps.to_string()),
            ("frames", &config.frames.to_string()),
            ("seed", &config.seed.to_string()),
            ("cfg_scale", &config.cfg_scale.to_string()),
            ("width", &config.width.to_string()),
            ("height", &config.height.to_string()),
            ("do_vid2vid", &config.do_vid2vid.to_string()),
            ("fps", &config.fps.to_string()),
        ])
        .timeout(std::time::Duration::from_secs(60 * 60 * 24 * 7)) // 7 days
        .send()
        .await;

    log::info!("API call complete");
    log::info!("Response: {:?}", response_result);

    let response = match response_result {
        Ok(response) => response,
        Err(error) => {
            log::info!("API call failed: {}", error);
            return Err("API call failed".into());
        }
    };

    if response.status().is_success() {
        log::info!("API call successful");
        let response_data: Value = match response.json().await {
            Ok(response) => response,
            Err(error) => {
                log::info!("API call failed: {}", error);
                return Err("API call failed".into());
            }
        };
        log::info!("{:?}", response_data);

        // Store the response JSON separately

        // Store the status URL separately
        let status_url = response_data["status_url"].as_str().map(|url| url.parse::<reqwest::Url>()?);
        if let Some(status_url) = status_url {
            log::info!("Checking status...");
            let status_response = match reqwest::get(status_url)
                .await {
                    Ok(response) => response,
                    Err(error) => {
                        log::info!("Status check failed: {}", error);
                        return Err("Status check failed".into());
                    }
                };
            if status_response.status().is_success() {
                let status_data: Value = match status_response.json().await {
                    Ok(response) => response,
                    Err(error) => {
                        log::info!("Status check failed: {}", error);
                        return Err("Status check failed".into());
                    }
                };
                log::info!("Status: {}", status_data["status"]);

                // Use both status_data and response_json as needed
            } else {
                log::info!("Status check failed with status code: {}", status_response.status());
                let response = match status_response.text().await {
                    Ok(response) => response,
                    Err(error) => {
                        log::info!("Status check failed: {}", error);
                        return Err("Status check failed".into());
                    }
                };
                log::info!("{}", response);
            }
        }

        // Store the video data separately
        let video_data = response_data["video_data"].as_str()?;
        let folder_path = format!("{}/{}", ".db", metadata.user_id);
        save_video(video_data, &folder_path, 0);
        Ok(())
        
    } else {
        log::info!("API call failed with status code: {}", response.status());
        log::info!("{}", response.text().await?);
        Err("API call failed".into())
    }
}