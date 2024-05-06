use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use super::*;
use chrono::Utc;
const TIMESTAMP_TOLERANCE : i64 = 600; //tolerancia em segundos
const REWARD: u64 = 1000; //numero de moedas de recompensa

// A struct for the Blockchain
#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pool: Vec<Transaction>,
    unspent_transactions: HashMap<String, Rc<RefCell<Output>>>,
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
    pub fn add_to_pool(&mut self, transaction: Transaction) -> Result<bool, String>{

        //validar a transacao
        if !Transaction::validate_transaction(&transaction){
            return Err(String::from("Invalid transaction. Not added"));
        }

        // Gastar os outputs utilizados na transacao
        if self.spend_inputs(&transaction){
            return Err(String::from("Could not spend the inputs"));
        }
        //adicionar outputs ao hashmap
        for output in &transaction.outputs {
            self.unspent_transactions.insert(output.borrow().sender.clone(), output.clone());
        }

        //adicionar a pool
        self.pool.push(transaction);
        Ok(true)
    }

    //rotina para verificar integridade do hashmap
    fn check_unspent_map_integrity(&mut self) -> Result<(), &'static str> {
        // Verifica se todos os outputs no HashMap estão marcados como spent = false
        let mut is_integral = true;
        for (_, output) in &self.unspent_transactions {
            if output.borrow().spent {
                is_integral = false;
                break;
            }
        }
    
        // Refaz o hashmap
        let mut new_hashmap = HashMap::new();
        for block in &self.chain {
            for transaction in &block.data {
                for output in &transaction.outputs {
                    if !output.borrow().spent {
                        new_hashmap.insert(output.borrow().sender.clone(), output.clone());
                    }
                }
            }
        }
        for transaction in &self.pool{
            for output in &transaction.outputs{
                if !output.borrow().spent{
                    new_hashmap.insert(output.borrow().sender.clone(), output.clone());
                }
            }
        }
    
        // Compara o hashmap refeito com o existente
        if !is_integral || new_hashmap != self.unspent_transactions {
            self.unspent_transactions = new_hashmap;
            return Err("HashMap inconsistente. Anterior substituído.");
        }
    
        Ok(())
    }

    //tornar todos os outputs utilizados como input em uma transacao em gastos
    fn spend_inputs(&mut self, transaction: &Transaction) -> bool{
        for input in &transaction.input {
            if let Some(output_ref) = self.unspent_transactions.get_mut(&input.sender) {
                output_ref.borrow_mut().spent = true;
            } else {
                return false;
            }
        }
        true
    }
    //adicionar bloco a blockchain ============= MODIFICAR
    pub fn add_block(&mut self, mined_block: Block, miner_wallet: &String) -> Result<(), &'static str> {
        //extrair hash do ultimo bloco
        let prev_hash = self.chain.last().unwrap().hash.clone();

        //criar bloco a ser adicionado
        let new_block = mined_block;

        //validar bloco
        if !self.validate_block(new_block){
            return Err("Invalid Block")
        }

        //verificar se todas as trasacoes do bloco existem na pool
        for transaction in &data {
            if !self.pool.iter().any(|t| *t == *transaction) {
                return Err("Transaction not found in the pool");
            }
        }

        //remover as transacoes do bloco na pool
        for transaction in &new_block.data {
            if let Some(pos) = self.pool.iter().position(|t| *t == *transaction) {
                self.pool.remove(pos);
            }
        }
        
        // recompensar o minerador com um output
        let reward_output = Rc::new(RefCell::new(Output {
            sender: String::from("System"),
            receiver: miner_wallet.to_string(),
            amount: REWARD,
            spent: false,
        }));

        //add reward transaction to the block
        let reward_transaction = Transaction {
            input: Vec::new(),
            outputs: vec![reward_output.clone()],
        };

        //adicionar transacao de recompensa ao bloco
        new_block.data.push(reward_transaction);

        //adicionar bloco a chain
        self.chain.push(new_block);
        
    }

    //validar bloco
    fn validate_block(&block &Block) -> bool {
        //verificar a prev_hash
        if block.prev_hash != self.chain.last().unwrap().hash {
            return false;
        }
        //verificar o bloco atraves da funcao da classe
        if !Block::validate_block(&block){
            return false;
        }
        //verificar o timestamp
        if block.timestamp > Utc::now().timestamp() || block.timestamp < Utc::now().timestamp() - TIMESTAMP_TOLERANCE {
            return false;
        }
    }
}