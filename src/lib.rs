//! # Canary Keys (Core)
//! 
//! This library features the core features of the Canary Blockchain
//! 
//! 

extern crate CanaryValidationLib;

/// Folder for different constants involved in different areas
pub mod constants;
/// File that lists Canary Errors
pub mod canary_errors;

/// Utilities
pub mod utils;

/// Consensus Mechanisms
pub mod consensus;

/// The BlockLattice Implementation
pub mod blocklattice;

pub mod app;

pub mod types;
pub mod pivotpoint;
pub mod hash_links;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
