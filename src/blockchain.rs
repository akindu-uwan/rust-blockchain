use num_bigint::BigUint;

use crate::block::{Block, BlockHeader};
use crate::models::Transaction;
use crate::mining::mine_block;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub bits: u32,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_tx = Transaction {
            txid: "genesis_tx".to_string(),
            inputs: vec![],
            outputs: vec![],
        };

        let header = mine_block("0".repeat(64), "genesis_merkle".into(), 0x1f00ffff);

        let genesis_block = Block {
            header,
            transactions: vec![genesis_tx],
        };

        Blockchain {
            chain: vec![genesis_block],
            bits: 0x1f00ffff,
        }
    }

    pub fn add_block(&mut self, txs: Vec<Transaction>) {
        let prev_hash = self.get_latest_hash();
        let merkle = "fake_merkle".to_string();
        let header = mine_block(prev_hash, merkle, self.bits);

        let block = Block { header, transactions: txs };
        self.chain.push(block);
    }

    pub fn get_latest_hash(&self) -> String {
        use crate::crypto::{sha256d, hash_to_hex};
        use crate::block::serialize_header;

        let last = &self.chain[self.chain.len() - 1];
        let raw = serialize_header(&last.header);
        let hash = sha256d(&raw);
        hash_to_hex(&hash)
    }

    pub fn is_valid_block(&self, block: &Block, prev_block: &Block) -> bool {
        use crate::block::serialize_header;
        use crate::crypto::{sha256d, hash_to_hex};
        use crate::difficulty::bits_to_target;

        let header_bytes = serialize_header(&block.header);
        let hash = sha256d(&header_bytes);
        let hash_num = BigUint::from_bytes_be(&hash);
        let target = bits_to_target(block.header.bits);

        let prev_header_bytes = serialize_header(&prev_block.header);
        let prev_hash = sha256d(&prev_header_bytes);
        let prev_hash_hex = hash_to_hex(&prev_hash);

        hash_num < target && block.header.prev_block_hash == prev_hash_hex
    }

    pub fn validate_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if !self.is_valid_block(current, previous) {
                println!("Invalid block at index {}", i);
                return false;
            }
        }

        println!("Blockchain is valid.");
        true
    }
}
