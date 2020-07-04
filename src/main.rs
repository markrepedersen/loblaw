pub mod config;
pub mod dynamic;
pub mod health_check;
pub mod request;
pub mod status;
pub mod algorithm {
    pub mod algorithm;
    pub mod random;
    pub mod round_robin;
    pub mod trie;
    pub mod url_hash;
}

use {
    algorithm::algorithm::*,
    config::*,
    request::*,
    status::*,
    std::{
        net::SocketAddr,
        rc::Rc,
        sync::{Arc, Mutex},
    },
    tokio::try_join,
};

pub type Threadable<T> = Arc<Mutex<T>>;

fn init_globals(config: &Config) -> Result<Global, Box<dyn std::error::Error>> {
    let strategy = Strategy::new(config);

    Ok(Global {
        strategy,
        ip: config.ip.clone(),
        port: config.port.clone(),
    })
}

/// Start up a config server for dynamic configuration changes using REST API calls.
fn init_config_server() {}

async fn handle_requests(
    ip: &str,
    port: &str,
    config: &Threadable<Global>,
) -> Result<(), Box<dyn std::error::Error>> {
    match format!("{}:{}", ip, port).parse::<SocketAddr>() {
        Ok(addr) => {
            let config = config.clone();
            let handler = RequestHandler::new(addr);
            handler.run(config).await
        }
        Err(e) => panic!("Invalid address due to '{}'.", e),
    }
}

// TODO:
// - Maintain session persistence: all requests from a client with same session should be (on config: true) routed to the same server.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Rc::new(Config::parse()?);
    let globals = Arc::new(Mutex::new(init_globals(&config)?));
    let ip = config.ip.as_str();
    let port = config.port.as_str();

    if let Err(e) = try_join!(
        handle_requests(ip, port, &globals),
        health_check::run(&config)
    ) {
        panic!("Error running server: {}.", e);
    }

    Ok(())
}
