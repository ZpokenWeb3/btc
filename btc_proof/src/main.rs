use hex;
use sha2::{Sha256, Digest};

struct BTCHeader {
    version: [u8; 4],
    prev_hash: [u8; 32],
    merkle_root: [u8; 32],
    timestamp: [u8; 4],
    bits: [u8; 4],
    nonce: [u8; 4],
}

fn reverse_bytes(bytes: &[u8]) -> Vec<u8> {
    let mut reversed = bytes.to_vec();
    reversed.reverse();
    reversed
}

pub fn main() {
    let raw_block = "010000007ec47d8bbca521ebbc8a988aeebef4368d79917b10b2088b1237000000000000959c09303c995c62755d5ab07b670cf5bfae2728651d20487593fad3fe07ad527e6ab54dacb5001bd0d686350101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0704acb5001b0121ffffffff0100f2052a010000004341049dc9b104e10a99e9e59518d9d76b8a1a3a054e64283e1706fe679eefad237ec828cd4e2fd4d3da9cc90361d0f281798926c8115e11c0117bec8ce278befd52c8ac00000000";
    let raw_block_bytes = hex::decode(raw_block).expect("Invalid hex string");
    println!("Block: {:#?}", raw_block_bytes);
    let hash = "000000000000a609f0b57fc9f6b30797a02d0729f4a7835dd0c7fbb87387a836";
    let hash_bytes = hex::decode(hash).expect("Invalid hex string");
    println!("Hash: {:#?}", hash_bytes);
    // let version: u32 = 1;
    // let version_bytes: [u8; 4] = version.to_be_bytes();
    // let previousblockhash = "00000000000037128b08b2107b91798d36f4beee8a988abceb21a5bc8b7dc47e";  // Previous Block Hash
    // let merkleroot = "52ad07fed3fa937548201d652827aebff50c677bb05a5d75625c993c30099c95";  // Merkle Root
    // let time: u32 = 1303734910;  // Time
    // let time_bytes: [u8; 4] = time.to_be_bytes();
    // let bits: u32 = 0x1b00b5ac;  // Bits
    // let bits_bytes: [u8; 4] = bits.to_be_bytes();
    // let nonce: u32 = 898029264;  // Nonce
    // let nonce_bytes: [u8; 4] = nonce.to_be_bytes();

    // let previousblockhash_bytes = hex::decode(previousblockhash).expect("Invalid hex string");
    // let merkleroot_bytes = hex::decode(merkleroot).expect("Invalid hex string");

    // let previousblockhash_bytes = reverse_bytes(&previousblockhash_bytes);
    // let merkleroot_bytes = reverse_bytes(&merkleroot_bytes);

    // let mut header = Vec::new();
    // header.extend_from_slice(&version_bytes);
    // header.extend_from_slice(&previousblockhash_bytes);  // 32 bytes
    // header.extend_from_slice(&merkleroot_bytes);        // 32 bytes
    // header.extend_from_slice(&time_bytes);
    // header.extend_from_slice(&bits_bytes);
    // header.extend_from_slice(&nonce_bytes);

    // let first_sha256 = Sha256::digest(&header);
    // let second_sha256 = Sha256::digest(&first_sha256);

    // println!("Block hash: {:x}", second_sha256);

    // let (prove_hash, verify_hash) = guest::build_check_header();
    // let result = std::panic::catch_unwind(|| prove_hash(&hash_bytes, &header));
    // match result {
    //     Ok((output, proof)) => {
    //         let is_valid = verify_hash(proof);
    //         println!("output: {:#?}", output);
    //         println!("valid: {}", is_valid);
    //     }
    //     Err(_) => {
    //         println!("Proof generation failed because hashes are different!");
    //     }
    // }
    
}
