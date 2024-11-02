use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.mine_block(4);
        block
    }

    pub fn calculate_hash(&self) -> String {
        let block_data = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(block_data);
        let result = hasher.finalize();
        hex::encode(result)
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Block mined: {}", self.hash);
    }
}
