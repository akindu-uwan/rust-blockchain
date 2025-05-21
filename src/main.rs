mod models;

use models::*;

fn main() {
    
    let out = TxOut {
        value: 50000,
        pubkey_hash: "recipient hash".to_string(),
    };

    let tx = Transaction {
        txid: "dummyid".to_string(),
        inputs: vec![],
        outputs: vec![out],
    };

    let header = BlockHeader {
        version: 1,
        prev_block_hash: "000000".to_string(),
        merkle_root: "merkle_root".to_string(),
        timestamp: 1680000000,
        bits: 0x1d00ffff,
        nonce: 0,
    };

    let block = Block {
        header,
        txs: vec![tx],
    };

    println!("{:#?}", block);

}
