use crate::{algorithm::algorithm::Algorithm, config::Server, CONFIG};
use rand::Rng;

pub struct Random;
unsafe impl Send for Random {}
unsafe impl Sync for Random {}

impl Algorithm for Random {
    fn select(&mut self) -> Result<&Server, Box<dyn std::error::Error>> {
        let i = rand::thread_rng().gen_range(0, CONFIG.servers.len());
        match CONFIG.servers.get(i) {
            Some(s) => Ok(s),
            None => panic!(),
        }
    }
}
