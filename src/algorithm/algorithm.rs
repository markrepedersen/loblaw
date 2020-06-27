use std::sync::Arc;
use {
    crate::{
        algorithm::{random::Random, round_robin::RoundRobin},
        config::Server,
    },
    strum_macros::EnumString,
};

#[derive(EnumString)]
pub enum Strategy {
    RoundRobin,
    WeightedRoundRobin,
    Random,
    LeastConnections,
    WeightedLeastConnections,
    URLHash,
    SourceIPHash,
    LeastTraffic,
    LeastLatency,
}

pub trait Algorithm {
    fn select(&mut self) -> Result<&Server, Box<dyn std::error::Error>>;
}

pub fn build(strategy: Strategy) -> Arc<dyn Algorithm + Send + Sync> {
    match strategy {
        Strategy::Random => Arc::new(Random {}),
        Strategy::RoundRobin | _ => Arc::new(RoundRobin { current_server: 0 }),
    }
}
