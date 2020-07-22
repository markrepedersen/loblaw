use {
    crate::{config::Config, Threadable, with_read_lock},
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

pub async fn run(config: Threadable<Config>) -> Result<(), Box<dyn std::error::Error>> {
    let (limit, interval, servers) = with_read_lock(config, |config| {
        (
            Duration::from_secs(config.health_check.timeout),
            config.health_check.interval,
            config.backends.clone(),
        )	
    });

    for (_, server) in servers.into_iter() {
        spawn(async move {
            loop {
                let (start, interval, stream) = (
                    Instant::now(),
                    Duration::from_millis(interval * 1000),
                    TcpStream::connect(format!("{}:{}", server.ip, server.port)),
                );
                match timeout(limit, stream).await.unwrap() {
                    Ok(ref stream) => {
                        if let Err(e) = stream.shutdown(Shutdown::Both) {
                            eprintln!("Error shutting down stream: {}", e);
                        }
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
