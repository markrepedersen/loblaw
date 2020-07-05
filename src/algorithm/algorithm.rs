use {
    crate::{
        algorithm::{random::Random, round_robin::RoundRobin},
        config::{BackendConfig, Config},
    },
    hyper::{Body, Request},
    serde::Deserialize,
    strum_macros::EnumString,
};

/// A user specified dynamic strategy for forwarding requests to a given server.
/// Requests will be fed to a strategy and a server with an (ip, port, path) triplet will be output.
#[derive(EnumString, Deserialize, Debug, Clone)]
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

pub trait Algorithm {
    /// Configuration for strategies such as initializing servers.
    fn configure(&mut self, config: &Config);

    /// Determines the server to which the given request should be forwarded.
    fn server(&mut self, req: &Request<Body>) -> &BackendConfig;
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
    fn server(&mut self, req: &Request<Body>) -> &BackendConfig {
        match *self {
            Strategy::RoundRobin(ref mut strategy) => strategy.server(req),
            Strategy::Random(ref mut strategy) => strategy.server(req),
            _ => unimplemented!(),
        }
    }
}
