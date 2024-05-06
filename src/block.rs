use chrono::Utc;
use sha2::{Sha256, Digest};
use std::{fmt::{Write, DebugStruct}, sync::mpsc::Sender};
use super::*;

const DIFFICULTY: &str = "0000";
const MIN_TRANSACTIONS : usize = 5;

// Block struct

#[derive(Debug)]
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
        let mut block = Block {
            timestamp: 0,
            data,
            prev_hash,
            hash: String::new(),
            nonce: 0,
        };
        block
    }

    //cria a hash de um bloco
    fn calculate_hash(block: &Block) -> String {
        let mut hasher = Sha256::new();
        //itera sobre as transacoes
        for transaction in &block.data{
            //iterate sobre as saidas e as converte em bytes
            for output in &transaction.output{
                hasher.update(output.sender.as_bytes());
                hasher.update(output.receiver.as_bytes());
                hasher.update(output.amount.to_ne_bytes());
                hasher.update(output.signature.as_bytes());
            }
        }

        hasher.update(&block.prev_hash.as_bytes());
        hasher.update(&block.nonce.to_string().as_bytes());
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
            let hash= self.calculate_hash(&self);
            if check_difficulty(&hash) {
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

    hash.starts_with(DIFFICULTY) //mais facil de visualizar porem um desempenho menor

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

