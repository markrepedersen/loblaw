use crate::config::{BackendConfig, ServerStatus};
use {
    crate::{algorithm::algorithm::Algorithm, config::Config},
    hyper::{Body, Request},
    rand::Rng,
    serde::Deserialize,
};

#[derive(Default, Debug, Deserialize, Clone)]
pub struct Random {
    pub servers: Vec<BackendConfig>,
}

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

    fn server(&mut self, _: &Request<Body>) -> &BackendConfig {
        let i = rand::thread_rng().gen_range(0, self.servers.len());
        self.servers.get(i).unwrap()
    }
}
