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
}
