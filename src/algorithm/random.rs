use {
    crate::{
        algorithm::algorithm::{Algorithm, RequestInfo},
        config::{BackendConfig, Config, ServerStatus},
    },
    async_trait::async_trait,
    rand::Rng,
    serde::Deserialize,
};

#[derive(Default, Debug, Deserialize, Clone)]
pub struct Random {
    pub servers: Vec<BackendConfig>,
}

#[async_trait]
impl Algorithm for Random {
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
        let i = rand::thread_rng().gen_range(0, self.servers.len());
        self.servers.get(i).map(ToOwned::to_owned)
    }
}
