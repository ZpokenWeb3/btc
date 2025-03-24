#![cfg_attr(feature = "guest", no_std)]

use sha2::{Digest, Sha256};

#[jolt::provable]
fn eq_hash(hash1: &[u8], hash2: &[u8]) -> [u8; 32] {
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

#[jolt::provable]
fn check_header<T>(hash: &[u8], header: &[u8]) -> [u8; 32] {
    let res = sha256(&sha256(header));
    eq_hash(hash, &res)
}

#[jolt::provable]
fn check_height(prev_height1: u128, curr_height: u128) -> bool {
    if prev_height1 + 1 == curr_height {
        return true;
    }
    panic!("Height mismatch: proof should be invalid!");
}

#[jolt::provable]
pub fn check_proof_of_work(header: &[u8; 80]) -> bool {
    // step 1: Extract the 'bits' field in the header and convert into byte array
    let bit_bytes: [u8; 4] = header.bits();
    // step 2: pass array to convert the compressed target into fully expanded form
    let target = bits_to_target(&bit_bytes);
    // step 3: Calculate the hash of the header
    let hash = get_block_hash(header);
    // if hash returns expected type continue, else, fail.
    if let Ok(value) = hash {
        // Convert the hash to a little endian U256
        let hash_int = U256::from_le_slice(&value); // reverse order
        // step 4: Compare the hash to the target
        hash_int <= target
    }
    // } else {
    //     false
    // }
    panic!("PoW: proof should be invalid!");
}

// TODO 
// pub fn get_retarget_height(height: u32) -> u32 {
//     if DIFFICULTY_ADJUSTMENT_INTERVAL > height {
//         return 0;
//     }

//     (height - 1) / DIFFICULTY_ADJUSTMENT_INTERVAL * DIFFICULTY_ADJUSTMENT_INTERVAL
// }

// TODO 
// pow of the chain