pub mod config;
pub mod dynamic;
pub mod health_check;
pub mod request;
pub mod algorithm {
    pub mod algorithm;
    pub mod random;
    pub mod round_robin;
    pub mod trie;
    pub mod url_hash;
}

use {
    algorithm::algorithm::{Algorithm, Strategy},
    config::*,
    request::*,
    std::str::FromStr,
    std::{
        net::SocketAddr,
        sync::{Arc, Mutex},
    },
    tokio::try_join,
};

pub type Threadable<T> = Arc<Mutex<T>>;

/// Start up a config server for dynamic configuration changes using REST API calls.
fn init_config_server() {}

async fn handle_requests(
    config: &Threadable<Config>,
    strategy: &Threadable<Strategy>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (ip, port) = {
        let config = config.clone();
        let config = config.lock().unwrap();
        (config.ip.clone(), config.port.clone())
    };

    match format!("{}:{}", ip, port).parse::<SocketAddr>() {
        Ok(addr) => {
            let handler = RequestHandler::new(addr);
            handler.run(strategy).await
        }
        Err(e) => panic!("Invalid address due to '{}'.", e),
    }
}

// TODO:
// - Maintain session persistence: all requests from a client with same session should be (on config: true) routed to the same server.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (config, strategy) = {
        let config = Config::parse()?;
        let mut strategy = Strategy::from_str(config.strategy.as_str()).unwrap();
        strategy.configure(&config);
        (Arc::new(Mutex::new(config)), Arc::new(Mutex::new(strategy)))
    };

    if let Err(e) = try_join!(
        handle_requests(&config, &strategy),
        health_check::run(&config)
    ) {
        panic!("Error running server: {}.", e);
    }

    Ok(())
}
