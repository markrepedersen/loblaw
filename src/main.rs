pub mod config;
pub mod dynamic;
pub mod health_check;
pub mod request;
pub mod algorithm {
    pub mod algorithm;
    pub mod ip_hash;
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

fn with_lock<R, T>(data: Threadable<T>, f: impl FnOnce(&mut T) -> R) -> R {
    let strategy = &mut data.lock().expect("Could not lock mutex.");
    f(strategy)
}

/// Start up a config server for dynamic configuration changes using REST API calls.
fn init_config_server() {}

async fn handle_requests(
    config: &Threadable<Config>,
    strategy: &Threadable<Strategy>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (ip, port, persistence_type) = with_lock(config.clone(), |config| {
        (
            config.ip.clone(),
            config.port.clone(),
            config.persistence_type.clone(),
        )
    });

    match format!("{}:{}", ip, port).parse::<SocketAddr>() {
        Ok(addr) => {
            let handler = RequestHandler::new(addr);
            handler.run(strategy, persistence_type).await
        }
        Err(e) => panic!("Invalid address due to '{}'.", e),
    }
}

fn init() -> Result<(Threadable<Config>, Threadable<Strategy>), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let mut strategy = Strategy::from_str(config.strategy.as_str()).unwrap();
    strategy.configure(&config);
    Ok((Arc::new(Mutex::new(config)), Arc::new(Mutex::new(strategy))))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (config, strategy) = init()?;
    if let Err(e) = try_join!(
        handle_requests(&config, &strategy),
        health_check::run(&config)
    ) {
        panic!("Error running server: {}.", e);
    }

    Ok(())
}
