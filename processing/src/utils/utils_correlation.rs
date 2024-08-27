use models::{VideoScene, DatabaseConfig, AudioFeaturesPython};
use models::AudioChunk;

pub fn compute_correlation(features: &[AudioFeaturesPython], chunks: &[AudioChunk]) -> (Vec<i32>, Vec<f64>) {
    let mut similar_audio_snippets = Vec::with_capacity(features.len());
    let mut chunk_ids = Vec::with_capacity(features.len());

    for (index, (audio_feature, chunk)) in features.iter().zip(chunks.iter()).enumerate() {
        if index >= chunks.len() {
            log::info!("Chunks length is smaller than index");
            break;
        }

        log::debug!("index {}/{}", index + 1, chunks.len());
        let feature_values = audio_feature.into_list();
        let min = feature_values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max = feature_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        let normalized_feature_values: Vec<f64> = feature_values
            .iter()
            .map(|x| (x - min) / (max - min))
            .collect();

        let similarity_score = compute_similarity_score(&normalized_feature_values, &feature_values);
        chunk_ids.push(chunk.get_chunk_id());
        similar_audio_snippets.push(similarity_score);
    }

    (chunk_ids, similar_audio_snippets)
}

fn compute_similarity_score(normalized_feature_values: &[f64], feature_values: &[f64]) -> f64 {
    let mut similarity_score = 0.0;

    if normalized_feature_values.len() != feature_values.len() {
        log::info!("Normalized feature values and feature values have different lengths");
        return similarity_score;
    }

    for (normalized, actual) in normalized_feature_values.iter().zip(feature_values) {
        let diff = actual - normalized;
        similarity_score += diff;
    }

    if similarity_score.is_nan() || similarity_score.is_infinite() {
        log::info!("Similarity score is NaN or infinite");
        similarity_score = 0.0;
    }

    similarity_score
}

pub fn calculate_scene_string(audio_id: i32, scene_features: &[AudioFeaturesPython], scenes: &[VideoScene], database_config: &DatabaseConfig) -> String {
    let mut feature_names = AudioFeaturesPython::names();
    feature_names.sort();

    let mut table = format!("scene\t{}\n", feature_names.join("\t"));

    log::info!("scene_features: {}", scene_features.len());
    log::info!("scenes: {}", scenes.len());

    for (index, scene) in scenes.iter().enumerate() {
        log::info!("scene: {}", index);
        log::info!("feature_names: {}", feature_names.len());

        let avg_feature_values = match AudioFeaturesPython::get_avg(audio_id, scene.get_chunks(), database_config) {
            Ok(avg_feature_values) => avg_feature_values,
            Err(err) => {
                log::info!("Could not get avg feature values: {}", err);
                continue;
            }
        };

        let values = feature_names.iter().map(|feature_name| {
            let feature_value = avg_feature_values.get(feature_name);
            let next_value = scene_features.get(index + 1).and_then(|dict| Some(dict.get(feature_name)));
            
            let state = calculate_state(next_value, feature_value);

            state.to_string()
        });

        let row = format!("{}\t{}\n", index + 1, values.collect::<Vec<_>>().join("\t"));
        table.push_str(&row);
    }

    table
}

fn calculate_state(next_value: Option<f64>, feature_value: f64) -> &'static str {
    let changed_progress = next_value.map_or(0.0, |next| {
        feature_value.log2() - next.log2()
    });

    match changed_progress.abs() {
        p if p > 0.8 => "+3",
        p if p > 0.4 => "+2",
        p if p > 0.2 => "+1",
        p if p < -0.8 => "-3",
        p if p < -0.4 => "-2",
        p if p < -0.2 => "-1",
        _ => "=0",
    }
}