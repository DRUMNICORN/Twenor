use log::info;
use r2d2_mysql::mysql::Params;

use crate::{db::DatabaseConfig, scenes::{get_scenes, SceneDB, get_avg_scene_features}, metadata::{get_metadata, get_metadata_by_track_id, Metadata}, featutes::{get_features, AudioFeaturesDB}};

struct Config {
    INSTRUCTION: String,
    BASE_PROMPT: String,
    DIRECTOR_PROMPT: String,
    GOAL_PROMPT: String,
    OUTPUT_FORMAT_PROMPT: String,
    EXAMPLE_SCENE: String,
    TOPIC_GUIDANCE_PROMPT: String,
}

 impl Config {
    pub fn default() -> Config {
        Config {
            INSTRUCTION: "[INSTRUCTION]".to_string(),
            BASE_PROMPT: "Write a music video script based on the following track information.".to_string(),
            DIRECTOR_PROMPT: "Act as a director and create a script for a music video based on the given track information.".to_string(),
            GOAL_PROMPT: "Write an interesting and engaging script.".to_string(),
            OUTPUT_FORMAT_PROMPT: "Output a JSON array of scenes in the following format:".to_string(),
            EXAMPLE_SCENE: "{
                \"script\": {
                    \"title\": \"...\",  # title of the script
                    \"description\": \"...\",  # description of the script, one sentence (up to twenty words)
                },
                \"scenes\": [-
                    {
                        \"scene_id\": 0,  # unique scene id
                        \"scene_title\": \"...\",  # title of the scene
                        \"scene_description\": \"...\",  # description of the scene, one sentence (up to twenty words)
                        \"scene_tags\": [\"...\", \"...\", \"...\"],  # tags of the scene (up to twenty tags)
                        \"scene_color\": \"#...\",  # creative fitting Color of the scene in hex format
                    },
                    { 
                        # Additional scenes
                    },
                    ...
                ]
            }".to_string(),
            TOPIC_GUIDANCE_PROMPT: "Use the following track information as guidance and follow the instructions below:".to_string(),
        }
    }
 }

pub fn handle_writing_state(track_id: i32, database_config: &DatabaseConfig) {
    let t_scenes = get_scenes(track_id, database_config);
    let t_scenes_len = t_scenes.len();
    let t_metadata = get_metadata_by_track_id(track_id, database_config);
    let t_scene_features = get_features(track_id, database_config);
    
    let config: Config = Config::default();

    // Generate the story and plot the scene features
    let t_story = combine_scene_values(t_scene_features, t_scenes, database_config);
    // Generate the prompt
    let t_prompt = generate_prompt(t_story, t_metadata, t_scenes_len, config);
    println!("{}", t_prompt);

    let script = ScriptDB::new(-1, track_id, t_prompt);
    script.save_script(database_config);


    info!("Done handling writing state");

}
fn combine_scene_values(scene_features: Vec<AudioFeaturesDB>, scenes: Vec<SceneDB>, database_config: &DatabaseConfig) -> String {   
    let mut feature_names = AudioFeaturesDB::names();
    feature_names.sort();

    let mut table = format!("scene\t{}\n", feature_names.join("\t"));

    // for (index, feature_dict) in scene_features.iter().enumerate() {
    //     let mut values = Vec::new();

    //     for feature_name in &feature_names {
    //         let feature_value = feature_dict.get(feature_name);
    //         let next_value_dict = scene_features.get(index + 1);
    //         let next_value = next_value_dict.and_then(|dict| Some(dict.get(feature_name)));

    //         let state = if let Some(next_value) = next_value {
    //             if feature_value > next_value {
    //                 "++"
    //             } else if feature_value < next_value {
    //                 "--"
    //             } else {
    //                 "="
    //             }
    //         } else {
    //             if feature_value > 0.5 {
    //                 "+"
    //             } else if feature_value < 0.5 {
    //                 "-"
    //             } else {
    //                 "0"
    //             }
    //         };

    //         values.push(state.to_string());
    //     }

    //     let row = format!("{}\t{}\n", index, values.join("\t"));
    //     table.push_str(&row);
    // }

    // loop all scenes check when scene starts and end and than add corresponding feature values 
    info!("scene_features: {}", scene_features.len());
    info!("scenes: {}", scenes.len());
    for (index, scene) in scenes.iter().enumerate() {
        let avg_feature_values = get_avg_scene_features(scene.get_chunks(), database_config);

        let mut values = Vec::new();
        info!("scene: {}", index);
        info!("feature_names: {}", feature_names.len());
        for feature_name in &feature_names {
            let feature_value = avg_feature_values.get(feature_name);
            let next_value_dict = scene_features.get(index + 1);
            let next_value = next_value_dict.and_then(|dict| Some(dict.get(feature_name)));

            let state = if let Some(next_value) = next_value {
                if feature_value > next_value {
                    "++"
                } else if feature_value < next_value {
                    "--"
                } else {
                    "="
                }
            } else {
                if feature_value > 0.5 {
                    "+"
                } else if feature_value < 0.5 {
                    "-"
                } else {
                    "0"
                }
            };

            values.push(state.to_string());
        }

        let row = format!("{}\t{}\n", index, values.join("\t"));
        table.push_str(&row);
    }

    

    table
}

