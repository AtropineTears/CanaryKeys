pub fn hash_to_binary_representation(hash: String) -> String {
    // TODO: Handle Error
    let decoded_hex = hex::decode(hash).expect("Failed To Decode From Hex");
    
    let mut res: String = String::default();
    for c in decoded_hex {
        res.push_str(&format!("{:b}", c));
    }
    res
}