use blake2_rfc::blake2b::*;
use crate::utils::pow_to_bytes::NonceConversion;
use crate::constants::{CONSENSUS_DEFAULT_VALUE_TEN_SECONDS_1,CONSENSUS_2,CONSENSUS_3,CONSENSUS_4};

/// # Proof of Work API
/// 
/// This struct represents PoW nonces. The difficulty is the number of bits in front of the hash.
/// 
/// ## How To Do Proof of Work
/// 
/// ```
/// 
/// fn main(){
///     ProofOfWorkAPI::new(b"Hello World",CONSENSUS_DEFAULT_VALUE_TEN_SECONDS_1);
/// }
/// 
/// ```
pub struct ProofOfWorkAPI;

pub struct ProofOfWork64API;

pub struct VerifyNonce;

impl VerifyNonce {
    pub fn new<T: AsRef<[u8]>>(input: T, nonce: u64) -> u8 {
        let mut bytes: Vec<u8> = input.as_ref().to_vec();
        let mut nonce_as_bytes = NonceConversion::to_bytes_u64(&vec![nonce]);

        bytes.append(&mut nonce_as_bytes);

        let output_hash = blake2b(8, &[], &bytes);

        let hash = output_hash.as_bytes();

        if hash < &CONSENSUS_DEFAULT_VALUE_TEN_SECONDS_1.to_be_bytes() {
            return 0u8 // Failure
        }
        else if hash >= &CONSENSUS_4.to_be_bytes() {
            return 4u8
        }
        else if hash >= &CONSENSUS_3.to_be_bytes() {
            return 3u8
        }
        else if hash >= &CONSENSUS_2.to_be_bytes() {
            return 2u8
        }
        else if hash >= &CONSENSUS_DEFAULT_VALUE_TEN_SECONDS_1.to_be_bytes() {
            return 1u8
        }
        else {
            return 0u8
        }

    }
}

impl ProofOfWorkAPI {
    pub fn new<T: AsRef<[u8]>>(input: T, threshold: u64) -> u128 {
        let mut nonce: u128 = 0u128;
        let bytes: Vec<u8> = input.as_ref().to_vec();

        loop {
            // Initialize Bytes With Nonce
            let mut bytes_with_nonce: Vec<u8> = bytes.clone();

            // Convert Nonce To Bytes
            let mut nonce_bytes: Vec<u8> = NonceConversion::to_bytes_u128(&vec![nonce]);

            // Append Nonce To Input Bytes
            bytes_with_nonce.append(&mut nonce_bytes);
            // Hash Input with Nonce
            let output_hash = blake2b(8, &[], &bytes_with_nonce);
            
            if output_hash.as_bytes() >= &threshold.to_be_bytes() {
                return nonce
            }
            else {
                println!("Nonce: {}",nonce);
                nonce += 1u128;
            }
        }
    }
}

impl ProofOfWork64API {
    pub fn new<T: AsRef<[u8]>>(input: T, threshold: u64) -> u64 {
        let mut nonce: u64 = 0u64;
        let bytes: Vec<u8> = input.as_ref().to_vec();

        loop {
            // Initialize Bytes With Nonce
            let mut bytes_with_nonce: Vec<u8> = bytes.clone();

            // Convert Nonce To Bytes
            let mut nonce_bytes: Vec<u8> = NonceConversion::to_bytes_u64(&vec![nonce]);

            // Append Nonce To Input Bytes
            bytes_with_nonce.append(&mut nonce_bytes);
            // Hash Input with Nonce
            let output_hash = blake2b(8, &[], &bytes_with_nonce);
            
            if output_hash.as_bytes() >= &threshold.to_be_bytes() {
                return nonce
            }
            else {
                println!("Nonce: {}",nonce);
                nonce += 1u64;
            }
        }
    }
}

#[test]
fn test_pow(){
    ProofOfWorkAPI::new("Hello World!", 0xffffc00000000000);
}

#[test]
fn test_verification(){
    let x = VerifyNonce::new("Hello World!", 319625u64);
    println!("Number: {}",x);
}