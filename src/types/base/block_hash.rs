use serde::{Serialize,Deserialize};
use hex::*;
use crate::canary_errors::CanaryErrors;

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq,PartialOrd)]
pub struct BlockHash(pub String);

impl BlockHash {
    pub fn genesis_hash() -> Self {
        let block_hash: Self = BlockHash(String::from("5741c8e332bb88ca066326207be10d230c504725c4828266835ee895c0f2f123"));
        return block_hash
    }
    pub fn decode_from_hex(&self) -> Result<Vec<u8>,CanaryErrors> {
        let res = hex::decode(self.0);

        match res {
            Ok(v) => return Ok(v),
            Err(_) => return Err(CanaryErrors::FailedToDecodeFromHex)
        }
    }
}