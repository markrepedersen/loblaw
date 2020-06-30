use {
    crate::algorithm::{random::Random, round_robin::RoundRobin},
    strum_macros::EnumString,
};

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

pub trait Algorithm {
    fn server(&mut self) -> usize;
}

impl Algorithm for Strategy {
    fn server(&mut self) -> usize {
        match *self {
            Strategy::RoundRobin(ref mut strategy) => strategy.server(),
            Strategy::Random(ref mut strategy) => strategy.server(),
            _ => unimplemented!(),
        }
    }
}
