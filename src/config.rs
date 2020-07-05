use {
    serde::Deserialize,
    std::{collections::HashMap, fs::read_to_string},
};

#[derive(Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Config {
    pub ip: String,
    pub port: String,
    pub strategy: String,
    pub persist_session: bool,
    pub replicas: usize,
    pub backends: HashMap<String, BackendConfig>,
    pub mappings: HashMap<String, StrategyMapping>,
    pub health_check: HealthCheckConfig,
}

impl Config {
    pub fn strategy(&self) -> &String {
        &self.strategy
    }

    pub fn strategy_mut(&mut self) -> &mut String {
        &mut self.strategy
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ip: String::from("127.0.0.1"),
            port: String::from("8080"),
            strategy: String::from("RoundRobin"),
            persist_session: false,
            replicas: 0,
            backends: HashMap::new(),
            mappings: HashMap::new(),
            health_check: HealthCheckConfig::default(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct StrategyMapping {
    pub path: String,
}

#[derive(Debug, Copy, Clone, Deserialize)]
#[allow(dead_code)]
pub enum ServerStatus {
    Alive,
    Busy,
    Dead,
    Throttled,
}

impl Default for ServerStatus {
    fn default() -> Self {
        ServerStatus::Alive
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(default)]
pub struct BackendConfig {
    pub ip: String,
    pub port: String,
    pub path: String,
    pub scheme: String,
    pub status: ServerStatus,
    pub num_connections: u64,
}

impl BackendConfig {
    #[inline]
    #[allow(dead_code)]
    pub fn status(&self) -> &ServerStatus {
        &self.status
    }

    #[inline]
    #[allow(dead_code)]
    pub fn status_mut(&mut self) -> &mut ServerStatus {
        &mut self.status
    }

    #[inline]
    #[allow(dead_code)]
    pub fn ip(&self) -> &String {
        &self.ip
    }

    #[inline]
    #[allow(dead_code)]
    pub fn ip_mut(&mut self) -> &mut String {
        &mut self.ip
    }

    #[inline]
    #[allow(dead_code)]
    pub fn port(&self) -> &String {
        &self.port
    }

    #[inline]
    #[allow(dead_code)]
    pub fn port_mut(&mut self) -> &mut String {
        &mut self.port
    }

    #[inline]
    #[allow(dead_code)]
    pub fn scheme(&self) -> &String {
        &self.scheme
    }

    #[inline]
    #[allow(dead_code)]
    pub fn scheme_mut(&mut self) -> &mut String {
        &mut self.scheme
    }

    #[inline]
    #[allow(dead_code)]
    pub fn path(&self) -> &String {
        &self.path
    }

    #[inline]
    #[allow(dead_code)]
    pub fn path_mut(&mut self) -> &mut String {
        &mut self.path
    }

    #[inline]
    #[allow(dead_code)]
    pub fn num_connections(&self) -> &u64 {
        &self.num_connections
    }

    #[inline]
    #[allow(dead_code)]
    pub fn num_connections_mut(&mut self) -> &mut u64 {
        &mut self.num_connections
    }
}

impl Default for BackendConfig {
    fn default() -> Self {
        Self {
            ip: String::from("127.0.0.1"),
            port: String::from("8080"),
            path: String::from("/backend"),
            scheme: String::from("http"),
            status: ServerStatus::default(),
            num_connections: 0,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct HealthCheckConfig {
    pub timeout: u64,
    pub interval: u64,
    pub healthy_threshold: usize,
    pub unhealthy_threshold: usize,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            timeout: 10,
            interval: 5,
            healthy_threshold: 5,
            unhealthy_threshold: 5,
        }
    }
}

impl Config {
    pub fn parse() -> Result<Self, Box<dyn std::error::Error>> {
        let config: Config = {
            let contents = read_to_string("config.toml")?;
            toml::from_str(contents.as_str())?
        };
        Ok(config)
    }
}
