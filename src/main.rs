extern crate blockchainlib;
use std::io::{self, Write};

use blockchainlib::*;

fn main() {
    //creating a structure to store wallets
    print!("\nPlease introduce the genesis wallet password\n");
    io::stdout().flush().unwrap();
    let mut genesis_pass = String::new();
    io::stdin()
        .read_line(&mut genesis_pass)
        .expect("Failed to read line");
    let mut wallet_list: Vec<Wallet> = Vec::new();
    //creating the first wallet
    let wallet1 = Wallet::new(genesis_pass);
    wallet_list.push(wallet1);

    //creating the blockchain
    let mut blockchain = Blockchain::new(
        wallet_list[0].public_key.clone(),
        &wallet_list[0].private_key,
    );

    print!(
        "Blockchain created. Genesis block added and set to wallet: {}\n\n",
        wallet_list[0].public_key.clone()
    );

    loop {
        println!("\nMenu:");
        println!("1. Print wallet list");
        println!("2. Create new wallet");
        println!("3. Get wallet's balance");
        println!("4. Create new transaction");
        println!("5. Print Blockchain and transaction Pool");
        println!("6. Mine blockchain");
        println!("0. Quit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice = match choice.trim().parse::<u32>() {
            Ok(choice) => choice,
            Err(_) => {
                println!("Invalid choice. Please try again.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("\n========================================= WALLETS =========================================");
                for (index, wallet) in wallet_list.iter().enumerate() {
                    print!("\nWallet {} = {:?}", index + 1, wallet.public_key);
                }
                print!("\n\n===========================================================================================\n\n")
            }
            2 => {
                println!("\nPlease enter the password for this new wallet:\n");
                io::stdout().flush().unwrap();
                let mut pass = String::new();
                io::stdin()
                    .read_line(&mut pass)
                    .expect("Failed to read line");
                let new_wallet = Wallet::new(pass.clone());
                let wallet_address = new_wallet.public_key.clone();
                wallet_list.push(new_wallet);
                print!(
                    "\nWallet {} created and sucessfully added!\n",
                    wallet_address
                );
            }
            3 => {
                println!("\nPlease enter the wallet number:\n");
                io::stdout().flush().unwrap();
                let mut wallet_index = String::new();
                io::stdin()
                    .read_line(&mut wallet_index)
                    .expect("Failed to read line");

                let wallet_index = match wallet_index.trim().parse::<usize>() {
                    Ok(wallet_index) => wallet_index - 1,
                    Err(_) => {
                        println!("Invalid choice. Please try again.");
                        continue;
                    }
                };
                if wallet_index >= wallet_list.len() {
                    print!("\nWallet number does not exist\n");
                    continue;
                }

                println!("\nPlease enter the wallet's password:\n");
                io::stdout().flush().unwrap();
                let mut pass = String::new();
                io::stdin()
                    .read_line(&mut pass)
                    .expect("Failed to read line");

                if pass != wallet_list[wallet_index].password {
                    print!("\nPassword incorrect\n");
                    continue;
                }

                wallet_list[wallet_index].fetch_utxo(&blockchain);
                print!(
                    "\nWallet address: {}\nBalance: {}\n",
                    wallet_list[wallet_index].public_key.clone(),
                    wallet_list[wallet_index].balance.clone()
                );
            }
            4 => {
                //getting sender address
                println!("\nPlease enter the sender wallet number:\n");
                io::stdout().flush().unwrap();
                let mut sender_wallet_index = String::new();
                io::stdin()
                    .read_line(&mut sender_wallet_index)
                    .expect("Failed to read line");

                let sender_wallet_index = match sender_wallet_index.trim().parse::<usize>() {
                    Ok(sender_wallet_index) => sender_wallet_index - 1,
                    Err(_) => {
                        println!("Invalid choice. Please try again.");
                        continue;
                    }
                };
                if sender_wallet_index >= wallet_list.len() {
                    print!("\nWallet number does not exist\n");
                    continue;
                }

                println!("\nPlease enter the wallet's password:\n");
                io::stdout().flush().unwrap();
                let mut pass = String::new();
                io::stdin()
                    .read_line(&mut pass)
                    .expect("Failed to read line");

                if pass != wallet_list[sender_wallet_index].password {
                    print!("\nPassword incorrect\n");
                    continue;
                }

                //getting receiver address
                println!("\nPlease enter the receiver wallet number:\n");
                io::stdout().flush().unwrap();
                let mut receiver_wallet_index = String::new();
                io::stdin()
                    .read_line(&mut receiver_wallet_index)
                    .expect("Failed to read line");

                let receiver_wallet_index = match receiver_wallet_index.trim().parse::<usize>() {
                    Ok(receiver_wallet_index) => receiver_wallet_index - 1,
                    Err(_) => {
                        println!("Invalid choice. Please try again.");
                        continue;
                    }
                };
                if receiver_wallet_index >= wallet_list.len() {
                    print!("\nWallet number does not exist\n");
                    continue;
                }
                //getting transaction amount
                print!("Enter the amount: ");
                io::stdout().flush().unwrap();
                let mut amount = String::new();
                io::stdin()
                    .read_line(&mut amount)
                    .expect("Failed to read line");

                let amount = match amount.trim().parse::<u64>() {
                    Ok(amount) => amount,
                    Err(_) => {
                        println!("Invalid amount. Please try again.");
                        continue;
                    }
                };

                //making the transaction
                let receiver_address = wallet_list[receiver_wallet_index].public_key.clone();
                if let Some(transaction) = wallet_list[sender_wallet_index].make_transaction(
                    receiver_address,
                    amount,
                    &blockchain,
                ) {
                    // Add the transaction to the block data
                    match blockchain.add_to_pool(transaction) {
                        Ok(_) => println!("Transaction added successfully"),
                        Err(err) => println!("Error: {}", err),
                    }
                } else {
                    println!("Not enough funds to make the transaction");
                }
            }
            5 => {
                print!("\nBlockchain :\n");
                for block in &blockchain.chain {
                    print!("\n\tBlock :");
                    print!("\n\t\tTimestamp: {}", block.timestamp);
                    print!("\n\t\tPrevious Hash: {}", block.prev_hash);
                    print!("\n\t\tHash: {}", block.hash);
                    print!("\n\t\tTransactions : ");
                    for transaction in &block.data {
                        print!("\n\t\t\tTransaction :");
                        print!("\n\t\t\t\tInputs :");
                        for input in &transaction.inputs {
                            print!("\n\t\t\t\t\t{:?}", input)
                        }
                        print!("\n\t\t\t\tOutputs :");
                        for output in &transaction.outputs {
                            print!("\n\t\t\t\t\t{:?}", output)
                        }
                    }
                }

                print!("\nPool :\n");
                for transaction in &blockchain.pool {
                    print!("\nTransaction :");
                    print!("\n\tInputs :");
                    for input in &transaction.inputs {
                        print!("\n\t\t{:?}", input)
                    }
                    print!("\nTransaction :");
                    print!("\n\tOutputs :");
                    for output in &transaction.outputs {
                        print!("\n\t\t{:?}", output)
                    }
                }
            }
            6 => {
                //get miner wallet adress
                println!("\nPlease enter the miner wallet number:\n");
                io::stdout().flush().unwrap();
                let mut miner_wallet_index = String::new();
                io::stdin()
                    .read_line(&mut miner_wallet_index)
                    .expect("Failed to read line");

                let miner_wallet_index = match miner_wallet_index.trim().parse::<usize>() {
                    Ok(miner_wallet_index) => miner_wallet_index - 1,
                    Err(_) => {
                        println!("Invalid choice. Please try again.");
                        continue;
                    }
                };
                if miner_wallet_index >= wallet_list.len() {
                    print!("\nWallet number does not exist\n");
                    continue;
                }

                println!("\nPlease enter the wallet's password:\n");
                io::stdout().flush().unwrap();
                let mut pass = String::new();
                io::stdin()
                    .read_line(&mut pass)
                    .expect("Failed to read line");

                if pass != wallet_list[miner_wallet_index].password {
                    print!("\nPassword incorrect\n");
                    continue;
                }
                let miner_adress = wallet_list[miner_wallet_index].public_key.clone();
                print!(
                    "This is the previous hash: {}",
                    &blockchain.chain[blockchain.chain.len() - 1].hash.clone()
                );
                let mut block = Block::new(
                    blockchain.pool.clone(),
                    blockchain
                        .chain
                        .get(blockchain.chain.len() - 1)
                        .unwrap()
                        .hash
                        .clone(),
                );

                block.mine(blockchain.pool.clone());

                print!("\n\nMined block: {:?}", &block);

                match blockchain.add_block(block, miner_adress) {
                    Ok(_) => println!("\n\n\nblock added!"),
                    Err(err) => println!("\n\n\nError: {}", err),
                }
            }
            0 => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}
