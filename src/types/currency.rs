use serde::{Serialize,Deserialize};

/// CanaryCurrency is the type for making transactions and sending tokens.
#[derive(Debug,Clone,Serialize,Deserialize,Hash,PartialEq,PartialOrd)]
pub struct CanaryCurrency(u128);

impl CanaryCurrency {
    pub fn new() -> Self {
        return Self {
            0: 0u128
        }
    }
    /// Has Amount checks to see if someone has a certain amount of the currency and can thus spend it.
    pub fn has_amount(&self, amount: u128) -> bool {
        if self.0 >= amount {
            return true
        }
        else {
            return false
        }
    }
}

/// CanaryTokens are tokens on the blockchain that can be sent.
#[derive(Debug,Clone,Serialize,Deserialize,Hash,PartialEq,PartialOrd)]
pub struct CanaryTokens(CanaryTokenAbbreviation,u128);

impl CanaryTokens {
    pub fn has_amount(&self, amount: u128) -> bool {
        if self.1 >= amount {
            return true
        }
        else {
            return false
        }
    }
}

#[derive(Debug,Clone,Serialize,Deserialize,Hash,PartialEq,PartialOrd)]
pub struct CanaryTokenAbbreviation(String);

impl CanaryTokenAbbreviation {
    pub fn validate_length(&self) -> bool {
        if self.0.len() > 2 && self.0.len() < 5 {
            return true
        }
        else {
            return false
        }
    }
}