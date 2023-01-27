extern crate serde;

use self::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Product {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Company")]
    pub company: String,
}

impl Product {
    pub fn new() -> Self {
        Product {
            name: "TwenorLibrary".to_string(),
            version: "0.0.0".to_string(),
            company: "Twenor".to_string(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn get_company(&self) -> &str {
        &self.company
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_version(&mut self, version: &str) {
        self.version = version.to_string();
    }

    pub fn set_company(&mut self, company: &str) {
        self.company = company.to_string();
    }
}

// Path: src-tauri\lib\recordbox_xml_parser\src\product.rs
