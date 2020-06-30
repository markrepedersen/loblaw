use crate::algorithm::algorithm::Strategy;

#[derive(Debug, Copy, Clone)]
pub enum ServerStatus {
    Alive,
    Busy,
    Dead,
    Throttled,
}

#[derive(Debug)]
pub struct Global {
    pub strategy: Strategy,
    pub ip: String,
    pub port: String,
}

impl Global {
    pub fn strategy(&self) -> &Strategy {
        &self.strategy
    }

    pub fn strategy_mut(&mut self) -> &mut Strategy {
        &mut self.strategy
    }
}

#[derive(Debug, Clone)]
pub struct Server {
    pub status: ServerStatus,
    pub ip: String,
    pub port: String,
    pub path: String,
    pub num_connections: u64,
}
