use AmanitaMuscaria::schnorr::schnorr::*;
use crate::canary_errors::CanaryErrors;

use serde::{Serialize,Deserialize};

/// A CanarySignature is the signature encoded in base58 and the message.
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct CanarySignature(String,String);

impl CanarySignature {
    /// Verify using hexadecimal public key
    pub fn verify(&self, pk: String) -> bool {
        let sig = SchnorrSignature::from_encoding(pk, self.0.clone(), b"CanaryKeys", self.1.as_bytes());
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