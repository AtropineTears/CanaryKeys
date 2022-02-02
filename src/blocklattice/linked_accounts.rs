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
    linked_accounts: HashMap<String,(String,Url)>,
}

impl LinkedAccounts {
    pub fn get_all_accounts(&self) -> (Vec<&str>,Vec<&str>,Vec<Url>) {
        // Key
        let mut services: Vec<&str> = vec![];
        
        // Username and Links
        let mut usernames: Vec<&str> = vec![];
        let mut links: Vec<Url> = vec![];
        
        for (k,v) in self.linked_accounts.iter() {
            services.push(&k);
            usernames.push(&v.0);
            links.push(v.1.clone());
        }

        return (services,usernames,links)
    }
}