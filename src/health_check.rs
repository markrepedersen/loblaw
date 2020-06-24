use crate::config::Config;
use std::{
    net::Shutdown,
    time::{Duration, Instant},
};
use tokio::{
    io::ErrorKind,
    net::TcpStream,
    spawn,
    time::{delay_for, timeout},
};

// The load balancer should run a health check on each server periodically.
pub async fn health_check(config: &'static Config) -> Result<(), Box<dyn std::error::Error>> {
    let limit = Duration::from_secs(config.health_check.timeout);
    for (_, server) in config.servers.iter() {
        spawn(async move {
            loop {
                let (start, interval, stream) = (
                    Instant::now(),
                    Duration::from_millis(config.health_check.interval * 1000),
                    TcpStream::connect(format!("{}:{}", server.ip, config.health_check.port)),
                );
                match timeout(limit, stream).await.unwrap() {
                    Ok(ref stream) => {
                        if let Err(e) = stream.shutdown(Shutdown::Both) {
                            eprintln!("Error shutting down stream: {}", e);
                        }
                    }
                    Err(ref e) if e.kind() == ErrorKind::TimedOut => {
                        eprintln!("Health check timed out: {}", e)
                    }
                    Err(ref e) => {
                        eprintln!("Health check failed: {:?}", e);
                    }
                };
                let elapsed = start.elapsed();
                if elapsed < interval {
                    delay_for(interval - elapsed).await;
                }
            }
        });
    }
    Ok(())
}
