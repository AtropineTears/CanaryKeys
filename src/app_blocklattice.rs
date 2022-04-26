//! # CanaryKeys
//! 
//! ## 

use crate::types::address::CanaryAddress;
use crate::types::base::block_hash::BlockHash;


//=====APP=====//
pub struct CanaryKeysApp {
    pub layer: CanaryLayerType,
    
    pub accounts: HashMap<CanaryAddress,AccountChainPrimary>,
    pub validators: HashMap<CanaryAddress,,
}

//=====ROOT INDEX====//
pub struct CanaryAccountIndex {
    pub chains: Vec<u16>,
}


//=====Account Chain (Transactions)=====//
pub struct AccountChainPrimary {
    pub address: CanaryAddress,
    
    // Transaction Blocks
    pub tx_blocks: Vec<TransactionBlock>,

    // Representative
    pub 
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


/// # Block Layout
/// 
/// The Block Layout sets the stage for other blocks. It contains the basics that most blocks have.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq,PartialOrd,Hash)]
pub struct BlockLayout {
    // Block ID
    pub id: u64,
    
    // Hash
    pub hash: BlockHash,
    pub previous_hash: BlockHash,

    // Time
    pub timestamp: i64,

    // Nonce
    pub nonce: u64,
}