use AmanitaMuscaria::seed::*;
use AmanitaMuscaria::schnorr::schnorr::*;

pub struct CanaryGenerateSeedAPI;

impl CanaryGenerateSeedAPI {
    pub fn generate_test_schnorr_keypair() -> SchnorrKeypair {
        let secret_phrase = BIP39API::generate(Language::English);
        let x = secret_phrase.derive_seed("Test", Language::English);
        return SchnorrKeypair::new(x.as_bytes())
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