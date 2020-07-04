use hyper::{Body, Request};
use {
    crate::{
        algorithm::{random::Random, round_robin::RoundRobin},
        config::Config,
        status::Server,
    },
    std::str::FromStr,
    strum_macros::EnumString,
};

/// A user specified dynamic strategy for forwarding requests to a given server.
/// Requests will be fed to a strategy and a server with an (ip, port, path) triplet will be output.
#[derive(EnumString, Debug)]
pub enum Strategy {
    RoundRobin(RoundRobin),
    WeightedRoundRobin(RoundRobin),
    Random(Random),
    LeastConnections(RoundRobin),
    WeightedLeastConnections(RoundRobin),
    URLHash(RoundRobin),
    SourceIPHash(RoundRobin),
    LeastTraffic(RoundRobin),
    LeastLatency(RoundRobin),
}

impl Strategy {
    pub fn new(config: &Config) -> Self {
        let mut strategy = Strategy::from_str(config.strategy.as_str()).unwrap();
        strategy.configure(config);
        strategy
    }
}

pub trait Algorithm {
    /// Configuration for strategies such as initializing servers.
    fn configure(&mut self, config: &Config);

    /// Determines the server to which the given request should be forwarded.
    fn server(&mut self, req: &Request<Body>) -> &Server;
}

impl Algorithm for Strategy {
    /// Configuration for strategies such as initializing servers.
    fn configure(&mut self, config: &Config) {
        match *self {
            Strategy::RoundRobin(ref mut strategy) => strategy.configure(config),
            Strategy::Random(ref mut strategy) => strategy.configure(config),
            _ => unimplemented!(),
        };
    }

    /// Determines the server to which the given request should be forwarded.
    fn server(&mut self, req: &Request<Body>) -> &Server {
        match *self {
            Strategy::RoundRobin(ref mut strategy) => strategy.server(req),
            Strategy::Random(ref mut strategy) => strategy.server(req),
            _ => unimplemented!(),
        }
    }
}
