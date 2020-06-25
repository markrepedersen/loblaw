use crate::config::{
	Server,
	Config
};
use crate::algorithm::round_robin::RoundRobin;

pub trait Algorithm {
	fn select(&mut self) -> Result<&Server, Box<dyn std::error::Error>>;
}

pub fn build(name: String, config: &'static Config) -> Box<dyn Algorithm> {
	Box::new(
		match name.to_lowercase().as_str() {
			"roundrobin" | _ => RoundRobin{ config, current_server: 0 }
		}
	)
}
