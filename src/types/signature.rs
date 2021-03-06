use AmanitaMuscaria::schnorr::schnorr::*;
use crate::canary_errors::CanaryErrors;

use base32::*;
use hex::*;

use serde::{Serialize,Deserialize};

/// A CanarySignature is the signature encoded in base58 and the message.
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct CanarySignature(pub String,pub String);

impl CanarySignature {
    /// Verify using **Base32 (Crockford)**
    pub fn verify(&self, pk: String) -> bool {
        let decoded_key_result = base32::decode(Alphabet::Crockford, &pk);

        let decoded_key = match decoded_key_result {
            Some(v) => v,
            None => return false
        };
        
        let pk_in_hex = hex::encode_upper(decoded_key);
        
        let sig = SchnorrSignature::from_encoding(pk_in_hex, self.0.clone(), b"CanaryKeys", self.1.as_bytes());
        let is_valid = sig.verify();

        return is_valid

    }
    /// Sign using Schnorr Keypair and Message
    pub fn sign(keypair: SchnorrKeypair, message: String) -> Result<Self,CanaryErrors> {
        let sig = keypair.sign(b"CanaryKeys",message.as_bytes());

        let message = String::from_utf8(sig.return_message());
        let signature_in_base58 = sig.return_signature_as_base58();

        let msg_output = match message {
            Ok(v) => v,
            Err(_) => return Err(CanaryErrors::FailedToVerifySchnorrSignature)
        };


        return Ok(
            Self(signature_in_base58,msg_output)
        )
    }
}