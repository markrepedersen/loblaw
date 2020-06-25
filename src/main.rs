#[macro_use]
extern crate lazy_static;

mod config;
mod health_check;
mod request;
mod algorithm {
    pub mod algorithm;
    mod round_robin;
}

use config::Config;
use std::net::SocketAddr;
use tokio::try_join;

lazy_static! {
    static ref ARGS: Config = Config::parse().unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match format!("{}:{}", ARGS.ip, ARGS.port).parse::<SocketAddr>() {
        Ok(addr) => {
            if let Err(e) = try_join!(
                request::handle_requests(&addr),
                health_check::health_check(&ARGS),
            ) {
                eprintln!("Error running server: {}.", e);
            }
        }
        Err(e) => eprintln!("Invalid address: {}.", e),
    }

    Ok(())
}
