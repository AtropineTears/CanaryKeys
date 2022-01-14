use blake2::Blake2b::*;
use utils::pow_to_bytes::*;
use hex::*;

/// # Proof of Work API
/// 
/// This struct represents PoW nonces. The difficulty is the number of bits in front of the hash.
pub struct ProofOfWorkAPI;

impl ProofOfWorkAPI {
    pub fn new<T: AsRef<[u8]>>(input: T, difficulty: u128) {
        let mut nonce: u128 = 0u128;
        let bytes: Vec<u8> = input.as_ref();

        loop {
            // Initialize Hasher and Convert Nonce To Bytes
            let mut hasher = Blake2b::new();
            let nonce_bytes = NonceConversion::to_bytes_u128(nonce);

            // Append Nonce To Input Bytes
            let bytes_with_nonce: Vec<u8> = bytes.append(nonce_bytes);
            // Update Hasher
            hasher.update(bytes_with_nonce);
            // Get Hash In Bytes
            let output_hash_in_bytes = hasher.finalize();

            if output_hash_in_bytes[0..8]
            nonce += 1usize;
        }
        }
        input.as_ref()
    }
}