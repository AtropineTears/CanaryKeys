use serde::{Serialize,Deserialize};
use crate::types::traits::ValidateCanaryType;
use CanaryValidationLib::CanaryValidateEncodingAPI;
use blake2_rfc::blake2b::*;
use hex::*;
use crate::canary_errors::CanaryErrors;

/// # types/address/ CanaryAddress
/// 
/// A CanaryAddress is an address used in a block lattice.
/// 
/// There are multiple methods that exist on this type.
/// 
/// ## Methods
/// 
/// ### Validation
/// 
/// - validate_length()
/// 
/// - validate_format()
/// 
/// ### Checksums
/// 
/// - checksum_8()
/// 
/// - checksum_256()
/// 
/// - keyed_checksum_256(key: &[u8])
/// 
/// ### Compare
/// 
/// - compare_with_expected_address()
/// 
/// ### Encoding
/// 
/// - to_bytes()
#[derive(Debug,Clone,Hash,PartialEq,Eq,Serialize,Deserialize)]
pub struct CanaryAddress(pub String);

impl ValidateCanaryType for CanaryAddress {
    fn validate_length(&self) -> bool {
        if self.0.len() == 64usize {
            return true
        }
        else {
            return false
        }
    }
    /// Validates Address Is In Hexadecimal
    fn validate_format(&self) -> bool {
        CanaryValidateEncodingAPI::hex(&self.0)
    }
    fn validate(&self) -> bool {
        let format_bool = self.validate_format();
        let length_bool = self.validate_length();

        if format_bool == true && length_bool == true {
            return true
        }
        else {
            return false
        }
    }
}

impl CanaryAddress {
    pub fn keyed_checksum_32(&self, key: &[u8]) -> String {
        return hex::encode_upper(blake2b(32, key, self.0.as_bytes()))
    }
    pub fn checksum_32(&self) -> String {
        return hex::encode_upper(blake2b(32,&[], self.0.as_bytes()))
    }
    /// Creates a blake2b checksum (8 bytes) from the hexadecimal formatted string. With encoding, it is 16 bytes.
    pub fn checksum_8(&self) -> String {
        return hex::encode_upper(blake2b(8, &[], self.0.as_bytes()))
    }
    /// Compares two addresses with each other to see if they match
    pub fn compare_with_expected_address(&self,expected: String) -> bool {
        if self.0 == expected {
            return true
        }
        else {
            return false
        }
    }
    /// Converts Address To Bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>,CanaryErrors>{
        let x = hex::decode(self.0.clone());

        match x {
            Ok(v) => return Ok(v),
            Err(_) => return Err(CanaryErrors::FailedToDecodeFromHex),
        };
    }
}