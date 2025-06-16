mod models;
mod utxo;
mod script;
mod crypto;
mod difficulty;
mod block;
mod mining;

//use crate::models::*;
//use crate::utxo::*;
//use crate::script::ScriptItem::{Op, Data};
//use crate::script::execute_script;
//use crate::crypto::*;
//use crate::difficulty::validate_pow;
use mining::mine_block;
use difficulty::bits_to_target;

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

    /* 
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
    */
    /* 
    let pubkey = "my-public-key";
    let sig = "signed-by-me";
    let pubkey_hash = ripemd160_sha256(pubkey);

    // Spending side (TxIn)
    let script_sig = vec![
        Data(sig.to_string()),
        Data(pubkey.to_string()),
    ];

    // Locking script (TxOut)
    let script_pubkey = vec![
        Op("OP_DUP".into()),
        Op("OP_HASH160".into()),
        Data(pubkey_hash.clone()),
        Op("OP_EQUALVERIFY".into()),
        Op("OP_CHECKSIG".into()),
    ];

    let valid = execute_script(&script_sig, &script_pubkey);
    println!("✅ Script result: {}", valid);  // should be true
    */

    /* 
    //1.5 Test
    //simulate mining with dummy data
    let header = b"fake block header";
    let bits = 0x1d00ffff;

    let is_valid = validate_pow(header, bits);
    println!("Block valid under pow? {}", is_valid);
    */

    //2.1 Test
    let prev_hash = "0000000000000000".to_string();
    let merkle_root = "4e3f5...".to_string();
    let bits = 0x1f00ffff;

    let mined_header = mine_block(prev_hash, merkle_root, bits);
    println!("{:#?}", mined_header);




}
