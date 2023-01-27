extern crate anyhow;
extern crate log;
extern crate serde_json;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

use anyhow::Result;

use log::{Filename, Log};

use crate::datatypes::StateParameterTypes;

static LOG: Log = Log::new(Filename::Library);

#[derive(Debug, PartialEq, Clone)]
pub struct Config {
    path: String,
    config: HashMap<String, StateParameterTypes>,
}

impl Config {
    pub fn default(path: String) -> Config {
        let mut config: HashMap<String, StateParameterTypes> = HashMap::new();
        config.insert(
            "config_path".to_string(),
            StateParameterTypes::from_string(path.clone()),
        );
        Config {
            path: path.to_string(),
            config: config,
        }
    }

    pub fn from_file(path: String) -> Result<Config> {
        // create default config
        let mut config = Config::default(path);

        // load config from file
        config.load()?;

        // save config to file
        config.save()?;

        Ok(config)
    }

    pub fn load(&mut self) -> Result<()> {
        // check if config file exists
        if !std::path::Path::new(&self.path).exists() {
            return Ok(());
        }

        let mut file = File::open(&self.path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: HashMap<String, StateParameterTypes> = serde_json::from_str(&contents)?;
        self.config = config;
        LOG.debug(&format!("Loaded config: {:?}", self.config));
        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        let mut file = File::create(&self.path)?;
        let json = serde_json::to_string_pretty(&self.config)?;
        file.write_all(json.as_bytes())?;
        LOG.debug(&format!("Saved config: {:?}", self.config));
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&StateParameterTypes> {
        self.config.get(key)
    }

    pub fn set(&mut self, key: &str, value: StateParameterTypes) {
        self.config.insert(key.to_string(), value);
    }

    pub fn to_hashmap(&self) -> HashMap<String, StateParameterTypes> {
        self.config.clone()
    }
}
