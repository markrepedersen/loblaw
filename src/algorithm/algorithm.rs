use crate::config::Server;
use crate::algorithm::round_robin::RoundRobin;

pub enum AlgoType {
	RoundRobin(RoundRobin),
}

pub fn build(name: String, servers: Vec<Server>) -> AlgoType {
	AlgoType::RoundRobin(RoundRobin {servers})
}

pub trait Algorithm {
	fn new(servers: Vec<Server>) -> Self;
	fn select() -> Result<Server, Box<dyn std::error::Error>>;
}

