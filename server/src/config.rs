use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server_addr: String,
    pub image_width: u32,
    pub image_height: u32,
    pub grid_offset_x: usize,
    pub grid_offset_y: usize,
}

impl Config {
    pub fn read() -> Self {
        let data = fs::read("config.json").expect("Failed to read config.json");
        serde_json::from_slice(data.as_slice()).expect("Failed to decode config")
    }
}
