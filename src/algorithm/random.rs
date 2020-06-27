use crate::{
    algorithm::algorithm::Algorithm,
    config::{Config, Server},
};
use rand::Rng;

pub struct Random {
    pub config: &'static Config
}

impl Algorithm for Random {
    fn select(&mut self) -> Result<&Server, Box<dyn std::error::Error>> {
        let i = rand::thread_rng().gen_range(0, self.config.servers.len());
        match self.config.servers.get(i) {
            Some(s) => Ok(s),
            None => panic!(),
        }
    }
}
