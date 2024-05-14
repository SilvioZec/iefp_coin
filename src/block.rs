use chrono::Utc;
use sha2::{Sha256, Digest};
use std::fmt::Write;
use super::*;

const DIFFICULTY: &str = "000";
const MIN_TRANSACTIONS : usize = 1;

// Block struct

#[derive(Debug, Clone)]
pub struct Block{
    pub timestamp:i64,
    pub data:Vec<Transaction>,
    pub prev_hash:String,
    pub hash:String,
    nonce: u64,
}

// Block functions
impl Block {
    //constructor
    pub fn new(data: Vec<Transaction>, prev_hash: String) -> Self {
        let block = Block {
            timestamp: 0,
            data,
            prev_hash,
            hash: String::new(),
            nonce: 0,
        };
        block
    }

    //cria a hash de um bloco
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        //itera sobre as transacoes
        for transaction in &self.data{
            //iterate sobre as saidas e as converte em bytes
            for output in &transaction.outputs{
                hasher.update(output.sender.as_bytes());
                hasher.update(output.receiver.as_bytes());
                hasher.update(output.amount.to_ne_bytes());
                hasher.update(output.signature.as_bytes());
            }
        }

        hasher.update(&self.prev_hash.as_bytes());
        hasher.update(&self.nonce.to_string().as_bytes());
        //calcular a hash final de 256 bytes
        let hash = hasher.finalize();
        let mut hash_str = String::new();
        //paracada byte da hash, converte para hexadecimal e adiciona a string
        for byte in hash {
            write!(&mut hash_str, "{:02x}", byte).expect("Unable to write");
        }
        hash_str
    }

    //block miner
    pub fn mine (&mut self, pool: Vec<Transaction>) {
        self.data = pool;
        //verificar se existe o minimo de transacoes necessarias
        if self.data.len() < MIN_TRANSACTIONS{
            return;
        }

        //aumenta o nonce ate satisfazer a funcao de checar dificuldade
        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;
            let hash= self.calculate_hash();
            if Block::check_difficulty(&hash) {
                self.hash = hash;
                self.timestamp = Utc::now().timestamp();
                return;
            }
        }
    }
    
    pub fn validate_block (block: &Block) -> bool {
        //cria um hash para o bloco e compara com o hash atribuido
        let hash = Block::calculate_hash(block);
        if hash != block.hash {
            return false;
        }

        //verifica se o hash satisfaz a dificuldade
        if  !Self::check_difficulty(&block.hash){
            return false;
        }
        
        //valida as transacoes
        for individual_transaction in &block.data{
            if !Transaction::validate_transaction(individual_transaction){
                return false;
            }
        }
        true
    }

    pub fn check_difficulty(hash: &String) -> bool {
    hash.starts_with(DIFFICULTY)
    }
}

