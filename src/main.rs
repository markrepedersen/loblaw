#![feature(type_alias_impl_trait, async_closure, bool_to_option)]

pub mod config;
pub mod dynamic;
pub mod error;
pub mod health_check;
pub mod request;
pub mod timed_future;
pub mod algorithm {
    pub mod algorithm;
    pub mod ip_hash;
    pub mod least_latency;
    pub mod random;
    pub mod round_robin;
    pub mod trie;
    pub mod url_hash;
}

use {
    algorithm::algorithm::{Algorithm, Strategy},
    config::*,
    request::*,
    std::{
        net::SocketAddr,
        str::FromStr,
        sync::{Arc, RwLock},
    },
    tokio::try_join,
};

pub type Threadable<T> = Arc<RwLock<T>>;

pub fn with_read_lock<R, T>(data: Threadable<T>, f: impl FnOnce(&T) -> R) -> R {
    let strategy = &data.read().expect("Could not lock mutex.");
    f(strategy)
}

pub fn with_write_lock<R, T>(data: Threadable<T>, f: impl FnOnce(&mut T) -> R) -> R {
    let strategy = &mut data.write().expect("Could not lock mutex.");
    f(strategy)
}

async fn handle_requests(
    config: Threadable<Config>,
    strategy: Threadable<Strategy>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (ip, port, persistence_type) = with_read_lock(config.clone(), |config| {
        (
            config.ip.clone(),
            config.port.clone(),
            config.persistence_type.clone(),
        )
    });

    match format!("{}:{}", ip, port).parse::<SocketAddr>() {
        Ok(addr) => {
            let handler = RequestHandler::new(addr, persistence_type, strategy);
            handler.run().await
        }
        Err(e) => panic!("Invalid address due to '{}'.", e),
    }
}

fn init() -> Result<(Threadable<Config>, Threadable<Strategy>), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let mut strategy = Strategy::from_str(config.strategy.as_str()).unwrap();
    strategy.configure(&config);
    Ok((
        Arc::new(RwLock::new(config)),
        Arc::new(RwLock::new(strategy)),
    ))
}

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (config, strategy) = init()?;
    if let Err(e) = try_join!(
        handle_requests(config.clone(), strategy.clone()),
        health_check::run(config.clone())
    ) {
        panic!("Error running server: {}.", e);
    }

    Ok(())
}
