use crate::algorithm::algorithm::Strategy;

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
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
    #[inline]
    #[allow(dead_code)]
    pub fn ip(&self) -> &String {
        &self.ip
    }

    #[inline]
    #[allow(dead_code)]
    pub fn ip_mut(&mut self) -> &mut String {
        &mut self.ip
    }

    #[inline]
    #[allow(dead_code)]
    pub fn port(&self) -> &String {
        &self.port
    }

    #[inline]
    #[allow(dead_code)]
    pub fn port_mut(&mut self) -> &mut String {
        &mut self.port
    }

    #[inline]
    #[allow(dead_code)]
    pub fn strategy(&self) -> &Strategy {
        &self.strategy
    }

    #[inline]
    #[allow(dead_code)]
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

impl Server {
    #[inline]
    #[allow(dead_code)]
    pub fn status(&self) -> &ServerStatus {
        &self.status
    }

    #[inline]
    #[allow(dead_code)]
    pub fn status_mut(&mut self) -> &mut ServerStatus {
        &mut self.status
    }

    #[inline]
    #[allow(dead_code)]
    pub fn ip(&self) -> &String {
        &self.ip
    }

    #[inline]
    #[allow(dead_code)]
    pub fn ip_mut(&mut self) -> &mut String {
        &mut self.ip
    }

    #[inline]
    #[allow(dead_code)]
    pub fn port(&self) -> &String {
        &self.port
    }

    #[inline]
    #[allow(dead_code)]
    pub fn port_mut(&mut self) -> &mut String {
        &mut self.port
    }

    #[inline]
    #[allow(dead_code)]
    pub fn path(&self) -> &String {
        &self.path
    }

    #[inline]
    #[allow(dead_code)]
    pub fn path_mut(&mut self) -> &mut String {
        &mut self.path
    }

    #[inline]
    #[allow(dead_code)]
    pub fn num_connections(&self) -> &u64 {
        &self.num_connections
    }

    #[inline]
    #[allow(dead_code)]
    pub fn num_connections_mut(&mut self) -> &mut u64 {
        &mut self.num_connections
    }
}
