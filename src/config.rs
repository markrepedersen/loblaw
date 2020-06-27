use serde::Deserialize;
use std::fs::read_to_string;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub strategy: String,
    pub servers: Vec<Server>,
    pub health_check: HealthCheck,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Server {
    pub ip: String,
    pub port: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HealthCheck {
    pub timeout: u64,
    pub interval: u64,
    pub healthy_threshold: usize,
    pub unhealthy_threshold: usize,
}

impl Config {
    pub fn parse() -> Result<Self, Box<dyn std::error::Error>> {
        let contents: String = read_to_string("config.toml")?;
        let config: Config = toml::from_str(contents.as_str())?;
        Ok(config)
    }
}
