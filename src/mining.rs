use crate::block::BlockHeader;
use crate::crypto::{sha256d, hash_to_hex};
use crate::difficulty::bits_to_target;
use num_bigint::BigUint;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn mine_block(prev_hash: String, merkle_root: String, bits: u32) -> BlockHeader {
    let mut nonce = 0u64;
    let target = bits_to_target(bits);

    loop {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let header = BlockHeader {
            version: 1,
            prev_block_hash: prev_hash.clone(),
            merkle_root: merkle_root.clone(), 
            timestamp,
            bits,
            nonce,
        };

        let serialized = bincode::serialize(&header).unwrap();
        let hash = sha256d(&serialized);
        let hash_num = BigUint::from_bytes_be(&hash);

        if hash_num < target {
            //println!("Mined block with nonce {}: {}", nonce, hash_to_hex(&hash));
            //println!("{}", target);
            //println!("{}", hash_num);
            return header;
        }

        nonce += 1;

        if nonce % 100_000 == 0 {
            //println!("...mining... nonce: {}", nonce);
        }
    }
}
