use crate::{algorithm::algorithm::Algorithm, config::Server, CONFIG};
use rand::Rng;

#[derive(Default)]
pub struct Random;

impl Algorithm for Random {
    fn server(&mut self) -> usize {
        rand::thread_rng().gen_range(0, CONFIG.servers.len())
    }
}
