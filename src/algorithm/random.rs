use crate::{algorithm::algorithm::Algorithm, config::Config, status::*};
use hyper::{Body, Request};
use rand::Rng;

#[derive(Default, Debug)]
pub struct Random {
    pub servers: Vec<Server>,
}

impl Algorithm for Random {
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
        let i = rand::thread_rng().gen_range(0, self.servers.len());
        self.servers.get(i).unwrap()
    }
}
