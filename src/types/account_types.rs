use serde::{Serialize,Deserialize};

#[derive(Debug,Clone,PartialEq,Hash,Serialize,Deserialize)]
pub enum CanaryAccountTypes {
    User, // Default
    Organization, // Controlled By Multiple
    Community, // Community Project
    Token, // Smart Contract
}