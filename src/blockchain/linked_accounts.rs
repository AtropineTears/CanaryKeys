use std::collections::HashMap;
use url::Url;

/// # LinkedAccounts
/// 
/// A Hashmap that lets you add any types of accounts
/// 
/// ## List of Supported
/// 
/// Github
pub struct LinkedAccounts {
    // Account, (Username,URL)
    linked_accounts: HashMap<String,(String,Url)>
}