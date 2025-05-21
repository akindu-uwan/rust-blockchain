#[derive(Debug, Clone)]
pub struct TxOut {
    pub value: u64, //amount in satoshi
    pub pubkey_hash: String, //recipient public key hash
}

#[derive(Debug, Clone)]
pub struct TxIn {
    pub pre_tx: String, //txid of the previous hash
    pub index: usize, //index of the output
    pub signature: String, //digital signature
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub txid: String, //transaction ID
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
}

#[derive(Debug, Clone)]
pub struct BlockHeader {
    pub version: u32,
    pub prev_block_hash: String,
    pub merkle_root: String,
    pub timestamp: u64,
    pub bits: u32,
    pub nonce: u64,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub txs: Vec<Transaction>,
}