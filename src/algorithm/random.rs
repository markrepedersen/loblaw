use crate::algorithm::algorithm::Algorithm;
use rand::Rng;

#[derive(Default, Debug)]
pub struct Random {
    len: usize,
}

impl Algorithm for Random {
    fn server(&mut self) -> usize {
        rand::thread_rng().gen_range(0, self.len)
    }
}
