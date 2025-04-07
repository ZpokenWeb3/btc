use hex;
use sha2::{Sha256, Digest};
use btc_proof::{utils::*, types::*};
use std::fs::File;
use std::io::Read;
use std::time::Instant;

pub fn main() {
    let block_range = 120098..=120104;
    let mut total_time = 0;  
    let mut prev_block_hash: Option<String> = None;  
    let mut prev_block_height: Option<u32> = None;  

    for block_height in block_range {
        let block_filename = format!("../blocks/block_{}.json", block_height);
        println!("\nProcessing block: {}", block_filename);
        
        let start_time = Instant::now();
        
        let mut file = File::open(&block_filename).expect("Failed to open file");
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Failed to read file");

        let block: Block = serde_json::from_str(&data).expect("Failed to parse JSON");
        
        let header = BTCHeader::from_block(&block);

        let expected_hash = reverse(&hex::decode(block.hash.clone()).expect("Invalid hex string"));

        if block_height > 120098 {
            if let Some(prev_hash) = &prev_block_hash {
                // check prev_hash
                println!("\nPrev_hash check");
                let exp_prev_hash = reverse(&hex::decode(block.prev_block).expect("Invalid hex string"));
                let prev_hash = reverse(&hex::decode(prev_hash).expect("Invalid hex string"));
                let start_time = Instant::now();
                let (prove_prev_hash, verify_prev_hash) = guest::build_eq();
                let result = std::panic::catch_unwind(|| prove_prev_hash(&exp_prev_hash, &prev_hash));
                match result {
                    Ok((output, proof)) => {
                        let is_valid = verify_prev_hash(proof);
                        println!("Valid proof: {}", is_valid);
                    }
                    Err(_) => {
                        println!("Block {}: Proof generation failed because hashes are different!", block_height);
                    }
                }
                let elapsed_time = start_time.elapsed();
                let elapsed_ms = elapsed_time.as_millis();
                println!("Prev_hash processed in {} ms", elapsed_ms);
                total_time += elapsed_ms;
                // check prev_height
                if let Some(prev_height) = &prev_block_height {
                    println!("\nPrev_height check");
                    let height = block.height;
                    let start_time = Instant::now();
                    let (prove_height, verify_height) = guest::build_check_height();
                    let result = std::panic::catch_unwind(|| prove_height(*prev_height, height));
                    match result {
                        Ok((output, proof)) => {
                            let is_valid = verify_height(proof);
                            println!("Valid proof: {}", is_valid);
                        }
                        Err(_) => {
                            println!("Block {}: Proof generation failed because hashes are different!", block_height);
                        }
                    }
                    let elapsed_time = start_time.elapsed();
                    let elapsed_ms = elapsed_time.as_millis();
                    println!("Prev_hash processed in {} ms", elapsed_ms);
                    total_time += elapsed_ms;
                }
                
            }
        }

        // check tx & merkle root
        // Convert all txids in bytes (LE -> BE)
        let mut txids: Vec<Vec<u8>> = block.tx.iter()
            .map(|tx| {
                let mut bytes = hex::decode(&tx.hash).expect("Invalid txid hex");
                bytes.reverse(); // txid are in little-endian
                bytes
            })
            .collect();

        let mut expected_root_bytes = hex::decode(&block.mrkl_root).expect("Invalid merkle_root hex");
        expected_root_bytes.reverse(); // little-endian
        println!("\nTx & merkle root check");
        let start_time_tx = Instant::now();
        let (prove_tx, verify_tx) = guest::build_check_tx();
        // Check the first one, but it has to be provided separately
        let result = std::panic::catch_unwind(|| prove_tx(txids[0].as_slice(), &expected_root_bytes, txids.clone()));
        match result {
            Ok((output, proof)) => {
                let is_valid = verify_tx(proof);
                println!("Block {}: Proof generated successfully", block_height);
                println!("Valid proof: {}", is_valid);
            }
            Err(_) => {
                println!("Block {}: Proof generation failed because tx does not match merke root!", block_height);
            }
        }
        let elapsed_time_tx = start_time_tx.elapsed();
        let elapsed_ms_tx = elapsed_time_tx.as_millis();
        println!("Tx & merkle root processed in {} ms", elapsed_ms_tx);

        // PoW check
        let bits: [u8; 4] = header.bits();
        println!("\nPoW check");
        let start_time_pow = Instant::now();
        let (prove_pow, verify_pow) = guest::build_check_proof_of_work();
        let result = std::panic::catch_unwind(|| prove_pow(&expected_hash, &bits.to_vec()));
        match result {
            Ok((output, proof)) => {
                let is_valid = verify_pow(proof);
                println!("Block {}: Proof generated successfully", block_height);
                println!("Valid proof: {}", is_valid);
            }
            Err(_) => {
                println!("Block {}: Proof generation failed because PoW in not valid!", block_height);
            }
        }
        let elapsed_time_pow = start_time_pow.elapsed();
        let elapsed_ms_pow = elapsed_time_pow.as_millis();
        println!("PoW processed in {} ms", elapsed_ms_pow);

        // Block header check
        println!("\nHeader hash check");
        let start_time_header = Instant::now();
        let (prove_hash, verify_hash) = guest::build_check_header_hash();
        let result = std::panic::catch_unwind(|| prove_hash(&expected_hash, &header.as_bytes()));
        match result {
            Ok((output, proof)) => {
                let is_valid = verify_hash(proof);
                println!("Block {}: Proof generated successfully", block_height);
                println!("Valid proof: {}", is_valid);
            }
            Err(_) => {
                println!("Block {}: Proof generation failed because hashes are different!", block_height);
            }
        }
        let elapsed_time_header = start_time_header.elapsed();
        let elapsed_ms_header = elapsed_time_header.as_millis();
        println!("Header processed in {} ms", elapsed_ms_header);

        let elapsed_time = start_time.elapsed();
        let elapsed_ms = elapsed_time.as_millis();
        println!("Block {} processed in {} ms", block_height, elapsed_ms);
        
        total_time += elapsed_ms;

        prev_block_hash = Some(block.hash.clone());
        prev_block_height = Some(block.height);
    }

    println!("\nTotal time for processing all blocks: {} ms", total_time);
}
