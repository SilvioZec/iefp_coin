use super::*;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use rand::random;

#[derive(Debug)]
pub struct Wallet {
    pub private_key: SecretKey,
    pub public_key: String,
    pub balance: u64,
    utxo: Vec<Output>,
    pub password: String,
}

impl Wallet {
    pub fn new(password: String) -> Self {
        //cria um par de chaves elipticas e atribui a carteira
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&random::<[u8; 32]>()).unwrap();
        let public_key = PublicKey::from_secret_key(&secp, &secret_key).to_string();

        Wallet {
            private_key: secret_key,
            public_key: public_key,
            balance: 0,
            utxo: Vec::new(),
            password,
        }
    }

    //calcula o saldo da carteira com base nas utxo (saidas nao gastas)
    fn calculate_balance(&self) -> u64{
        let mut balance: u64 = 0;
        for output in &self.utxo{
            if output.receiver == self.public_key {
                balance += output.amount;
            }
        }
        balance
    }

    // Busca todas as saidas nao gastas destinadas a essa carteira e calcula o saldo
    pub fn fetch_utxo(&mut self, blockchain: &Blockchain){
        self.utxo.clear();
        for block in &blockchain.chain{
            for transaction in &block.data{
                for output in &transaction.outputs{
                    if !output.spent.get() && output.receiver == self.public_key{
                        self.utxo.push(output.clone());    
                    }
                }
            }
        }
        for transaction in &blockchain.pool{
            for output in &transaction.outputs{
                if !output.spent.get() && output.receiver == self.public_key{
                    self.utxo.push(output.clone());    
                }
            }
        }
        self.balance = self.calculate_balance();
    }

    //cria uma transacao
    pub fn make_transaction(&mut self, receiver: String, amount: u64, blockchain: &Blockchain) -> Option<Transaction>{
        self.fetch_utxo(blockchain);

        if self.balance < amount {
            return None; // impossivel fazer a transacao sem fundos
        }

        //calcular o restante e criar dois outputs, um para si mesmo e outro para o destinatario
        let remain = self.balance - amount;
        let first_output = Output::new(self.public_key.clone(), receiver, amount, &self.private_key);
        let remain_output = Output::new(self.public_key.clone(), self.public_key.clone(), remain, &self.private_key);

        //criar e adicionar ao vetor
        let mut outputs = Vec::new();
        outputs.push(first_output);
        outputs.push(remain_output);

        //criando a transacao

        let transaction = Transaction::new(self.utxo.clone(), outputs);
        self.utxo.clear();
        Some(transaction)
    }
}