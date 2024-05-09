use std::cell::Cell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
use chrono::Utc;
const TIMESTAMP_TOLERANCE : i64 = 600; //tolerancia em segundos
const REWARD: u64 = 1000; //numero de moedas de recompensa

// A struct for the Blockchain
#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pool: Vec<Transaction>,
    unspent_outputs: HashMap<String, Vec<Rc<Output>>>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(Vec::new(), String::new());
        Blockchain {
            chain: vec![genesis_block],
            pool: Vec::new(),
            unspent_outputs: HashMap::new(),
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

        //adicionar a pool
        self.pool.push(transaction);

        //adicionar outputs ao hashmap
        // Iterate over the outputs in the last transaction added to the pool
        let last_idx = self.pool.len() - 1;
        for output in self.pool[last_idx].outputs.iter() {
            self.add_to_hashmap(Rc::new(*output));
        }

        Ok(true)
    }

    //adiciona output ao hashmap
    fn add_to_hashmap(&mut self, output: Rc<Output>) {
        if output.spent.get() {
            return;
        }
    
        let wallet_address = output.receiver.clone();

        match self.unspent_outputs.entry(wallet_address){
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                //carteira existe, adicionar ao vetor
                entry.get_mut().push(output.clone());
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                let vec = vec![output.clone()];
                entry.insert(vec);
            }
        }
    }

    //Rotina para verificar a integridade do hashmap
    fn check_unspent_map_integrity(&mut self) -> Result<(), &'static str> {
        //verificar se todos os outputs do mapa nao estao gastos
        for (_, outputs) in &self.unspent_outputs {
            for output in outputs {
                if output.spent.get() {
                    return Err("Found spent outputs on hashmap")
                }
            }
        }
        Ok(())
    }

    //tornar todos os outputs utilizados como input em uma transacao em gastos
    pub fn spend_inputs(&mut self, transaction: &Transaction) -> bool {
        for input in &transaction.inputs {
            if let Some(outputs) = self.unspent_outputs.get_mut(&input.receiver) {
                if let Some(output) = outputs.iter_mut().find(|o| *o.receiver == input.receiver && !o.spent.get()) {
                    output.spent.set(true);
                } else {
                    return false; // Input nao encontrado ou ja gasto
                }
            } else {
                return false; // input nao encontrado no hashmap
            }
        }
        true
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