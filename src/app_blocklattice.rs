use crate::types::address::CanaryAddress;
use crate::types::base::block_hash::BlockHash;

pub struct CanaryKeysApp {
    pub layer: CanaryLayerType,
    
    pub accounts: HashMap<CanaryAddress,AccountChainPrimary>,
}

pub struct RootAccount {

}

pub struct AccountChainPrimary {
    pub address: CanaryAddress,
    pub tx_blocks: Vec<TransactionBlock>,
}

pub struct TransactionBlock {
    block_attributes: BlockLayout,
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