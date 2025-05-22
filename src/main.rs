mod models;
mod utxo;

use models::*;
use utxo::UTXOSet;

fn main() {
    /* 
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
    */

    // Dummy transactions
    let txid1 = "tx1".to_string();
    let tx1 = Transaction {
        txid: txid1.clone(),
        inputs: vec![],
        outputs: vec![TxOut {
            value: 50000,
            pubkey_hash: "alice".to_string(),
        }],
    };

    let tx2 = Transaction {
        txid: "tx2".to_string(),
        inputs: vec![TxIn {
            prev_tx: txid1.clone(),
            index: 0,
            signature: "signed".to_string(),
        }],
        outputs: vec![TxOut {
            value: 30000,
            pubkey_hash: "bob".to_string(),
        }],
    };

    let mut utxo = UTXOSet::new();
    utxo.apply_transaction(&tx1);
    utxo.apply_transaction(&tx2);

    println!("💰 Alice balance: {}", utxo.get_balance("alice")); // should be 0
    println!("💰 Bob balance:   {}", utxo.get_balance("bob"));   // should be 30000


}
