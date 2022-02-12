use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub db_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ip: String::from("127.0.0.1"),
            port: 8080,
            db_path: String::from("./db"),
        }
    }
}

impl Config {
    pub fn load_from_file(path: &str) -> Result<Self, toml::de::Error> {
        let file = fs::read(path).unwrap();
        toml::from_slice(&file)
    }
}