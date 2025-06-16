use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockHeader {
    pub version: u32,
    pub prev_block_hash: String,
    pub mekle_root: String,
    pub timestamp: u64,
    pub bits: u32,
    pub nonce: u64,
}