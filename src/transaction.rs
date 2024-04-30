use super::*;

#[derive(Debug)]
pub struct Transaction {
    pub input: Vec<Output>,
    pub output: Vec<Output>,
    pub value: u64,
}

impl Transaction {
    fn new(input: Vec<Output>, output: Vec<Output>) -> Self{
        let value = Self::calculate_value(&input, &output);
        Transaction {
            input,
            output,
            value, 
        }
    }

    fn calculate_value (input: &Vec<Output>, output: &Vec<Output>) -> u64{
        let mut input_total : u64 = 0;
        for individual_input in input{
            input_total += individual_input.amount;
        }
        let mut output_total: u64 = 0;
        for individual_output in output{
            output_total += individual_output.amount;
        }
        if input_total == output_total{input_total}
        else {
            0
        }
    }
}