fn generate_prompt(story: String, metadata: Metadata, num_scenes: usize, config: Config) -> String {
    let mut prompt = format!(
        "{}\n{}\n{}\n{}\n{}\n",
        config.INSTRUCTION,
        config.BASE_PROMPT,
        config.DIRECTOR_PROMPT,
        config.GOAL_PROMPT,
        config.OUTPUT_FORMAT_PROMPT,
    );

    let example_scene = config.EXAMPLE_SCENE;
    prompt.push_str(&format!("{}\n\n", example_scene));

    prompt.push_str(&format!("Now, write a story about a movie with {} scenes.\n", num_scenes));

    prompt.push_str(&format!("{}\n", config.TOPIC_GUIDANCE_PROMPT));
    for (key, value) in metadata.iter() {
        prompt.push_str(&format!("{}: {}\n", key, value));
    }

    prompt.push_str(&format!("\n{}", story));

    prompt
}


//ALL for script now

/*DROP TABLE IF EXISTS `SCRIPT_LIST`;
CREATE TABLE `SCRIPT_LIST`
(
  `script_id` int(11) NOT NULL AUTO_INCREMENT,
  `track_id` int(11) DEFAULT NULL,

  `script_prompt` mediumtext DEFAULT NULL,

  PRIMARY KEY (`script_id`),
  KEY `track_id` (`track_id`),

  CONSTRAINT `SCRIPT_LIST_ibfk_1` FOREIGN KEY (`track_id`) REFERENCES `TRACK_LIST` (`track_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

*/

pub struct ScriptDB {
    script_id: i32,
    track_id: i32,
    script_prompt: String,
    script_response: String,
}

impl ScriptDB {
    pub fn get_prompt(&self) -> String {
        self.script_prompt.clone()
    }

    fn new(
        script_id: i32,
        track_id: i32,
        script_prompt: String,
    ) -> ScriptDB {
        let script = ScriptDB {
            script_id,
            track_id,
            script_prompt,
            script_response: "".to_string(),
        };

        script
    }

    pub fn load_script(track_id: i32, database_config: &DatabaseConfig) -> ScriptDB {
        let mut conn = database_config.db_pool.get().unwrap();

        let query = format!("SELECT * FROM SCRIPT_LIST WHERE track_id = {}", track_id);
        let script = conn.query(query).unwrap();

        let mut script_id = -1;
        let mut track_id = -1;
        let mut script_prompt = "".to_string();

        for row in script {
            let row = row.unwrap();
            script_id = row.get("script_id").unwrap();
            track_id = row.get("track_id").unwrap();
            script_prompt = row.get("script_prompt").unwrap();
        }

        ScriptDB::new(script_id, track_id, script_prompt)
    }

    fn save_script(&self, database_config: &DatabaseConfig) {
        if ScriptDB::check_exists(self.track_id, database_config) {
            self.update_script(database_config);
            return;
        }
        
        let mut conn = database_config.db_pool.get().unwrap();

        let query = "INSERT INTO SCRIPT_LIST (track_id, script_prompt) VALUES (?, ?)";
        let params: Params = Params::from((self.track_id, self.script_prompt.as_str()));
        conn.prep_exec(query, params)
            .expect("Failed to execute query");
    }

    fn check_exists(track_id: i32, database_config: &DatabaseConfig) -> bool {
        let mut conn = database_config.db_pool.get().unwrap();

        let query = format!("SELECT * FROM SCRIPT_LIST WHERE track_id = {}", track_id);
        let script = conn.query(query).unwrap();

        let mut script_id = -1;
        let mut track_id = -1;
        let mut script_prompt = "".to_string();

        for row in script {
            let row = row.unwrap();
            script_id = row.get("script_id").unwrap();
            track_id = row.get("track_id").unwrap();
            script_prompt = row.get("script_prompt").unwrap();
        }

        if script_id == -1 {
            false
        } else {
            true
        }
    }

    fn update_script(&self, database_config: &DatabaseConfig) {
        let mut conn = database_config.db_pool.get().unwrap();

        let query = "UPDATE SCRIPT_LIST SET script_prompt = ? WHERE track_id = ?";
        let params: Params = Params::from((self.script_prompt.as_str(), self.track_id));
        conn.prep_exec(query, params)
            .expect("Failed to execute query");
    }
}