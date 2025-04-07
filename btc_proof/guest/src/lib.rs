// #![cfg_attr(feature = "guest", no_std)]

use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};

#[jolt::provable]
fn eq(hash1: &[u8], hash2: &[u8]) -> bool {
    if hash1.eq(hash2) && hash1.len() == 32 {
        return true;
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
fn check_header_hash<T>(hash: &[u8], header: &[u8]) -> bool {
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

#[jolt::provable]
fn build_merkle_root(mut hashes: Vec<[u8; 32]>) -> [u8; 32] {
    if hashes.is_empty() {
        return [0u8; 32];
    }
    while hashes.len() > 1 {
        if hashes.len() % 2 != 0 {
            hashes.push(*hashes.last().unwrap());
        }

        let mut new_level = Vec::new();
        for i in (0..hashes.len()).step_by(2) {
            let mut concat = Vec::new();
            concat.extend_from_slice(&hashes[i]);
            concat.extend_from_slice(&hashes[i + 1]);
            new_level.push(sha256(&sha256(&concat)));
        }
        hashes = new_level;
    }
    hashes[0]
}

#[jolt::provable]
fn check_tx(tx: &[u8; 32], merkle_root: &[u8], all_tx: Vec<[u8; 32]>) -> bool {
    for i in all_tx.iter() {
        if eq(i, tx) == true {
            let expected_mrkl_root = build_merkle_root(all_tx);
            return eq(&expected_mrkl_root, merkle_root);
        }
        else {
            return false;
        }
    }
    panic!("Merkle root mismatch: proof should be invalid!");
}
