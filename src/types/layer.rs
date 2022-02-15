use serde::{Serialize,Deserialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct CanaryLayerType(String,bool);

impl CanaryLayerType {
    pub fn default_main() -> Self {
        return Self(String::from("CanaryKeysMain"),false)
    }
    pub fn validate_length(&self) -> bool {
        if self.0.len() > 16 {
            return false
        }
        else {
            return true
        }
    }
    pub fn is_testnet(&self) -> bool {
        return self.1
    }
}