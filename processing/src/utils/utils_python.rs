use std::{
    error::Error,
    path::Path,
    process::{Command, Stdio}, env,
};

use models::AudioFeaturesPython;

pub fn compute_audio_features(data: &[f64]) -> Result<AudioFeaturesPython, Box<dyn Error>> {
    log::info!("Computing features with Python script");

    if data.is_empty() {
        return Err("Data is empty, please provide data".into());
    }

    let data_str = data.iter().map(ToString::to_string).collect::<Vec<_>>().join(",");

    let python_script_path = env::var("PYTHON_SCRIPT_PATH")?;
    let python_script_path = Path::new(&python_script_path);
    let buffer_file_path = python_script_path.with_file_name("buffers/id.wav");

    let path_parent = buffer_file_path.parent().ok_or_else(|| "Invalid buffer file path")?;
    std::fs::create_dir_all(path_parent)?;
    std::fs::write(&buffer_file_path, data_str)?;

    log::info!("Buffer saved to file: {:?}", buffer_file_path);

    let python_script_path_str = python_script_path.to_str().ok_or("Invalid buffer file path")?;
    let child = Command::new("python")
        .arg(python_script_path_str)
        .arg(buffer_file_path.to_str().ok_or("Invalid buffer file path")?)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let output = child.wait_with_output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        log::info!("Python script stdout: {}", stdout);
        let features: AudioFeaturesPython =
            serde_json::from_str(&stdout).map_err(|e| format!("Failed to parse JSON: {}", e))?;

        log::info!("Features computed with Python script: {:?}", features);
        Ok(features)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::error!("Python script execution failed: {}", stderr);
        Ok(AudioFeaturesPython::default())
    }
}
