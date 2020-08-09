use {
    crate::{
        algorithm::algorithm::{Algorithm, RequestInfo},
        config::{BackendConfig, Config, ServerStatus},
    },
    async_trait::async_trait,
    futures::{future::ready, stream, StreamExt},
    serde::Deserialize,
    tokio::net::TcpStream,
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
        let buf = stream::iter(self.servers.clone())
            .map(|server| async move {
                TcpStream::connect(format!("{}:{}", server.ip, server.port))
                    .await
                    .is_ok()
            })
            .buffer_unordered(self.servers.len())
            .enumerate()
            .skip_while(|(_, res)| ready(!*res))
            .next()
            .await;

        buf.and_then(move |(i, _)| self.servers.get(i).map(ToOwned::to_owned))
    }
}
