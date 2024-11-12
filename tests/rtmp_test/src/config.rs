use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub address: String,
    pub port: u16,
    pub mongo_uri: String,
    pub encryption_key: Option<[u8; 32]>, // Optional 32-byte encryption key
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_data = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&config_data)?;
        Ok(config)
    }
}
