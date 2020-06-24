use crate::config::Server;
use crate::algorithm::algorithm::Algorithm;


pub struct RoundRobin {
	pub servers: Vec<Server>
}

impl Algorithm for RoundRobin {
	fn select(&self) -> Result<Server, Box<dyn std::error::Error>> {
		panic!();
	}
}
