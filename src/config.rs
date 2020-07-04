use serde::Deserialize;
use std::{collections::HashMap, fs::read_to_string};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub ip: String,
    pub port: String,
    pub strategy: String,
    pub backends: HashMap<String, BackendConfig>,
    pub mappings: HashMap<String, StrategyMapping>,
    pub health_check: HealthCheckConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StrategyMapping {
    pub path: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BackendConfig {
    pub ip: String,
    pub port: String,
    pub path: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HealthCheckConfig {
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
