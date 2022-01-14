pub enum CanaryErrors {
    // Encoding
    FailedToDecodeFromHex,
    FailedToDecodeFromBase32,
    FailedToDecodeFromBase58,

    // Validation
    FailedToValidatePublicKey,
    FailedToValidateAddress,
}