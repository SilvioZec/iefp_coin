

#[derive(Debug, Clone, PartialEq)]
pub struct Output {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub signature: String,
    pub spent: bool,
}

use std::str::FromStr;

use secp256k1::{Secp256k1, Message, SecretKey, PublicKey, Signature};

impl Output {
    pub fn new(sender_pub_key : String, receiver_pub_key : String, amount : u64, secret_key: &SecretKey) -> Self {
        let mut output = Output {
            sender : sender_pub_key,
            receiver : receiver_pub_key,
            amount : amount,
            signature : String::new(),
            spent: false,
        };
        output.sign(secret_key);
        output
    }

    //assina a saida com a chave privada do remetente
    fn sign(&mut self, secret_key: &SecretKey) {
        let context = Secp256k1::new();
        let message = self.create_message();
        let signature = context.sign(&Message::from_digest_slice(&message).unwrap(), secret_key);
        
        self.signature = signature.to_string();
    }

    //cria uma mensagem em bytes para ser codificada (assinada) pela chave privada
    fn create_message(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(self.sender.as_bytes());
        bytes.extend(self.receiver.as_bytes());
        bytes.extend(&self.amount.to_le_bytes());
        bytes
    }
}


impl Output {
    //verifica se a saida é valida, confirmando a assinatura e o valor
    pub fn verify(&self) -> bool{
        let context = Secp256k1::new();
        let message = self.create_message();
        let signature = Signature::from_str(&self.signature).unwrap();
        let sender_pub_key = PublicKey::from_str(&self.sender).unwrap();

        context.verify(&Message::from_slice(&message).unwrap(), &signature, &sender_pub_key).is_ok()
    }
}