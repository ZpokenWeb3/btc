use serde::{Deserialize, Serialize};
use crate::utils::reverse;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionInput {
    pub sequence: u32,
    pub witness: String,
    pub script: String,
    pub index: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionOutput {
    pub value: u64,
    pub script: String,
    pub addr: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: String,
    pub ver: u32,
    pub vin_sz: u32,
    pub vout_sz: u32,
    pub size: u32,
    pub weight: u32,
    pub fee: u64,
    pub time: u64,
    pub inputs: Vec<TransactionInput>,
    pub out: Vec<TransactionOutput>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub hash: String,
    pub height: u32,
    pub ver: u32,
    pub prev_block: String,
    pub mrkl_root: String,
    pub time: u32,
    pub bits: u32,
    pub nonce: u32,
    pub size: u32,
    pub weight: u32,
    pub tx: Vec<Transaction>,
}

#[derive(Debug)]
pub struct BTCHeader(pub [u8; 80]);

impl BTCHeader {
    pub fn as_bytes(&self) -> [u8; 80] {
        self.0
    }

    pub fn from_block(block: &Block) -> Self {
        let mut header = Vec::new();
        header.extend_from_slice(&block.ver.to_le_bytes());
        let prev_block_bytes = hex::decode(&block.prev_block).expect("Invalid hex string");
        let merkle_root_bytes = hex::decode(&block.mrkl_root).expect("Invalid hex string");
        header.extend_from_slice(&reverse(&prev_block_bytes));
        header.extend_from_slice(&reverse(&merkle_root_bytes));
        header.extend_from_slice(&block.time.to_le_bytes());
        header.extend_from_slice(&block.bits.to_le_bytes());
        header.extend_from_slice(&block.nonce.to_le_bytes());
        Self(header.try_into().expect("Error converting to BTCHeader"))
    }

    pub fn time(&self) -> u32 {
        u32::from_le_bytes(
            self.0[68..72]
                .try_into()
                .expect("Conversion should never fail"),
        )
    }

    pub fn bits(&self) -> [u8; 4] {
        self.0[72..76]
            .try_into()
            .expect("Conversion should never fail")
    }
    
    pub fn prev_hash(&self) -> [u8; 32] {
        self.0[4..36]
            .try_into()
            .expect("Conversion should never fail")
    }

    pub fn merkle_root(&self) -> [u8; 32] {
        self.0[36..68]
            .try_into()
            .expect("Conversion should never fail")
    }
}

impl AsRef<[u8]> for BTCHeader {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}