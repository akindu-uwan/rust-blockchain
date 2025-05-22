use std::collections::HashMap;
use crate::models::*;
use crate::models::{TxIn, TxOut, Transaction};

// String = UTXOKey
pub type UTXOKey = String;

pub struct UTXOSet {
    pub set: HashMap<UTXOKey, TxOut>,
}

// Create key format
fn key(txid: &str, index: usize) -> UTXOKey {
    format!("{}:{}", txid, index)
}

impl UTXOSet {
    pub fn new() -> Self {
        UTXOSet {
            set: HashMap::new(),
        }
    }

    pub fn apply_transaction(&mut self, tx: &Transaction) {
        // Spend TxOut (TxIn)
        for input in &tx.inputs {
            let k = key(&input.prev_tx, input.index);
            self.set.remove(&k);
        }

        // Add new TxOut
        for (i, output) in tx.outputs.iter().enumerate() {
            let k = key(&tx.txid, i);
            self.set.insert(k, output.clone());
        }
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        self.set
            .values()
            .filter(|utxo| utxo.pubkey_hash == address)
            .map(|utxo| utxo.value)
            .sum()
    }

    pub fn list_utxos(&self, address: &str) -> Vec<(UTXOKey, TxOut)> {
        self.set
            .iter()
            .filter(|(_, utxo)| utxo.pubkey_hash == address)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
}
