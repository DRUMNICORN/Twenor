/*
DROP TABLE IF EXISTS `SCENE_LIST`;
CREATE TABLE `SCENE_LIST`
(
  `scene_id` int(11) NOT NULL AUTO_INCREMENT,
  `track_id` int(11) DEFAULT NULL,

  `scene_title` varchar(255) DEFAULT NULL,
  `scene_description` mediumtext DEFAULT NULL,
  `scene_tags` varchar(255) DEFAULT NULL,
  `scene_color` varchar(255) DEFAULT NULL,

  PRIMARY KEY (`scene_id`),
  KEY `track_id` (`track_id`),

  CONSTRAINT `SCENE_LIST_ibfk_1` FOREIGN KEY (`track_id`) REFERENCES `TRACK_LIST` (`track_id`) ON DELETE CASCADE ON UPDATE CASCADE,
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
*/

use log::{error, info};
use r2d2_mysql::mysql::{Params, Row};

use crate::{
    chunk::{clear_chunks_by_track_id, generate_chunks, get_chunks},
    correlation::get_correlation,
    db::DatabaseConfig,
    featutes::{clear_features, get_features_by_id, AudioFeatures, AudioFeaturesDB, get_features_by_chunk_id},
    next_state,
    state::TrackState,
    track::update_track_state,
};

pub struct SceneDB {
     scene_id: i32,
    track_id: i32,
    scene_title: String,
    scene_description: String,
    scene_tags: String,
    scene_color: String,
    scene_chunks: Vec<i32>,

    scene_start: f32,
    scene_end: f32,
}

impl SceneDB {
    pub fn get_chunks(&self) -> Vec<i32> {
        self.scene_chunks.clone()
    }

    fn new(
        scene_id: i32,
        track_id: i32,
        scene_title: String,
        scene_description: String,
        scene_tags: String,
        scene_color: String,
        scene_chunks: Vec<i32>,
        scene_start: f32,
        scene_end: f32,
    ) -> SceneDB {
        let scene = SceneDB {
            scene_id,
            track_id,
            scene_title,
            scene_description,
            scene_tags,
            scene_color,
            scene_chunks,
            scene_start,
            scene_end,
        };
        scene
    }

    pub fn save(&self, database_config: &DatabaseConfig) {
        let mut conn = database_config.db_pool.get().unwrap();

        let query = "INSERT INTO SCENE_LIST (track_id, scene_title, scene_description, scene_tags, scene_color, scene_chunks, scene_start, scene_end) VALUES (?, ?, ?, ?, ?, ?, ?, ?)";

        let chunk_ids = self.scene_chunks.clone();
        // serde to string
        let chunk_ids_string = serde_json::to_string(&chunk_ids).unwrap();

        let params: Params = Params::from((
            self.track_id,
            self.scene_title.as_str(),
            self.scene_description.as_str(),
            self.scene_tags.as_str(),
            self.scene_color.as_str(),
            chunk_ids_string.as_str(),
            self.scene_start,
            self.scene_end,
        ));
        conn.prep_exec(query, params)
            .expect("Failed to execute query");
    }

    pub fn load_by_scene_id(scene_id: i32, database_config: &DatabaseConfig) -> SceneDB {
        let mut conn = database_config.db_pool.get().unwrap();

        let query = format!("SELECT * FROM SCENE_LIST WHERE scene_id = {}", scene_id);
        let scene = conn.query(query).unwrap();

        let mut scene_id = -1;
        let mut track_id = -1;
        let mut scene_title = String::new();
        let mut scene_description = String::new();
        let mut scene_tags = String::new();
        let mut scene_color = String::new();
        let mut scene_chunks = Vec::new();
        let mut scene_start = -1.0;
        let mut scene_end = -1.0;

        for row in scene {
            let row = row.unwrap();
            scene_id = row.get("scene_id").unwrap();
            track_id = row.get("track_id").unwrap();
            scene_title = row.get("scene_title").unwrap();
            scene_description = row.get("scene_description").unwrap();
            scene_tags = row.get("scene_tags").unwrap();
            scene_color = row.get("scene_color").unwrap();
            let chunk_ids_string: String = row.get("scene_chunks").unwrap();
            let chunk_ids: Vec<i32> = serde_json::from_str(&chunk_ids_string).unwrap();
            scene_chunks = chunk_ids;
            scene_start = row.get("scene_start").unwrap();
            scene_end = row.get("scene_end").unwrap();
        }

        // serde from string

        let scene_db = SceneDB::new(
            scene_id,
            track_id,
            scene_title,
            scene_description,
            scene_tags,
            scene_color,
            scene_chunks,
            scene_start,
            scene_end,
        );

        scene_db
    }

