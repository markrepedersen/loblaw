mod config;
mod health_check;
mod request;
mod status;
mod algorithm {
    pub mod algorithm;
    mod random;
    mod round_robin;
}

use {
    algorithm::algorithm::*,
    config::*,
    health_check::*,
    request::*,
    status::*,
    std::{
        net::SocketAddr,
        str::FromStr,
        sync::{Arc, Mutex},
    },
    tokio::try_join,
};

pub type Threadable<T> = Arc<Mutex<T>>;

fn init_globals(config: &Config) -> Result<Global, Box<dyn std::error::Error>> {
    let strategy = Strategy::from_str(config.strategy.as_str()).unwrap();
    Ok(Global {
        strategy,
        ip: config.ip.clone(),
        port: config.port.clone(),
    })
}

fn init_backends(config: &Config) -> Result<Vec<Server>, Box<dyn std::error::Error>> {
    let mut servers = Vec::new();

    for backend in config.backends.iter() {
        servers.push(Server {
            status: ServerStatus::Alive,
            ip: backend.ip.clone(),
            port: backend.port.clone(),
            path: backend.path.clone(),
            num_connections: 0,
        })
    }

    Ok(servers)
}

async fn handle_requests(
    ip: &str,
    port: &str,
    config: &Threadable<Global>,
    servers: &Threadable<Vec<Server>>,
) -> Result<(), Box<dyn std::error::Error>> {
    match format!("{}:{}", ip, port).parse::<SocketAddr>() {
        Ok(addr) => {
            let config = config.clone();
            let servers = servers.clone();
            let handler = RequestHandler::new(addr);
            handler.run(config, servers).await
        }
        Err(e) => panic!("Invalid address due to '{}'.", e),
    }
}

async fn health_check(
    timeout: u64,
    interval: u64,
    servers: &Threadable<Vec<Server>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let handler = HealthCheck::new(timeout, interval);
    let servers = servers.clone();
    handler.run(servers).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let servers = Arc::new(Mutex::new(init_backends(&config)?));
    let globals = Arc::new(Mutex::new(init_globals(&config)?));
    let ip = config.ip.as_str();
    let port = config.port.as_str();

    if let Err(e) = try_join!(
        handle_requests(ip, port, &globals, &servers),
        health_check(
            config.health_check.timeout,
            config.health_check.interval,
            &servers
        )
    ) {
        panic!("Error running server: {}.", e);
    }

    Ok(())
}
