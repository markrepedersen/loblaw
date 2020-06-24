#[macro_use]
extern crate lazy_static;

mod client;
mod config;
mod health_check;

use config::Config;
use tokio::runtime::Runtime;

lazy_static! {
    static ref Args: Config = config::parse_config_file().unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut health_check_rt = Runtime::new()?;
    let mut request_forwarder_rt = Runtime::new()?;

    health_check_rt.block_on(async {
        if let Err(_) = health_check::health_check(&Args).await {
            eprintln!("Health check thread error.");
        }
    });

    request_forwarder_rt.block_on(async {
        if let Err(_) = client::handle_requests().await {
            eprintln!("Client request thread error.");
        }
    });

    Ok(())
}