    pub fn from_row(row: Row) -> SceneDB {
        let mut scene_id = -1;
        let mut track_id = -1;
        let mut scene_title = String::new();
        let mut scene_description = String::new();
        let mut scene_tags = String::new();
        let mut scene_color = String::new();
        let mut scene_chunks = Vec::new();
        let mut scene_start = -1.0;
        let mut scene_end = -1.0;

        scene_id = row.get("scene_id").unwrap();
        track_id = row.get("track_id").unwrap();
        scene_title = row.get("scene_title").unwrap();
        scene_description = row.get("scene_description").unwrap();
        scene_tags = row.get("scene_tags").unwrap();
        scene_color = row.get("scene_color").unwrap();
        let chunk_ids_string: String = row.get("scene_chunks").unwrap();
        let chunk_ids: Vec<i32> = serde_json::from_str(&chunk_ids_string).unwrap();
        scene_chunks = chunk_ids;
        scene_start = row.get("scene_start").unwrap();
        scene_end = row.get("scene_end").unwrap();

        // serde from string

        let scene_db = SceneDB::new(
            scene_id,
            track_id,
            scene_title,
            scene_description,
            scene_tags,
            scene_color,
            scene_chunks,
            scene_start,
            scene_end,
        );

        scene_db
    }
}

pub fn handle_scene_arrangment_state(track_id: i32, database_config: &DatabaseConfig) {
    info!("Loading correlation data");
    let corr_list = get_correlation(track_id, database_config).get_corr_list();

    info!("Loading chunk data");
    let chunk_list = get_chunks(track_id, database_config);

    // check if list have same size
    println!("{} {}", chunk_list.len(), corr_list.len());
    if chunk_list.len() != corr_list.len() {
        // check if corr_list is twice as big as chunk_list
        if chunk_list.len() * 2 == corr_list.len() {
            error!("Chunk list and corr list have different size");
            // remove 50% of the corr_list
            let mut new_corr_list: Vec<f64> = Vec::new();
            for i in 0..corr_list.len() {
                if i % 2 == 0 {
                    new_corr_list.push(corr_list[i]);
                }
            }
        } else {
            panic!("Chunk list and corr list have different size");
        }
    }

    let mut scene_list: Vec<SceneDB> = Vec::new();
    info!("Generating scenes");

    let mut current_chunk_ids: Vec<i32> = Vec::new();
    let mut start_index_chunk = 0;
    let mut end_index_chunk = 0;
    for i in 0..chunk_list.len() {
        if current_chunk_ids.len() == 0 {
            start_index_chunk = i;
        }
        current_chunk_ids.push(chunk_list[i].chunk_id);
        // let chunk = chunk_list[i];
        let corr = corr_list[i];

        let next_corr = corr_list[(i + 1) % corr_list.len()];
        let pev_corr = corr_list[(i + corr_list.len() - 1) % corr_list.len()];

        // check if pev is only in threashold of 20% of and
        // check if next is only in threashold of 20%

        let aplifitude_prev_to_curr = (corr - pev_corr).abs();
        let aplifitude_curr_to_next = (corr - next_corr).abs();

        let diff = aplifitude_prev_to_curr - aplifitude_curr_to_next;
        // abs
        info!("{} {} {}", diff, aplifitude_prev_to_curr, aplifitude_curr_to_next);
        if diff.abs() > 0.20 && aplifitude_prev_to_curr < 0.42 {
            end_index_chunk = i;
            info!("________________________________ {}", i + 1);
            let scene = SceneDB::new(
                -1,
                track_id,
                String::from(format!("Scene {}", scene_list.len() + 1)),
                String::from("No description"),
                String::from("test,foo,bar"),
                String::from("#F0F0F0"),
                current_chunk_ids.clone(),
                chunk_list[start_index_chunk].chunk_start,
                chunk_list[end_index_chunk].chunk_end,
            );
            scene_list.push(scene);
            current_chunk_ids.clear();
            continue;
        }
    }

    info!("Saving scenes");
    clear_scene_arrangment_state(track_id, database_config);
    for scene in scene_list {
        scene.save(database_config);
    }
    next_state(TrackState::Arranging, track_id, database_config);
}

