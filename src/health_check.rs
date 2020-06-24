use crate::config::Config;
use std::time::{Duration, Instant};
use tokio::{
    net::TcpStream,
    spawn,
    time::{delay_for, timeout},
};

// The load balancer should run a health check on each server periodically.
pub async fn health_check(config: &'static Config) -> Result<(), Box<dyn std::error::Error>> {
    let limit = Duration::from_secs(config.health_check.timeout as u64);
    loop {
        for server in config.servers.iter() {
            spawn(async move {
                let (start, interval, stream) = (
                    Instant::now(),
                    Duration::from_millis(config.health_check.interval * 1000),
                    TcpStream::connect(format!("{}:{}", server.ip, config.health_check.port)),
                );

                match timeout(limit, stream).await.unwrap() {
                    Ok(_) => {}
                    Err(_) => {
                        eprintln!("Health check timed out.");
                    }
                }

                let elapsed = start.elapsed();
                if elapsed < interval {
                    delay_for(interval - elapsed).await;
                }
            });
        }
    }
}
