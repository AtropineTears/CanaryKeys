//! # Rules
//! 
//! 1. All Accounts can
//!     - Send and Receive Money
//!     - Vote
//!     - Get Signed By Users using Web of Trust
//!     - 

use crate::types::address::CanaryAddress;

pub struct AccountIndex(Vec<String>);

pub enum AccountTypes {
    User0, // Default Account; Used to vote, send txs, get signed, and others
    Organization1, // An Organization 
    
    Delegate2, // An account that validates blocks
    Collection3, // A Collection of Accounts
    CertAuth4, // A Certificate Authority
    
    Contract5, // A Smart Contract
    StaticContract6, // Like a smart contract but free and performs static things
    
    Community7,
}

/// # Developer Notes
/// 
/// Require a vote to use a Canary Account
pub struct CanaryAccountRoot {
    // Account Type
    account_type: u8,
    address: CanaryAddress, // Public Key (base32)
    representative: CanaryAddress, // Who this account voted for; Must vote when account is opened
    is_delegate: bool,

        // Global to all accounts
        information_hash: Option<String>, // Can Change

        

            user_account: Option<String>, // 1
            organization_account: Option<String>, // 2
            delegate_account: Option<String>, // 3



    pow_nonce: u128, // pow

    signature: String, // Signature of Public Key + Description

}


/*
pub struct CanaryAccountTransactions {
    pub blocks: Vec<>
}
*/

pub struct Block {

}

/// # Canary Account Balance
/// 
/// This represents the balance available for a Canary Account
pub struct AccountBalance(u128);

pub struct TokenBalance();