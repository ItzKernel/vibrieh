use std::fs as fs;
use serde_json;

pub struct Config;

impl Config {
    pub fn read() -> serde_json::Value {
        let raw_config = fs::read_to_string("./config.json").expect("No config file found");
        serde_json::from_str(&raw_config).expect("Invalid json!")
    }
}