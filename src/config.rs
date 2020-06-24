use serde::Deserialize;
use std::fs::read_to_string;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub replicas: Option<usize>,
    pub servers: Vec<Server>,
    pub health_check: HealthCheck,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Server {
    pub ip: String,
    pub hostname: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HealthCheck {
    pub port: u16,
    pub path: String,
    pub timeout: u64,
    pub interval: u64,
    pub healthy_threshold: usize,
    pub unhealthy_threshold: usize,
}

// Config file should contain the list of server addresses.
pub fn parse_config_file() -> Result<Config, Box<dyn std::error::Error>> {
    let contents: String = read_to_string("config.toml")?;
    let config: Config = toml::from_str(contents.as_str())?;
    Ok(config)
}
