// Convert Bitcoin bits to full target as 256-bit number
pub fn bits_to_target(bits: u32) -> num_bigint::BigUint {
    use num_bigint::BigUint;
    use num_traits::FromPrimitive;

    let exponent = (bits >> 24) as u8;
    let coefficient = bits & 0x00ffffff;

    let base = BigUint::from_u32(coefficient).unwrap();
    let shift = 8 * (exponent as usize - 3);

    base << shift
}

use crate::crypto::sha256d;
use num_bigint::BigUint;

pub fn validate_pow(block_data: &[u8], bits: u32) -> bool {
    let hash = sha256d(block_data);
    let hash_num = BigUint::from_bytes_be(&hash);
    let target = bits_to_target(bits);
    println!("{}", hash_num);
    println!("{}", target);

    hash_num < target
}
