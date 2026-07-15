use std::fs;
use serde_json;
use serde::Deserialize;


#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub ip: String,
    pub port: String,
    pub server_path: String,
    pub max_size: u16,
    pub max_clients: u16,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let res: String = fs::read_to_string("__config__.json")?;
        Ok(serde_json::from_str(&res)?)
    }

    pub fn socket(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}