use blake2_rfc::blake2b::*;

pub struct CanaryHashLink {
    version: u8, // 0 (Test) | 1 (Alpha)
    id: u64,
    hash: String, // 32 bytes
    pivotpointlocation: String, // 48 bytes
}

#[derive(Debug,Clone,)]
pub struct HashLink(String);

/// # HashLinkImmutable
/// 
/// The HashLink Type is a type that represents a link to data by using two pieces of information:
/// 
/// 1. An unsigned 64-byte number (`u64`) representing where the data is stored
/// 2. A 48-byte string (encoded in hexadecimal making it 64 bytes) of the hash of the data
/// 3. 4-byte number representing the pivot point the data is stored at
/// 
/// It also requires knowing the Pivot Point that the data is stored at. An 8-byte checksum of the Pivot Point is included
/// 
/// This link is used to secure data in an immutable
pub struct HashLinkImmutable(u64,String,u64);

/// 
/// 
/// 1. An unsigned 64-byte number representing where the data is stored.
/// 
/// 2. A 48-byte hash of the public key used to update the page.
/// 
/// 3. 4-byte number representing the pivot point the data is stored at
pub struct HashLinkMutable(u64,String,u64);