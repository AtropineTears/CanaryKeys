use blake2_rfc::blake2b::blake2b;

pub enum PivotPointType {
    DefaultStorage, // Default Storage of Data
    Blog, // A Blogging Platform
    PublicSpace, // A Public Space where users can talk
    Messaging, // A Messaging Platform
    SmartContract, // A Smart Contract
    Funding, // A Platform for raising funds
    CommunityProject, // A Community Project

    DelegationPage, // A Delegate Page For Delegates to Advertise
    Advertisement, // A page for advertisements

    Collection, // A Collection of Addresses, Pivot Points,

    WebsiteStyle, // A Style For Pages (CSS Code)
    Website, // A Website
    OpenDatabase, // A Database
    Learning, // A Place To Learn
}

/// 48 byte string encoded in base32
/// u16 is the Pivot Point Type
/// # Format
/// 
/// pivotpointinbase32:number
pub struct PivotPointLink(String,u16);

pub struct PivotPointCreators {
    creation_block: String,
    
    schnorr_public_keys: Vec<String>, // List of Schnorr Public Keys
    falcon1024_public_keyes: Vec<String>, // List of Falcon1024 Public Keys (Post-Quantum)
    number_of_keys: u64,
}

pub struct PivotPointUsers {
    schnorr_public_keys: Vec<String>,
}

pub struct PivotPointRules {
    is_private: bool, // Only allows creators to post to the pivot point
    allow_messages: bool,
    
}

pub struct PivotPointIndex {
    list_of_inputs: Vec<(String,String)>,
    num_of_points: u64,
}

pub struct PivotPointID(u64,String);