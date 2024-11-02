pub mod block;
use block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            chain: vec![Block::new(0, "Genesis Block".to_string(), String::new())],
            difficulty: 4,
        }
    }

    pub fn add_block(&mut self, data: String) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let block = Block::new(self.chain.len() as u64, data, previous_hash);
        self.chain.push(block);
    }

    pub fn is_valid_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            // Recalculate the hash and compare
            if current.hash != current.calculate_hash() {
                println!("Hash mismatch at block {}", i);
                return false;
            }
            // Verify that previous_hash matches the hash of the previous block
            if current.previous_hash != previous.hash {
                println!("Previous hash mismatch at block {}", i);
                return false;
            }
        }
        true
    }
}
