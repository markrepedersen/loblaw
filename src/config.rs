use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    replicas: usize,
    servers: Servers,
}

#[derive(Deserialize)]
struct Servers {
    ip: Option<String>,
    hostname: Option<String>,
}
