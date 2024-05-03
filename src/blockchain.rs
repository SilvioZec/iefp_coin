use std::collections::HashMap;

use super::*;
use chrono::Utc;
const TIMESTAMP_TOLERANCE : i64 = 600; //tolerancia em segundos

// A struct for the Blockchain
#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pool: Vec<Transaction>,
    unspent_transactions: HashMap<String, Output>,
}
// Implementing the Blockchain
impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(Vec::new(), String::new());
        Blockchain {
            chain: vec![genesis_block],
            pool: Vec::new(),
            unspent_transactions: HashMap::new(),
        }
    }

    //adicionar transacao a pool
    pub fn add_to_pool(transaction: Transaction) -> Result<bool, String>{

        //validar a transacao
        if !Transaction::validate_transaction(&transaction){
            return Err(String::from("Invalid transaction. Not added"));
        }
        //verificar se o sender e o receiver sao validos
        //verificar se a transacao ja existe na pool
        //verificar se o sender tem saldo suficiente
        //gastar os outputs utilizados na transacao
        //adicionar a pool
        pool.push(transaction);
        Ok(true)
    }

    //rotina para verificar integridade do hashmap
    fn check_unspent_map_integrity() {
        //percorrer blockchain e verificar hashmap
    }

    //tornar todos os outputs utilizados como input em uma transacao em gastos
    fn spend_inputs(transaction: &Transaction) -> bool{
        let mut all_unspent_outputs: Vec<&mut Output> = vec![];
        true
    }
    //adicionar bloco a blockchain
    pub fn add_block(&mut self, data: Vec<Transaction>) {
        let prev_hash = self.chain.last().unwrap().hash.clone();
        let new_block = Block::new(data, prev_hash);
        self.chain.push(new_block);
    }

    //validar bloco
    fn validate_block (&block &Block) -> bool {
        //verificar a prev_hash
        if block.prev_hash != self.chain.last().unwrap().hash {
            return false;
        }
        //verificar o bloco atraves da funcao da classe
        if !Block::validate_block(block){
            return false;
        }
        //verificar o timestamp
        if block.timestamp > Utc::now().timestamp() || block.timestamp < Utc::now().timestamp() - TIMESTAMP_TOLERANCE {
            return false;
        }
    }
}