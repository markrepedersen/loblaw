use {
    crate::{algorithm::algorithm::Algorithm, config::*},
    hyper::{Body, Request},
    serde::Deserialize,
    std::collections::HashMap,
};

/// Maps the given request to a server using the URL's path as a directive.
#[derive(Default, Debug, Deserialize, Clone)]
pub struct IPHash {
    ip_mappings: HashMap<String, BackendConfig>,
}

impl Algorithm for IPHash {
    fn configure(&mut self, config: &Config) {
        for (name, mapping) in config.mappings.iter() {
            let path = mapping.path.clone();
            if let Some(backend) = config.backends.get(name) {
                self.ip_mappings.insert(
                    path,
                    BackendConfig {
                        status: ServerStatus::Alive,
                        ip: backend.ip().clone(),
                        port: backend.port().clone(),
                        scheme: backend.scheme().clone(),
                        path: backend.path().clone(),
                        num_connections: backend.num_connections().clone(),
                    },
                );
            }
        }
    }

    fn server(&mut self, req: &Request<Body>) -> &BackendConfig {
        let host = req.uri().host().unwrap();
        self.ip_mappings.get(host).unwrap()
    }
}
