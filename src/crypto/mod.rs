use AmanitaMuscaria::seed::*;
use AmanitaMuscaria::schnorr::schnorr::*;

pub struct CanaryGenerateSeedAPI;

impl CanaryGenerateSeedAPI {
    pub fn generate_schnorr_keypair(){
        
    }
}

/*
impl CanaryGenerateSeedAPI {
    pub fn generate<T: AsRef<str>>(language: Language,password: T) -> &str {
        let secret_phrase = BIP39API::generate(language).export_phrase();
        return secret_phrase
        SecretPhrase::new(phrase: T)
    }
}
*/