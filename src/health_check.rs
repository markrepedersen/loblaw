use {
    crate::config::Server,
    futures,
    std::{
        net::Shutdown,
        time::{Duration, Instant},
    },
    tokio::{
        io::ErrorKind,
        join,
        net::TcpStream,
        spawn,
        task::JoinHandle,
        time::{delay_for, timeout},
    },
};

pub struct HealthCheck {
    ip: String,
    port: u16,
    servers: Vec<Server>,
    timeout: u64,
    interval: u64,
}

impl HealthCheck {
    pub fn new(ip: String, port: u16, servers: Vec<Server>) -> Self {
        Self {
            ip,
            port,
            servers,
            timeout: 10,
            interval: 5,
        }
    }

    pub fn with_timeout(&mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        *self
    }

    pub fn with_interval(&mut self, interval: u64) -> Self {
        self.interval = interval;
        *self
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut threads: Vec<_> = Vec::new();
        let limit = Duration::from_secs(self.timeout);
        for server in self.servers.iter() {
            let handle = spawn(async move {
                loop {
                    let (start, interval, stream) = (
                        Instant::now(),
                        Duration::from_millis(self.interval * 1000),
                        TcpStream::connect(format!("{}:{}", server.ip, server.health_check.port)),
                    );
                    // println!(
                    //     "[AYA] Sending: [{}:{}] -> [{}:{}].",
                    //     ARGS.ip, ARGS.port, server.ip, ARGS.health_check.port
                    // );
                    match timeout(limit, stream).await.unwrap() {
                        Ok(ref stream) => {
                            // println!(
                            //     "[IAA] Response received: [{}:{}] -> [{}:{}].",
                            //     ARGS.ip, ARGS.port, server.ip, ARGS.health_check.port
                            // );
                            if let Err(e) = stream.shutdown(Shutdown::Both) {
                                eprintln!("Error shutting down stream: {}", e);
                            }
                        }
                        Err(ref e) if e.kind() == ErrorKind::TimedOut => {
                            eprintln!(
                                "[IAA] Timed out: [{}:{}] -> [{}:{}]",
                                self.ip, self.port, server.ip, server.health_check.port
                            );
                        }
                        Err(ref e) => {
                            eprintln!(
                                "[IAA] Failure due to '{}': [{}:{}] -> [{}:{}]",
                                e, self.ip, self.port, server.ip, server.health_check.port
                            );
                        }
                    };
                    let elapsed = start.elapsed();
                    if elapsed < interval {
                        delay_for(interval - elapsed).await;
                    }
                }
            });
            threads.push(handle);
        }
        futures::future::join_all(threads).await;
        Ok(())
    }
}
