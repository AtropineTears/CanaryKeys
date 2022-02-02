//! # Web Of Trust
//! 
//! This document displays information related to the Web of Trust implemented in this project.
//! 
//! ## How It Works
//! 
//! A user uses a Schnorr Keypair to sign an account if they are trustworthy

pub struct WebOfTrust {

}

pub struct WebOfTrustSignature {
    pk: String, // Hex
    signature: String, // Base58    
    pow_nonce: u128,
}