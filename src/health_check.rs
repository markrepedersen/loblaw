use {
    crate::{status::Server, Threadable},
    std::{
        net::Shutdown,
        time::{Duration, Instant},
    },
    tokio::{
        io::ErrorKind,
        net::TcpStream,
        spawn,
        time::{delay_for, timeout},
    },
};

pub struct HealthCheck {
    ip: String,
    port: String,
    timeout: u64,
    interval: u64,
}

impl HealthCheck {
    pub fn new(ip: &str, port: &str) -> Self {
        Self {
            ip: ip.to_string(),
            port: port.to_string(),
            timeout: 10,
            interval: 5,
        }
    }

    pub async fn run(
        &self,
        servers: Threadable<Vec<Server>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let limit = Duration::from_secs(self.timeout);
        let interval = self.interval.clone();
        let servers = {
            let s = servers.clone();
            let s = s.lock().unwrap();
            s.clone()
        };

        for server in servers.into_iter() {
            spawn(async move {
                loop {
                    let (start, interval, stream) = (
                        Instant::now(),
                        Duration::from_millis(interval * 1000),
                        TcpStream::connect(format!("{}:{}", server.ip, server.port)),
                    );
                    println!(
                        "[HEALTH] Sending health check to '{}:{}'.",
                        server.ip, server.port
                    );
                    match timeout(limit, stream).await.unwrap() {
                        Ok(ref stream) => {
                            if let Err(e) = stream.shutdown(Shutdown::Both) {
                                eprintln!("Error shutting down stream: {}", e);
                            }
                            println!(
                                "[HEALTH] Received response from '{}:{}'.",
                                server.ip, server.port
                            );
                        }
                        Err(ref e) if e.kind() == ErrorKind::TimedOut => {}
                        Err(ref e) => eprintln!("Error sending health check: {}.", e),
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
}
