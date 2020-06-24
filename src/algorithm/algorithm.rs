use crate::config::Server;
use crate::algorithm::round_robin::RoundRobin;

pub trait Algorithm {
	fn select(&self) -> Result<Server, Box<dyn std::error::Error>>;
	// fn build(name: String, servers: Vec<Server>) -> Box<dyn Algorithm> {
	// 	RoundRobin::new(servers)
	// }
}

pub fn build(name: String, servers: Vec<Server>) -> Box<dyn Algorithm> {
	Box::new(
		match name.to_lowercase().as_str() {
			"roundrobin" => RoundRobin{ servers },
			_ => RoundRobin{ servers }
		}
	)
}
