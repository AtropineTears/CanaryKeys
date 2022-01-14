pub struct Transaction {
    pub transaction_id: String,
    pub transaction_timestamp: i64,
    pub transaction_details: String,
}
pub struct Block {
    pub block_number: u64,
    block_timestamp: i64,
    pub block_nonce: u64,
    pub transaction_list: Vec<Transaction>,
    previous_block_hash: String,
}