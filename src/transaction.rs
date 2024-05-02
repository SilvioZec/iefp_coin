use super::*;

#[derive(Debug)]
pub struct Transaction {
    pub input: Vec<Output>,
    pub output: Vec<Output>,
}

impl Transaction {
    pub fn new(input: Vec<Output>, output: Vec<Output>) -> Self{
        Transaction {
            input,
            output,
        }
    }

    //utiliza a funcao de validacao em output para individualmente validar cada saida. caso alguma esteja incorreta, invalida toda a transacao
    pub fn validate_transaction (&self) -> bool {
        let mut invalid = false;


        let mut input_total: u64 = 0;
        let mut output_total: u64 = 0;

        //itera sobre as saidas verificando sua assinatura E calculando o valor total
        for individual_input in &self.input{
            input_total += individual_input.amount;
            if !Output::verify(&individual_input){
                invalid = true;
            }
        }
        for individual_output in &self.output{
            output_total += individual_output.amount;
            if !Output::verify(&individual_output){
                invalid = true;
            }
        }
        if input_total != output_total {
            invalid = true;
        }
        invalid
    }
}