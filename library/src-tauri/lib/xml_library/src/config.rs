use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
#[derive(Debug, PartialEq, Clone)]
pub struct Config {
    path: String,
    config: HashMap<String, String>,
}

impl Config {
    pub fn new(path: &str) -> Config {
        let mut config = HashMap::new();
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                println!("{}", format!("File Path: {}", path));
                let mut file = File::create(path).unwrap();
                file.write_all(b"").unwrap();
                File::open(path).unwrap()
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let lines = contents.lines();
        for line in lines {
            let mut parts = line.split("=");
            let key = parts.next().unwrap().to_string();
            let value = parts.next().unwrap().to_string();
            config.insert(key, value);
        }
        let mut config_class = Config {
            path: path.to_string(),
            config: config,
        };

        config_class.set("path", path);
        config_class.save();
        config_class
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.config.get(key)
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.config.insert(key.to_string(), value.to_string());
    }

    pub fn save(&self) {
        let mut file = File::create(&self.path).unwrap();
        for (key, value) in &self.config {
            let line = format!("{}={}\n", key, value);
            file.write_all(line.as_bytes()).unwrap();
        }
    }

    pub fn to_hashmap(&self) -> HashMap<String, String> {
        self.config.clone()
    }

    pub fn generate_xml_path(&self) -> String {
        let config_path = self.config.get("path").unwrap();
        let config_path_split = config_path.split("/").collect::<Vec<&str>>();
        let mut xml_path = String::new();
        for i in 0..config_path_split.len() - 1 {
            xml_path.push_str(config_path_split[i]);
            xml_path.push_str("/");
        }
        xml_path.push_str("library.xml");
        xml_path
    }
}
