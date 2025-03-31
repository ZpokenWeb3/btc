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
        println!("Processing block: {}", block_filename);
        
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
                        let output_hash = hex::encode(reverse(&output));
                        println!("Output prev_hash: {:#?}", output_hash);
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

        println!("\nHeader hash check");
        let (prove_hash, verify_hash) = guest::build_check_header_hash();
        let result = std::panic::catch_unwind(|| prove_hash(&expected_hash, &header.as_bytes()));
        match result {
            Ok((output, proof)) => {
                let is_valid = verify_hash(proof);
                let output_hash = hex::encode(reverse(&output));
                println!("Block {}: Proof generated successfully", block_height);
                println!("Output hash: {:#?}", output_hash);
                println!("Valid proof: {}", is_valid);
            }
            Err(_) => {
                println!("Block {}: Proof generation failed because hashes are different!", block_height);
            }
        }

        let elapsed_time = start_time.elapsed();
        let elapsed_ms = elapsed_time.as_millis();
        println!("Block {} processed in {} ms", block_height, elapsed_ms);
        
        total_time += elapsed_ms;

        prev_block_hash = Some(block.hash.clone());
        prev_block_height = Some(block.height);
    }

    println!("\nTotal time for processing all blocks: {} ms", total_time);
}
