extern crate blockchainlib;
use blockchainlib::*;

fn main() {
    // Create two wallets
    let mut wallet1 = Wallet::new();
    let mut wallet2 = Wallet::new();
    let output = Output::new(
        wallet1.public_key.clone(),
        wallet2.public_key.clone(),
        2000,
        &wallet1.private_key,
    );
    print!("Output: {:?}", &output);
    print!("{:?}", Output::verify(&output));


    // Create a new blockchain
    let mut blockchain = Blockchain::new(wallet1.public_key.clone(), &wallet1.private_key);
    // Make a transaction from wallet1 to wallet2
    if let Some(transaction) =
        wallet1.make_transaction(wallet2.public_key.clone(), 100, &blockchain)
    {
        let transaction_clone = transaction.clone();
        // Add the transaction to the block data
        match blockchain.add_to_pool(transaction) {
            Ok(_) => println!("Transaction added successfully"),
            Err(err) => println!("Error: {}", err),
        }

        // Print the transaction
        println!("Transaction: {:?}", transaction_clone);
    } else {
        println!("Not enough funds to make the transaction");
    }
    wallet1.fetch_utxo(&blockchain);
    wallet2.fetch_utxo(&blockchain);

    // Print the balance of each wallet
    println!("Wallet 1 balance: {}", wallet1.balance);
    println!("Wallet 2 balance: {}", wallet2.balance);

    //print the pool

    print!("\n\n\n POOL = {:?}", &blockchain.pool);

    //minerar um bloco

    let mut block = Block::new(blockchain.pool.clone(), blockchain.chain.get(blockchain.chain.len()-1).unwrap().prev_hash.clone());

    block.mine(blockchain.pool.clone());

    print!("\n\nMined block: {:?}", &block);

    match blockchain.add_block(block, wallet2.public_key.clone()){
        Ok(_) => println!("\n\n\nblock added!"),
        Err(err) => println!("\n\n\nError: {}", err),
    }

    wallet1.fetch_utxo(&blockchain);
    wallet2.fetch_utxo(&blockchain);
    // Print the balance of each wallet
    println!("\nWallet 1 balance: {}", wallet1.balance);
    println!("\nWallet 2 balance: {}", wallet2.balance);

    print!("\n\n\n POOL = {:?}", &blockchain.pool);
    print!("\n\n\n CHAIN = {:?}", &blockchain.chain);
}
