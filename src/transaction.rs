use super::*;

#[derive(Debug, PartialEq)]
pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
}

impl Transaction {
    pub fn new(inputs: Vec<Output>, outputs: Vec<Output>) -> Self {
        Transaction {
            inputs,
            outputs,
        }
    }

    // Utilizes the validation function on Output to individually validate each output. If any output is incorrect,
    // invalidates the entire transaction
    pub fn validate_transaction(&self) -> bool {
        let mut invalid = false;

        let input_total: u64 = self.inputs.iter().map(|o| o.amount as u64).sum();
        let output_total: u64 = self.outputs.iter().map(|o| o.amount as u64).sum();

        for individual_output in &self.outputs {
            if !Output::verify(&individual_output) {
                invalid = true;
            }
        }
        if input_total != output_total {
            invalid = true;
        }
        invalid
    }
}