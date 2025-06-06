use sha2::{Sha256, Digest};
use ripemd::{Ripemd160, Digest as RipemdDigest};

// SHA256(SHA256)
pub fn sha256d(data: &[u8]) -> [u8; 32] {
    let first = Sha256::digest(data);
    let second = Sha256::digest(&first);
    let mut result = [0u8; 32];
    result.copy_from_slice(&second);
    result
}

// Hex to String
pub fn hash_to_hex(hash: &[u8]) -> String {
    hash.iter().map(|b| format!("{:02x}", b)).collect()
}

// Compute Merkle Tree
pub fn compute_merkle_root(mut txids: Vec<String>) -> String {
    if txids.is_empty() {
        return hash_to_hex(&sha256d(b"")); // for empty trees
    }

    while txids.len() > 1 {
        let mut next_level = vec![];

        for i in (0..txids.len()).step_by(2) {
            let left = &txids[i];
            let right = if i + 1 < txids.len() {
                &txids[i + 1]
            } else {
                left // if odd number of leaves, duplicate the last one
            };

            let mut combined = hex::decode(left).unwrap();
            combined.extend(hex::decode(right).unwrap());

            let new_hash = sha256d(&combined);
            next_level.push(hash_to_hex(&new_hash));
        }

        txids = next_level;
    }

    txids[0].clone()
}

pub fn ripemd160_sha256(data: &str) -> String {
    let sha = Sha256::digest(data.as_bytes());
    let ripemd = Ripemd160::digest(&sha);
    hex::encode(ripemd)
}