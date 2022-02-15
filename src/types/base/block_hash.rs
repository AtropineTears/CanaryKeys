use serde::{Serialize,Deserialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct BlockHash(String);

impl BlockHash {
    pub fn genesis_hash() -> Self {
        let block_hash: Self = BlockHash(String::from("5741c8e332bb88ca066326207be10d230c504725c4828266835ee895c0f2f123"));
        return block_hash
    }
}