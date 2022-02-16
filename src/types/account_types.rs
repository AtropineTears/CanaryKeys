use serde::{Serialize,Deserialize};

pub enum CanaryAccountsEnum {
    User,
    Organization,
    Community,
    Token,
}

#[derive(Debug,Clone,PartialEq,Hash,Serialize,Deserialize)]
/// An up to 24 byte string that defines the Account Type.
pub struct CanaryAccountTypes(String);

impl CanaryAccountTypes {
    pub fn new<T: AsRef<str>>(input: T) -> Self {
        return Self(input.as_ref().to_string())
    }
    pub fn validate_length(&self) -> bool {
        if self.0.len() >= 24usize {
            return false
        }
        else {
            return true
        }
    }
}