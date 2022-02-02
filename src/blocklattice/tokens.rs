pub struct Token {

}

pub struct TokenInformation {
    common_name: String, // 16 chars
    abbreviation: String, // 3-4 chars
    referenced_block_id: Option<String>,
    referenced_block_height: Option<String>,
}