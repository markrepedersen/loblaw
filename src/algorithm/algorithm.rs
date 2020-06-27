use {
    crate::{
		algorithm::{
			round_robin::RoundRobin,
			random::Random
		},
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
	match strategy {
		Strategy::Random => Box::new(Random { config }),
		Strategy::RoundRobin | _ => Box::new(RoundRobin {
			config,
			current_server: 0
		}),
	}
}
