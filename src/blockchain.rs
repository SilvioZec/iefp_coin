use super::*;
use chrono::Utc;

// A struct for the Blockchain
#[derive(Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
}
// Implementing the Blockchain
impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block".to_owned(), String::new());
        Blockchain {
            chain: vec![genesis_block],
        }
    }
    pub fn add_block(&mut self, data: String) {
        let prev_hash = self.chain.last().unwrap().hash.clone();
        let new_block = Block::new(Self::current_timestamp(), data, prev_hash);
        self.chain.push(new_block);
    }
    fn current_timestamp() -> i64 {
        let dt = Utc::now().timestamp();
        dt
    }
}