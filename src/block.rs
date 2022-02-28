use std::fmt::Debug;

use crate::{difficulty_bytes_as_u128, u128_bytes, u32_bytes, u64_bytes};
use crate::{hashable::Hashable, BlockHash};

pub struct Block {
    /// This block's location within the list of blocks
    pub index: u32,
    /// Gives our blockchain a sense of time
    pub timestamp: u128,
    /// Cryptographic fingerprint of all of the above data concatnated together
    pub hash: BlockHash,
    /// Cryptographic fingerprint of previous block
    pub prev_block_hash: BlockHash,
    /// A special number used for mining (for PoW verification)
    pub nonce: u64,
    /// Any relevant information or events that have occurred for/in the block
    pub payload: String,
    pub difficulty: u128,
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Block[{}]: {} at: {} with: {} nonce: {}",
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.payload,
            &self.nonce
        )
    }
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: BlockHash,
        nonce: u64,
        payload: String,
        difficulty: u128,
    ) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce,
            payload,
            difficulty,
        }
    }

    pub fn mine(&mut self) {
        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;
            let hash = self.hash();
            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(self.payload.as_bytes());
        bytes.extend(&u128_bytes(&self.difficulty));

        bytes
    }
}

pub fn check_difficulty(hash: &BlockHash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash)
}

#[test]
fn test_block() {
    let mut block = Block::new(
        0,
        0,
        vec![0; 32],
        0,
        "Genesis block".to_string(),
        0x00008fffffffffffffffffffffffffff,
    );
    block.hash();
    println!("{:?}", &block);

    block.mine();
    println!("{:?}", &block);
}
