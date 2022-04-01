use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub api_url: String,
    pub tokens: Vec<String>,
}

impl Config {
    pub fn read() -> Config {
        let data = fs::read("../config.json").expect("Failed to read config.json");
        serde_json::from_slice(data.as_slice()).expect("Failed to deserialize config")
    }
}
