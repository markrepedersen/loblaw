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

pub async fn health_check(config: &'static Config) -> Result<(), Box<dyn std::error::Error>> {
    let limit = Duration::from_secs(config.health_check.timeout);
    for server in config.servers.iter() {
        spawn(async move {
            loop {
                let (start, interval, stream) = (
                    Instant::now(),
                    Duration::from_millis(config.health_check.interval * 1000),
                    TcpStream::connect(format!("{}:{}", server.ip, config.health_check.port)),
                );
                println!(
                    "[AYA] Sending: [{}:{}] -> [{}:{}].",
                    config.ip, config.port, server.ip, config.health_check.port
                );
                match timeout(limit, stream).await.unwrap() {
                    Ok(ref stream) => {
                        println!(
                            "[IAA] Response received: [{}:{}] -> [{}:{}].",
                            config.ip, config.port, server.ip, config.health_check.port
                        );
                        if let Err(e) = stream.shutdown(Shutdown::Both) {
                            eprintln!("Error shutting down stream: {}", e);
                        }
                    }
                    Err(ref e) if e.kind() == ErrorKind::TimedOut => {
                        eprintln!(
                            "[IAA] Timed out: [{}:{}] -> [{}:{}]",
                            config.ip, config.port, server.ip, config.health_check.port
                        );
                    }
                    Err(ref e) => {
                        eprintln!(
                            "[IAA] Failure ({}): [{}:{}] -> [{}:{}]",
                            e, config.ip, config.port, server.ip, config.health_check.port
                        );
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
