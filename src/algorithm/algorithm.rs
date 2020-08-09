use {
    crate::{
        algorithm::{
            ip_hash::IPHash, random::Random, round_robin::RoundRobin, url_hash::UriPathHash,
        },
        config::{BackendConfig, Config},
    },
    actix_web::{dev::ConnectionInfo, http::Uri},
    async_trait::async_trait,
    serde::Deserialize,
    std::fmt,
    strum_macros::EnumString,
    actix::prelude::*,
};

#[derive(Debug, Clone)]
pub struct ServerSelectionError;

impl fmt::Display for ServerSelectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "There was an internal error in choosing an appropriate server for this request."
        )
    }
}

pub struct RequestInfo {
    uri: Uri,
    connection_info: ConnectionInfo,
}

impl RequestInfo {
    pub fn new(uri: Uri, connection_info: ConnectionInfo) -> Self {
        Self {
            uri,
            connection_info,
        }
    }

    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    pub fn connection_info(&self) -> &ConnectionInfo {
        &self.connection_info
    }
}

/// A user specified dynamic strategy for forwarding requests to a given server.
/// Requests will be fed to a strategy and a server with an (ip, port, path) triplet will be output.
#[derive(EnumString, Deserialize, Debug, Clone)]
pub enum Strategy {
    RoundRobin(RoundRobin),
    WeightedRoundRobin(RoundRobin),
    Random(Random),
    LeastConnections(RoundRobin),
    WeightedLeastConnections(RoundRobin),
    UriPathHash(UriPathHash),
    SourceIPHash(IPHash),
    LeastTraffic(RoundRobin),
    LeastLatency(RoundRobin),
}

impl Actor for Strategy {
    type Context = Context<Self>;
}

#[async_trait]
pub trait Algorithm {
    /// Configuration for strategies such as initializing servers.
    fn configure(&mut self, config: &Config);

    /// Determines the server to which the given request should be forwarded.
    async fn server(&mut self, req: &RequestInfo) -> Option<BackendConfig>;
}

#[async_trait]
impl Algorithm for Strategy {
    /// Configuration for strategies such as initializing servers.
    fn configure(&mut self, config: &Config) {
        match *self {
            Strategy::RoundRobin(ref mut strategy) => strategy.configure(config),
            Strategy::WeightedRoundRobin(ref mut strategy) => strategy.configure(config),
            Strategy::Random(ref mut strategy) => strategy.configure(config),
            Strategy::SourceIPHash(ref mut strategy) => strategy.configure(config),
            Strategy::UriPathHash(ref mut strategy) => strategy.configure(config),
            Strategy::LeastLatency(ref mut strategy) => strategy.configure(config),
            Strategy::LeastTraffic(ref mut strategy) => strategy.configure(config),
            Strategy::LeastConnections(ref mut strategy) => strategy.configure(config),
            Strategy::WeightedLeastConnections(ref mut strategy) => strategy.configure(config),
        };
    }

    /// Determines the server to which the given request should be forwarded.
    async fn server(&mut self, req: &RequestInfo) -> Option<BackendConfig> {
        match *self {
            Strategy::RoundRobin(ref mut strategy) => strategy.server(req).await,
            Strategy::WeightedRoundRobin(ref mut strategy) => strategy.server(req).await,
            Strategy::Random(ref mut strategy) => strategy.server(req).await,
            Strategy::SourceIPHash(ref mut strategy) => strategy.server(req).await,
            Strategy::UriPathHash(ref mut strategy) => strategy.server(req).await,
            Strategy::LeastLatency(ref mut strategy) => strategy.server(req).await,
            Strategy::LeastTraffic(ref mut strategy) => strategy.server(req).await,
            Strategy::LeastConnections(ref mut strategy) => strategy.server(req).await,
            Strategy::WeightedLeastConnections(ref mut strategy) => strategy.server(req).await,
        }
    }
}
