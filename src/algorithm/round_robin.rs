use crate::config::{BackendConfig, ServerStatus};
use {
    crate::{algorithm::algorithm::Algorithm, config::Config},
    hyper::{Body, Request},
    serde::Deserialize,
};

#[derive(Default, Debug, Deserialize, Clone)]
pub struct RoundRobin {
    pub current_server: usize,
    pub servers: Vec<BackendConfig>,
}

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

    fn server(&mut self, _: &Request<Body>) -> &BackendConfig {
        let (i, len) = (self.current_server, self.servers.len());
        self.current_server = (self.current_server + 1) % len;
        self.servers.get(i).unwrap()
    }
}
