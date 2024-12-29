use std::{collections::HashMap, fs};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub supplier_private_key: String,
    pub network: Vec<HashMap<String, String>>,
}

impl Config {
    pub fn from_file(path: &str) -> Self {
        let config_data = fs::read_to_string(path).expect("Unable to read config file");
        serde_json::from_str(&config_data).expect("Unable to parse config file")
    }
}
