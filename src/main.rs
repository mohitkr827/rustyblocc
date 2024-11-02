use std::io;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
    println!("Genesis Block mined: {}", blockchain.chain[0].hash);

    // Get the number of blocks to add from the user
    let mut input = String::new();
    println!("Enter the number of blocks to mine:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_blocks: u32 = input.trim().parse().expect("Please enter a valid number");

    // Loop to add the specified number of blocks
    for i in 1..=num_blocks {
        let mut data = String::new();
        println!("Enter data for block {}:", i);
        io::stdin().read_line(&mut data).expect("Failed to read line");

        println!("Mining block {}...", i);
        blockchain.add_block(data.trim().to_string());
        println!("Block mined: {}", blockchain.chain.last().unwrap().hash);
    }

    // Display the entire blockchain
    println!("\nBlockchain:");
    for block in &blockchain.chain {
        println!("{:?}", block);
    }

    // Check if the blockchain is valid
    println!("\nIs blockchain valid? {}", blockchain.is_valid_chain());
}
