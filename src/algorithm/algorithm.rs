use {
    crate::{
        algorithm::round_robin::RoundRobin,
        config::{Config, Server},
    },
    std::str::FromStr,
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

pub fn build(config: &'static Config) -> Box<dyn Algorithm> {
    let strategy = Strategy::from_str(config.strategy.unwrap().as_str()).unwrap();
    Box::new(match strategy {
        Strategy::RoundRobin => RoundRobin {
            config,
            current_server: 0,
        },
        Strategy::WeightedRoundRobin => unimplemented!(),
        Strategy::Random => unimplemented!(),
        Strategy::LeastConnections => unimplemented!(),
        Strategy::WeightedLeastConnections => unimplemented!(),
        Strategy::URLHash => unimplemented!(),
        Strategy::SourceIPHash => unimplemented!(),
        Strategy::LeastTraffic => unimplemented!(),
        Strategy::LeastLatency => unimplemented!(),
    })
}
