use std::collections::HashMap;

use recordbox::node::Node;
use serde::{Deserialize, Serialize};

// enum that allows for different types of values
#[derive(Deserialize, Debug, Serialize, Clone, PartialEq)]
pub enum StateParameterTypes {
    String(String),
    Bool(bool),
    I32(i32),
    F32(f32),
    Object(String),
    None,
}

impl StateParameterTypes {
    pub fn to_display(&self) -> String {
        match self {
            StateParameterTypes::String(s) => s.clone(),
            StateParameterTypes::Bool(b) => b.to_string(),
            StateParameterTypes::I32(i) => i.to_string(),
            StateParameterTypes::F32(f) => f.to_string(),
            StateParameterTypes::Object(n) => n.to_string(),
            StateParameterTypes::None => String::new(),
        }
    }

    pub fn from_string(s: String) -> StateParameterTypes {
        match s.parse::<i32>() {
            Ok(i) => StateParameterTypes::I32(i),
            Err(_) => match s.parse::<f32>() {
                Ok(f) => StateParameterTypes::F32(f),
                Err(_) => match s.parse::<bool>() {
                    Ok(b) => StateParameterTypes::Bool(b),
                    Err(_) => match s.parse::<Node>() {
                        Ok(n) => StateParameterTypes::Object(n.to_string()),
                        Err(_) => StateParameterTypes::String(s),
                    },
                },
            },
        }
    }
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub enum Data {
    State(Vec<Property>),
    None,
}

impl Data {
    pub fn unwrap_state(&self) -> Vec<Property> {
        match self {
            Data::State(s) => s.clone(),
            Data::None => Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Property {
    pub key: String,
    pub value: StateParameterTypes,
}

pub fn state_to_hashmap(state: Vec<Property>) -> HashMap<String, StateParameterTypes> {
    let mut map = HashMap::new();
    println!("state_to_hashmap: {:?}", state);
    for property in state {
        println!("state_to_hashmap: {:?}", property);
        map.insert(property.key, property.value);
    }
    map
}
