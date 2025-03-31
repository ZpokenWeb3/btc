// #![cfg_attr(feature = "guest", no_std)]

use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};

#[jolt::provable]
fn eq(hash1: &[u8], hash2: &[u8]) -> [u8; 32] {
    if hash1.eq(hash2) && hash1.len() == 32 {
        return hash1.try_into().expect("Expected exactly 32 bytes");
    }
    panic!("Hash mismatch: proof should be invalid!");
}

#[jolt::provable]
fn sha256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    Into::<[u8; 32]>::into(result)
}

// TODO: check length (80 b) panics when [u8; 80] needs #[derive(Serialize)]
#[jolt::provable]
pub fn get_block_hash(header: &[u8]) -> [u8; 32] {
    let hash: [u8; 32] = sha256(&sha256(header));
    hash
}

#[jolt::provable]
fn check_header_hash<T>(hash: &[u8], header: &[u8]) -> [u8; 32] {
    let res = sha256(&sha256(header));
    eq(hash, &res)
}

#[jolt::provable]
fn check_height(prev_height: u32, curr_height: u32) -> bool {
    if prev_height + 1 == curr_height {
        return true;
    }
    panic!("Height mismatch: proof should be invalid!");
}