pub fn clear_scene_arrangment_state(track_id: i32, database_config: &DatabaseConfig) {
    let mut conn = database_config.db_pool.get().unwrap();

    let query = format!("DELETE FROM SCENE_LIST WHERE track_id = {}", track_id);
    conn.query(query).unwrap();
}

pub fn get_scenes(track_id: i32, database_config: &DatabaseConfig) -> Vec<SceneDB> {
    let mut conn = database_config.db_pool.get().unwrap();
    let query = format!("SELECT * FROM SCENE_LIST WHERE track_id = {}", track_id);
    info!("Loading scenes");

    info!("Query: {}", query);
    let scenes = conn.query(query).unwrap();

    let mut scene_list: Vec<SceneDB> = Vec::new();
    for row in scenes {
        let row = row.unwrap();
        let scene = SceneDB::from_row(row);
        scene_list.push(scene);
    }

    info!("Loaded {} scenes", scene_list.len());

    scene_list
}

fn get_scene_features(scene_id: i32, database_config: &DatabaseConfig) -> Vec<i32> {
    let mut conn = database_config.db_pool.get().unwrap();
    let query = format!("SELECT * FROM SCENE_FEATURES WHERE scene_id = {}", scene_id);
    info!("Loading scene features");

    info!("Query: {}", query);
    let features = conn.query(query).unwrap();

    let mut feature_list: Vec<i32> = Vec::new();
    for row in features {
        let row = row.unwrap();
        let feature_id: i32 = row.get("feature_id").unwrap();
        feature_list.push(feature_id);
    }

    info!("Loaded {} scene features", feature_list.len());

    feature_list
}

pub fn get_avg_scene_features(scene_chunks_ids: Vec<i32>, database_config: &DatabaseConfig) -> AudioFeatures {
    // let query = format!("SELECT * FROM SCENE_FEATURES WHERE scene_id = {}", scene_id);
    // info!("Loading scene features");

    // info!("Query: {}", query);
    // let features = conn.query(query).unwrap();

    // let mut feature_list: Vec<AudioFeatures> = Vec::new();
    // for row in features {
    //     let row = row.unwrap();
    //     let feature_id: i32 = row.get("feature_id").unwrap();
    //     let feature = get_features_by_id(feature_id, database_config).into();
    //     feature_list.push(feature);
    // }

    // info!("Loaded {} scene features", feature_list.len());
    // let feature_final = calculate_avg_feature(feature_list);
    // info!("Avg scene features: {:?}", feature_final);
    // feature_final

    let mut feature_list: Vec<AudioFeatures> = Vec::new();
    for chunk_id in scene_chunks_ids {
        let chunk = get_features_by_chunk_id(chunk_id, database_config).into();
        feature_list.push(chunk);
    }

    let feature_final = calculate_avg_feature(feature_list);
    info!("Avg scene features: {:?}", feature_final);

    feature_final

}

fn calculate_avg_feature(features: Vec<AudioFeatures>) -> AudioFeatures {
    let len = features.len() as f64;
    let mut feature_final = AudioFeatures::default();
    for feature in features {
        feature_final.acousticness += feature.acousticness;
        feature_final.danceability += feature.danceability;
        feature_final.energy += feature.energy;
        feature_final.instrumentalness += feature.instrumentalness;
        feature_final.liveness += feature.liveness;
        feature_final.loudness += feature.loudness;
        feature_final.speechiness += feature.speechiness;
        feature_final.tempo += feature.tempo;
        feature_final.valence += feature.valence;
        feature_final.key += feature.key;
        feature_final.mode += feature.mode;
        feature_final.time_signature += feature.time_signature;
        feature_final.duration += feature.duration;
    }

    feature_final.acousticness /= len as f64;
    feature_final.danceability /= len as f64;
    feature_final.energy /= len as f64;
    feature_final.instrumentalness /= len as f64;
    feature_final.liveness /= len as f64;
    feature_final.loudness /= len as f64;
    feature_final.speechiness /= len as f64;
    feature_final.tempo /= len as f64;
    feature_final.valence /= len as f64;
    feature_final.key /= len as f64;
    feature_final.mode /= len as f64;
    feature_final.time_signature /= len as f64;
    feature_final.duration /= len as f64;

    feature_final
}
