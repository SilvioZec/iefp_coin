    use blockchainlib::*;
    use std::thread;
    use std::time::Duration;

    fn main() {
        let mut iefp_coin = Blockchain::new();
        iefp_coin.add_block("Block 1 Data".to_owned());

        // Adiciona um atraso de 1 segundo
        thread::sleep(Duration::from_secs(1));

        iefp_coin.add_block("Block 2 Data".to_owned());
        
        println!("{:#?}", iefp_coin);
    }
