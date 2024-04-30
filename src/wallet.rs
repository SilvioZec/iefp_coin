use super::*;
use secp256k1::{Secp256k1, Message, SecretKey, PublicKey, Signature};

#[derive(Debug)]
pub struct Wallet {
    private_key: SecretKey,
    pub public_key: String,
    balance: u64,
    utxo: Vec<Output>,
}

impl Wallet {
    fn new() -> Self {
        let private_key = SecretKey::random(&mut rand::thread_rng());
        
        let public_key = secp256k1::PublicKey::from_secret_key(&secp, &private_key);
        let public_key_str = hex::encode(public_key.serialize_uncompressed());

        Wallet {
            private_key: SecretKey,
            public_key: public_key_str,
            balance: 0,
            utxo: Vec::new(),
        }
    }

    fn calculate_balance(&self) -> u64{
        let mut balance: u64 = 0;
        for output in self.utxo{
            if output.address == self.public_key {
                balance += output.amount;
            }
        }
        balance
    }

    fn fetch_utxo(&mut self, blockchain: &Blockchain){
        for block in &blockchain.chain{
            for transaction in &block.data{
                for output in &transaction.output{
                    if !output.spent{
                        if output.receiver == self.public_key{
                            self.utxo.push(output.clone());
                        }
                    }
                }
            }
        }
        for transaction in &blockchain.pool{
            for output in &transaction.output{
                if output.receiver == self.public_key{
                    self.utxo.push(output.clone());
                }
            }
        }
        calculate_balance();
    }

    fn make_transaction(&mut self, receiver: String, amount: u64) -> Option<Transaction>{
        self.fetch_utxo();

        if self.balance < amount {
            return None; // impossivel fazer a transacao sem fundos
        }

        let remain = self.balance - amount;
        let first_output = Output::new(self.public_key.clone(), receiver, amount, &self.private_key);
        let remain_output = Output::new(self.public_key.clone(), self.public_key.clone(), remain, &self.private_key);
        let mut outputs = Vec::new();
        outputs.push(first_output);
        outputs.push(remain_output);

        //criando a transacao

        let transaction = Transaction::new(self.utxo.clone(), outputs);
        self.utxo.clear();
        Some(transaction)
    }
}