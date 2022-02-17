//! # Canary Services
//! 
//! Canary has multiple different chains that are available to different users. It is naturally extendable and can support the addition of new chains.
//! 
//! Each new chain has its own block and is indexed by its block id.
//! 
//! ## Availble Chains
//! 
//! - Transaction (Note: Required) | 0x00
//! 
//! - Account Management | 0x01 | Contains all of the other services
//! 
    //! - WebOfTrust | 0x02
    //! 
    //! - Voting and Delegation (CanaryDelegation) | 0x03
    //! 
    //! - Social | 0x04
    //! 
    //! - Reputation Service | 0x05 | A File Reputation Service
    //! 
    //! - CanarySecurity | 0x06 | Security by Canary
    //! 
    //! - Blogging | 0x07 | Blogging through IPFS/IPNS
    //! 
    //! - PivotPoint | 0x08
    //! 
    //! - CanaryVM | 0x09
    //! 
    //! - Secure Messagining | 0x0A
    //! 
    //! - Developer Tools | 0x0B
    //! 
    //! - Service Hosting | 0x0C | Host different websites over the blockchain
    //! 
    //! - Identity | 0x0D | Create an Identity
//! 

use serde::{Serialize,Deserialize};

/// CanaryImplementation is the services provided by the address.
#[derive(Debug,Clone,Copy,Serialize,Deserialize,Hash,PartialEq)]
pub struct CanaryServicesType(u16);