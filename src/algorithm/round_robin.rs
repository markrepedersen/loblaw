use crate::config::{
	Server,
	Config
};
use crate::algorithm::algorithm::Algorithm;


pub struct RoundRobin {
	pub config: &'static Config,
	pub current_server: usize
}

impl Algorithm for RoundRobin {
	fn select(&mut self) -> Result<&Server, Box<dyn std::error::Error>> {
		let i = self.current_server;
		self.current_server = (self.current_server + 1) % self.config.servers.len();
		match self.config.servers.get(i) {
			Some(s) => Ok(s),
			None => panic!()
		}
	}
}
