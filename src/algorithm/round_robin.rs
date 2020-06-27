use crate::{algorithm::algorithm::Algorithm, config::Server, CONFIG};

pub struct RoundRobin {
    pub current_server: usize,
}

unsafe impl Send for RoundRobin {}
unsafe impl Sync for RoundRobin {}

impl Algorithm for RoundRobin {
    fn select(&mut self) -> Result<&Server, Box<dyn std::error::Error>> {
        let i = self.current_server;
        self.current_server = (self.current_server + 1) % CONFIG.servers.len();
        match CONFIG.servers.get(i) {
            Some(s) => Ok(s),
            None => panic!(),
        }
    }
}
