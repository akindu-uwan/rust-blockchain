use serde::{Serialize, Deserialize};
use crate::models::Transaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub version: u32,
    pub prev_block_hash: String,
    pub merkle_root: String,
    pub timestamp: u64,
    pub bits: u32,
    pub nonce: u64,
}

pub fn serialize_header(header: &BlockHeader) -> Vec<u8> {
    bincode::serialize(header).unwrap()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}