use crate::{algorithm::algorithm::Algorithm, config::Config, status::*};
use hyper::{Body, Request};

#[derive(Default, Debug)]
pub struct RoundRobin {
    pub current_server: usize,
    pub servers: Vec<Server>,
}

impl Algorithm for RoundRobin {
    fn configure(&mut self, config: &Config) {
        for (_, backend) in config.backends.iter() {
            self.servers.push(Server {
                status: ServerStatus::Alive,
                ip: backend.ip.clone(),
                port: backend.port.clone(),
                path: backend.path.clone(),
                num_connections: 0,
            })
        }
    }

    fn server(&mut self, _: &Request<Body>) -> &Server {
        let (i, len) = (self.current_server, self.servers.len());
        self.current_server = (self.current_server + 1) % len;
        self.servers.get(i).unwrap()
    }
}
