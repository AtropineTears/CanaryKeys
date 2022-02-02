//! # Chains
//! There are multiple chains for one account. Here are the following chains for an account.
//! 
//! ## Transaction Chain
//! 
//! The **Transaction Chain** is where transactions occur. This can be used to make payments, fund projects, send donations and other stuff.
//! 
//! Tokens are also available on this chain.
//! 
//! ## Voting Chain
//! 
//! The **Voting Chain** consists of anything related to being a delegate or casting your votes
//! 
//! ## Certificate Chain
//! 
//! The certificate chain is used for CanaryCerts. It consists of WoT PKI and DPOS
//! 
//! ## Social Chain
//! 
//! The social chain tracks changes to the social media links that one chooses to publish.
//! 
//! ## The Project Chain
//! 
//! The project chain consists of sponsored projects that are available to the end user.
//! 
//! ## Pivot Point
//! 
//! An extendable community or extension that accepts multiple different implementations for developers to easily extend on.
//! 
//! A Pivot Point consists of two things:
//! 
//! 1. The Key
//! 
//! 2. The Layer

// Serialization
use serde::{Serialize,Deserialize};

use std::collections::HashMap;

// Types
use crate::types::address::CanaryAddress;
use crate::types::currency::{CanaryCurrency,CanaryTokenAbbreviation,CanaryTokens};



pub struct CannaryApp {
    pub accounts: HashMap<CanaryAddress,AccountChain>,
    pub delegates: HashMap<CanaryAddress,>
    
    pub vm_state: bool,
}


/// # Block Layout
/// 
/// The Block Layout sets the stage for other blocks. It contains the basics that most blocks have.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq,PartialOrd,Hash)]
pub struct BlockLayout {
    // Block ID
    pub id: u64,
    
    // Hash
    pub hash: String,
    pub previous_hash: String,

    // Time
    pub timestamp: i64,

    // Nonce
    pub nonce: u128,
}


pub struct AccountChain {
    tx_blocks: Vec<TransactionBlock>,
}

//==========DELEGATE==========//
pub struct DelegateInformation {
    
}

pub struct DelegateAccount {

}


//==========TX BLOCK==========//
/// # Transaction Block
/// 
/// A Transaction Block lists the amount of currency and number of tokens
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct TransactionBlock {
    // Basics
    pub basics: BlockLayout,
    
    // Currency
    pub current_amount_of_currency: CanaryCurrency,
    pub current_amount_of_tokens: CanaryTokens,
}

pub struct CanaryBlockchain {
    tokens_chain: Vec,
    dns_chain: Vec,
}

pub struct PivotPointBlock {
    key: String,
    layer: String,
}