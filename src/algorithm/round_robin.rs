use crate::algorithm::algorithm::Algorithm;

#[derive(Default, Debug)]
pub struct RoundRobin {
    pub current_server: usize,
    pub len: usize,
}

impl Algorithm for RoundRobin {
    fn server(&mut self) -> usize {
        let i = self.current_server;
        self.current_server = (self.current_server + 1) % self.len;
        i
    }
}
