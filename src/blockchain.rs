use std::cell::Cell;
use secp256k1::SecretKey;
use super::*;
use chrono::Utc;
const TIMESTAMP_TOLERANCE : i64 = 600; //tolerancia em segundos
const REWARD: u64 = 1000; //numero de moedas de recompensa

// A struct for the Blockchain
#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pool: Vec<Transaction>,
}

impl Blockchain {
    pub fn new(wallet_address: String, private_key: &SecretKey) -> Self {
        let genesis_input = vec![Output::new(String::from("System"), wallet_address.clone(), 1000, private_key)];
        let genesis_output = vec![Output::new(String::from("System"), wallet_address.clone(), 1000, private_key)];
        let genesis_transaction = Transaction::new(genesis_input, genesis_output);
        let genesis_block = Block::new(vec![genesis_transaction], String::new());
        Blockchain {
            chain: vec![genesis_block],
            pool: Vec::new(),
        }
    }

    //adicionar transacao a pool
    pub fn add_to_pool(&mut self, transaction: Transaction) -> Result<bool, String>{

        //validar a transacao
        if !Transaction::validate_transaction(&transaction){
            return Err(String::from("Invalid transaction. Not added"));
        }

        // Gastar os outputs utilizados na transacao
        for input in &transaction.inputs{
            if !self.spend_input(input){
                    return Err(String::from("Could not spend the inputs"));
                }
        }
        

        //adicionar a pool
        self.pool.push(transaction);

        Ok(true)
    }


    //tornar todos os outputs utilizados como input em uma transacao em gastos
    pub fn spend_input(&mut self, input: &Output) -> bool {
        for block in &self.chain{
            for transaction in &block.data{
                for output in &transaction.outputs {
                    if output.signature == input.signature && !output.spent.get(){
                        output.spent.set(true);
                        return true;
                    }
                }
            }
        }
        false
    }

    //adicionar bloco a blockchain ============= MODIFICAR
    pub fn add_block(&mut self, mut mined_block: Block, miner_wallet: String) -> Result<(), &'static str> {
        //extrair hash do ultimo bloco
        let prev_hash = self.chain.last().unwrap().hash.clone();

        if mined_block.prev_hash != prev_hash{
            return Err("Previous hash incompatible")
        }

        //validar bloco
        if !blockchain::Blockchain::validate_block(&self,&mined_block){
            return Err("Invalid Block")
        }

        //verificar se todas as trasacoes do bloco existem na pool
        for transaction in &mined_block.data {
            if !self.pool.iter().any(|t| *t == *transaction) {
                return Err("Transaction not found in the pool");
            }
        }

        //remover as transacoes do bloco na pool
        self.pool.retain(|transaction| !mined_block.data.iter().any(|t| *t == *transaction));
        
        // recompensar o minerador com um output
        let reward_output = Output {
            sender: String::from("System"),
            receiver: miner_wallet.to_string(),
            amount: REWARD,
            signature: String::new(),
            spent: Cell::new(false),
        };

        //adicionar recompensa as transacoes
        let reward_transaction = Transaction::new(Vec::new(), vec![reward_output]);

        //adicionar transacao de recompensa ao bloco
        mined_block.data.push(reward_transaction);

        //adicionar bloco a chain
        self.chain.push(mined_block);
        Ok(())
        
    }

    //validar bloco
    fn validate_block(&self, block: &Block) -> bool {
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
        true
    }
}