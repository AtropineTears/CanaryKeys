/// # Certificate Algorithm
/// 
/// This enum represents different certificate algorithms.
#[derive(Debug,Clone,Copy,Hash,PartialEq,PartialOrd)]
pub enum CertificateAlgorithm {
    Schnorr,
    Falcon1024,
    Other,
}

/// # Canary Certificate
/// 
/// Simple certificate that uses "trust the public key" concept as opposed to using common names
pub struct CanaryCertificate {
    algorithm: CertificateAlgorithm,
    pk: String,
    
    verified_ethereum_address: Option<String>,
    github: Option<String>,


    signature: String,
}

pub struct CertificateLinkedAccounts {
    verified_ethereum_address: String,
}