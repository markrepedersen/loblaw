use {
    crate::{
        algorithm::algorithm::{Algorithm, RequestInfo},
        config::{BackendConfig, Config, ServerStatus},
    },
    async_trait::async_trait,
    serde::Deserialize,
};

#[derive(Default, Debug, Deserialize, Clone)]
pub struct RoundRobin {
    pub current_server: usize,
    pub servers: Vec<BackendConfig>,
}

#[async_trait]
impl Algorithm for RoundRobin {
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
        let (i, len) = (self.current_server, self.servers.len());
        self.current_server = (self.current_server + 1) % len;
        self.servers.get(i).map(ToOwned::to_owned)
    }
}
