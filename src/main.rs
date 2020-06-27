#[macro_use]
extern crate lazy_static;

mod config;
mod health_check;
mod request;
mod algorithm {
    pub mod algorithm;
    mod random;
    mod round_robin;
}

use algorithm::algorithm::{Algorithm, Strategy};
use config::Config;
use health_check::HealthCheck;
use request::RequestHandler;
use std::net::SocketAddr;
use std::{str::FromStr, sync::Arc};
use tokio::try_join;

lazy_static! {
    pub static ref CONFIG: Config = Config::parse().unwrap();
    pub static ref STRATEGY: Arc<dyn Algorithm + Send + Sync> = {
        let method = Strategy::from_str(CONFIG.strategy.as_str()).unwrap();
        algorithm::algorithm::build(method)
    };
}

async fn handle_requests() -> Result<(), Box<dyn std::error::Error>> {
    match format!("{}:{}", CONFIG.ip, CONFIG.port).parse::<SocketAddr>() {
        Ok(addr) => {
            let handler = RequestHandler::new(addr);
            handler.run().await
        }
        Err(e) => panic!("Invalid address due to '{}'.", e),
    }
}

async fn health_check() -> Result<(), Box<dyn std::error::Error>> {
    let ip = CONFIG.ip.clone();
    let servers = CONFIG.servers.clone();
    let handler = HealthCheck::new(ip, CONFIG.port, servers);
    handler.run().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = try_join!(handle_requests(), health_check()) {
        panic!("Error running server: {}.", e);
    }

    Ok(())
}
