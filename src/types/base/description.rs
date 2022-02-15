use serde::{Serialize,Deserialize};


/// A 512-byte description that is available on the blockchain
#[derive(Debug,Clone,Serialize,Deserialize,PartialEq,Hash)]
pub struct CanaryDescription(String);


impl CanaryDescription {
    pub fn validate_length(&self) -> bool {
        if self.0.len() > 512 {
            return false
        }
        else {
            return true
        }
    }
}