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
//! 
//! ## AmanitaNet

// Serialization
use crate::types::traits::ValidateCanaryType;
use serde::{Serialize,Deserialize};

#[macro_use]
use serde_json::*;

// Time
use chrono::*;

// HashMap
use std::collections::HashMap;


// Crypto
use blake2_rfc::blake2b::*;
use AmanitaMuscaria::schnorr::schnorr::{SchnorrKeypair,SchnorrSignature};
use AmanitaMuscaria::seed::*;

// Types
use crate::types::address::CanaryAddress;
use crate::types::currency::{CanaryCurrency,CanaryTokenAbbreviation,CanaryTokens};
use crate::types::layer::CanaryLayerType;
use crate::types::base::description::CanaryDescription;
use crate::types::signature::CanarySignature;
use crate::types::account_types::CanaryAccountTypes;

use crate::types::base::block_hash::BlockHash;

use crate::consensus::pow::{ProofOfWorkAPI,VerifyNonce};

use crate::constants::CONSENSUS_3;

//==========BASE BLOCKCHAIN==========//
// The Base Blockchain contains the following:
// - CanaryAccounts
    // - Account ID
    // - Description
    // - Links to other addresses on other blockchains

// Add to Block current state of all addresses

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct CanaryAccountsBlockchain {
    pub layer: CanaryLayerType,
    pub blocks: Vec<CanaryAccountsBlock>,
    
    // Locate an Addresses Block
    pub addresses: HashMap<CanaryAddress,u64>,
    //pub hash: Vec<String>
}


impl CanaryAccountsBlockchain {
    /// # Generate A New CanaryAccountsBlockchain
    /// 
    /// This will create a new CanaryAccountsBlockchain using the layer you specify.
    pub fn new(layer: CanaryLayerType) -> Self {
        return Self {
            layer: layer,
            blocks: vec![],
            addresses: HashMap::new(),
        }
    }
    pub fn genesis(&mut self){
        let genesis_block = CanaryAccountsBlock::genesis_of_chain();

        self.blocks.push(genesis_block)
    }
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct CanaryAccountsBlock {
    //pub basics: BlockLayout,
    pub block_id: u64,
    pub previous_hash: BlockHash,
    pub timestamp: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option< Vec<CanaryAccountTransaction> >,
    
    pub hash: BlockHash,
    pub nonce: u128,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct CanaryAccountTransaction {    
    pub address: CanaryAddress,
    pub description: CanaryDescription,
    pub account_type: CanaryAccountTypes,
    
    pub pow: u128,
    pub signature: CanarySignature,
}

impl CanaryAccountTransaction {
    /// # Create Transaction
    /// 
    /// This function creates a transaction on the base blockchain. The Proof of Work (nonce) is done with the address used as input.
    pub fn create_transaction(keypair: SchnorrKeypair,description: CanaryDescription, account_type: CanaryAccountTypes) -> Self {
        let address = keypair.return_pk_as_base32();

        let final_address = CanaryAddress(address);

        let address_in_bytes = final_address.to_bytes();

        let add_in_bytes = match address_in_bytes {
            Ok(v) => v,
            Err(_) => panic!("Failed To Sign Transaction Due To Address Not Converting To Bytes")
        };

        let nonce: u128 = ProofOfWorkAPI::new(add_in_bytes,CONSENSUS_3);

        if final_address.validate() == false {
            panic!("Could Not Validate Address")
        }

        // TODO: Sign Address, Description, Account Type, and PoW
        // TODO: Change Account Type to Account Implementation Using a 16 byte string!!!!!!!!

        let hash = Self::calculate_hash_for_signing(final_address.clone(), description.clone(), account_type.clone(), nonce);


        // TODO: Change
        let signature_result = CanarySignature::sign(keypair, hash);

        let signature = match signature_result {
            Ok(v) => v,
            Err(_) => panic!("Failed To Sign Transaction Due To Failed Signature")
        };

        Self {
            address: final_address,
            description: description,
            account_type: account_type,
            pow: nonce,
            
            signature: signature,
        }

    }
    // TODO: Fix validation of address by using Base32 Crockford regular expression
    pub fn verify_tx(&self) -> bool {
        if self.address.validate() == false || self.description.validate_length() == false {
            return false
        }
        else {
            // Deal with Proof of Work
            let address = self.address.to_bytes();

            let output_address_in_bytes = match address {
                Ok(v) => v,
                Err(_) => return false
            };

            // POW Difficulty
            let pow_difficulty = VerifyNonce::new(output_address_in_bytes, self.pow);

            if pow_difficulty < 3u8 {
                return false
            } 

            // Verify Signature
            if self.signature.verify(self.address.0.clone()) == true {
                return true
            }
            else {
                return false
            }
        }
    }
    /// Calculate Hash Using Blake2b (48 bytes) for signing
    fn calculate_hash_for_signing(address: CanaryAddress, description: CanaryDescription, account_type: CanaryAccountTypes, pow: u128) -> String {
        let data = serde_json::json!(
            {
            "CanaryAddress": address,
            "Description": description,
            "Account_Type": account_type,
            "PoW": pow,
            }
        );

        let hash = blake2b(48,&[],data.to_string().as_bytes());

        return hex::encode_upper(hash.as_bytes())
    }
}

impl CanaryAccountsBlock {
    pub fn genesis_of_chain() -> Self {
        // id = 0
        // hash_of_block

        let genesis_block = CanaryAccountsBlock { 
            block_id: 0u64, 
            previous_hash: BlockHash::genesis_hash(), 
            timestamp: Utc::now().timestamp(), 
            transactions: None, 
            hash: BlockHash::genesis_hash(), 
            nonce: 0u128,
        };

        return genesis_block
    }
    pub fn genesis() {
        // BASICS
            // id
            // hash of block
            // previous_hash
            // timestamp
            // nonce
        
        //let pk = keypair.return_pk_as_hex();


        let id: u64 = 0;
        let previous_hash: &str = "Genesis Block. Welcome To CanaryKeys.";

        // Hash Block
        let mut context = Blake2b::new(32);


        // Canary Signature
        //let canary_sig = CanarySignature::sign(keypair, message: String);

        //let basics = BlockLayout::new(0u64, "Genesis", "Genesis", 0u128);
    }
}


//==========LATTICE==========//

pub struct CanaryApp {
    pub layer: CanaryLayerType,
    
    pub accounts: HashMap<CanaryAddress,AccountChainPrimary>,
    //pub accounts_other: Option<HashMap<CanaryAddress,AccountChain>>,

    pub vm_state: bool,
}

impl CanaryApp {
    pub fn new() {

    }
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

impl BlockLayout {
    pub fn new<T: AsRef<str>>(id: u64, hash: T, previous_hash: T,nonce: u128) -> Self {
        Self {
            id: id,
            hash: hash.as_ref().to_string(),
            previous_hash: previous_hash.as_ref().to_string(),
            timestamp: Utc::now().timestamp(),
            nonce: nonce,
        }
    }
}

//==========PRIMARY CHAIN FOR TRANSACTIONS AND VOTING==========//
pub struct AccountChainPrimary {
    account_id: u64,
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

    pub from: Option<CanaryAddress>,
    pub to: Option<CanaryAddress>,

    pub representative: CanaryAddress,
}


//==========PIVOT POINT BLOCK==========//

pub struct PivotPointBlock {
    key: String,
    layer: String,
}


#[test]
fn create_new_blockchain(){
    let mut blockchain = CanaryAccountsBlockchain::new(CanaryLayerType::default_main());
    blockchain.genesis();

    println!("{:?}",blockchain.blocks);
}