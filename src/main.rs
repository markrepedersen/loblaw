mod config;
mod health_check;
mod request;
mod algorithm {
    pub mod algorithm;
    mod round_robin;
    mod random;
}

use algorithm::algorithm::Strategy;
use config::Config;
use health_check::HealthCheck;
use request::RequestHandler;
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::try_join;

async fn handle_requests(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let strategy = config.strategy.clone();
    match format!("{}:{}", config.ip, config.port).parse::<SocketAddr>() {
        Ok(addr) => {
            let strategy = match strategy {
                Some(s) => Strategy::from_str(s.as_str())?,
                None => Strategy::RoundRobin,
            };
            let handler = RequestHandler::new(addr, strategy);
            handler.run().await
        }
        Err(e) => panic!("Invalid address due to '{}'.", e),
    }
}

async fn health_check(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let ip = config.ip.clone();
    let servers = config.servers.clone();
    let handler = HealthCheck::new(ip, config.port, servers);
    handler.run().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    if let Err(e) = try_join!(handle_requests(&config), health_check(&config)) {
        panic!("Error running server: {}.", e);
    }

    Ok(())
}
