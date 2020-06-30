use crate::{algorithm::algorithm::Algorithm, config::Server, CONFIG};

#[derive(Default)]
pub struct RoundRobin {
    pub current_server: usize,
}

impl Algorithm for RoundRobin {
    fn server(&mut self) -> usize {
        let i = self.current_server;
        self.current_server = (self.current_server + 1) % CONFIG.servers.len();
        i
    }
}
