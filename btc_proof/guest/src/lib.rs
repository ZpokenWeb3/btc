// #![cfg_attr(feature = "guest", no_std)]

use sha2::{Digest, Sha256};
use num_bigint::BigUint;
use num_traits::Zero;
use log::info;

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

// TODO: check length
#[jolt::provable(stack_size = 10000, memory_size = 10000000)]
fn build_merkle_root(mut hashes: Vec<Vec<u8>>) -> [u8; 32] {
    if hashes.is_empty() {
        return [0u8; 32];
    }
    while hashes.len() > 1 {
        if hashes.len() % 2 != 0 {
            hashes.push(hashes.last().unwrap().clone());
        }

        let mut new_level = Vec::new();
        for i in (0..hashes.len()).step_by(2) {
            let mut concat = Vec::new();
            concat.extend_from_slice(&hashes[i]);
            concat.extend_from_slice(&hashes[i + 1]);
            new_level.push(sha256(&sha256(&concat)).to_vec());
        }
        hashes = new_level;
    }
    hashes[0].clone().try_into().expect("Error getting hash")
}

// TODO: check length
#[jolt::provable(stack_size = 10000, memory_size = 10000000)]
fn check_tx(tx: &[u8], merkle_root: &[u8], all_tx: Vec<Vec<u8>>) -> bool {
    for i in all_tx.iter() {
        if eq(i, tx) == true {
            info!("tx is found!");
            let expected_mrkl_root = build_merkle_root(all_tx);
            info!("expected_mrkl_root is {:#?}", expected_mrkl_root);
            return eq(&expected_mrkl_root, merkle_root);
        }
    }
    panic!("Merkle root mismatch: proof should be invalid!");
}

// TODO: check length
#[jolt::provable]
fn bits_to_target(bits: &[u8]) -> BigUint {
    let bits = u32::from_le_bytes(bits.try_into().expect("Error getting bits"));
    let (mant, expt) = {
        let unshifted_expt = bits >> 24;
        if unshifted_expt <= 3 {
            ((bits & 0xFFFFFF) >> (8 * (3 - unshifted_expt as usize)), 0)
        } else {
            (bits & 0xFFFFFF, 8 * ((bits >> 24) - 3))
        }
    };
    if mant > 0x7F_FFFF {
        BigUint::zero()
    } else {
        BigUint::from(mant) << expt as usize
    }
}

// TODO: add length check -> 80 bytes
#[jolt::provable]
pub fn check_proof_of_work(hash_bytes: &[u8], bits: &[u8]) -> bool {
    let target = bits_to_target(bits);
    let hash_int = BigUint::from_bytes_le(&hash_bytes); //U256::from_le_slice(&value);
    if hash_int <= target {
        return true;
    }
    panic!("PoW mismatch: proof should be invalid!");
}