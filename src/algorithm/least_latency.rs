use {
    crate::{
        algorithm::algorithm::{Algorithm, RequestInfo},
        config::{BackendConfig, Config, ServerStatus},
    },
    async_trait::async_trait,
    futures::{stream, StreamExt},
    serde::Deserialize,
    tokio::net::TcpStream,
    actix::clock::Instant,
    futures::future::ready
};

#[derive(Default, Debug, Deserialize, Clone)]
pub struct LeastLatency {
    pub current_server: usize,
    pub servers: Vec<BackendConfig>,
}

#[async_trait]
impl Algorithm for LeastLatency {
    fn configure(&mut self, config: &Config) {
        for (_, backend) in config.backends.iter() {
            self.servers.push(BackendConfig {
                status: ServerStatus::Alive,
                ip: backend.ip().clone(),
                port: backend.port().clone(),
                scheme: backend.scheme().clone(),
                path: backend.path().clone(),
                num_connections: backend.num_connections().clone(),
            })
        }
    }

    async fn server(&mut self, _: &RequestInfo) -> Option<BackendConfig> {
        let mut times = stream::iter(self.servers.clone())
            .map(|server| async move {
                let now = Instant::now();
                TcpStream::connect(format!("{}:{}", server.ip(), server.port()))
                    .await
                    .is_ok()
                    .then_some((server, now.elapsed()))
            })
            .buffer_unordered(self.servers.len())
            .filter(|res| ready(res.is_some()))
            .map(|res| res.unwrap())
            .collect::<Vec<_>>().await;

        times.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        times.first().map(|res| res.0.clone())
    }
}
