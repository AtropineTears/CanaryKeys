use crate::types::address::CanaryAddress;
use crate::types::base::block_hash::BlockHash;
use crate::types::layer::CanaryLayerType;

pub struct CanaryApp {
    layer: CanaryLayerType,
    blocks: 
}

pub struct TransactionBlock {
    id: u64,
    
    hash: BlockHash,
    previous_hash: BlockHash,
    
    timestamp: i64,
    nonce: u64,
        
    amount: u128,
    balance: u128,
}