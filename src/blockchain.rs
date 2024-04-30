use super::*;
use chrono::Utc;

// A struct for the Blockchain
#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pool: Vec<Transaction>,
}
// Implementing the Blockchain
impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, Vec::new(), String::new());
        Blockchain {
            chain: vec![genesis_block],
            pool: Vec::new(),
        }
    }
    pub fn add_block(&mut self, data: Vec<Transaction>) {
        let prev_hash = self.chain.last().unwrap().hash.clone();
        let new_block = Block::new(Self::current_timestamp(), data, prev_hash);
        self.chain.push(new_block);
    }
    fn current_timestamp() -> i64 {
        let dt = Utc::now().timestamp();
        dt
    }
}