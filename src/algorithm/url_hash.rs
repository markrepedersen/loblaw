use crate::{algorithm::algorithm::Algorithm, config::*};
use hyper::{Body, Request};
use std::collections::HashMap;

/// Maps the given request to a server using the URL's path as a directive.
#[derive(Default, Debug)]
pub struct UrlHash {
    url_mappings: HashMap<String, BackendConfig>,
}

impl Algorithm for UrlHash {
    fn configure(&mut self, config: &Config) {
        for (name, mapping) in config.mappings.iter() {
            let path = mapping.path.clone();
            if let Some(backend) = config.backends.get(name) {
                self.url_mappings.insert(
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
        let req_path = req.uri().path();
        self.url_mappings.get(req_path).unwrap()
    }
}
