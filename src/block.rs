use sha2::{Sha256, Digest};
use std::{fmt::Write, sync::mpsc::Sender};


use crate::{transaction, Transaction};

#[derive(Debug)]
pub struct Block{
    timestamp:i64,
    data:Vec<Transaction>,
    prev_hash:String,
    pub hash:String,
    nonce: u64,
}

// Block functions
impl Block {
    //constructor
    pub fn new(timestamp: i64, data: Vec<Transaction>, prev_hash: String) -> Self {
        let mut block = Block {
            timestamp,
            data,
            prev_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.mine();
        block
    }

    //hash creator
    fn calculate_hash(block: &Block) -> String {
        let mut hasher = Sha256::new();
        hasher.update(block.timestamp.to_string().as_bytes());
        
        //iterate through transactions
        for transaction in &block.data{
            //iterate through outputs and add to hasher
            for output in &transaction.output{
                hasher.update(output.sender.as_bytes());
                hasher.update(output.receiver.as_bytes());
                hasher.update(output.amount.to_ne_bytes());
                hasher.update(output.signature.as_bytes());
            }
        }

        hasher.update(&block.prev_hash.as_bytes());
        hasher.update(&block.nonce.to_string().as_bytes());
        let hash = hasher.finalize();
        let mut hash_str = String::new();
        for byte in hash {
            write!(&mut hash_str, "{:02x}", byte).expect("Unable to write");
        }
        hash_str
    }

    //block miner
    pub fn mine (&mut self) {
        //aumenta o nonce ate satisfazer a funcao de checar dificuldade
        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;
            let hash= Block::calculate_hash(&self);
            if Self::check_difficulty(&hash) {
                self.hash = hash;
                return;
            }
        }
    }
    
    pub fn check_difficulty(hash: &String) -> bool {

    hash.starts_with("0000") //mais facil de visualizar porem um desempenho menor

    /* 
        // Converte a string hexadecimal para bytes
    let hash_bytes = hex::decode(hash).unwrap();

    // Verifica se a hash inicia com a sequência "0000" em bytes
    let leading_zeroes = hash_bytes.iter().take_while(|&b| *b == 0).count();

    // Verifica se a quantidade de zeros iniciais na hash é igual ao comprimento da sequência "0000"
    leading_zeroes >= 2
    */
    
    }
}

