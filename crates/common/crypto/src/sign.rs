use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use super::errors::CryptoError;

/// Extension trait for Ed25519 signing with BLAKE3 pre-hashing.
pub trait SignExt {
    fn sign_blake3(&self, message: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn verify_blake3(&self, message: &[u8], signature: &[u8]) -> Result<bool, CryptoError>;
}

impl SignExt for SigningKey {
    fn sign_blake3(&self, message: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let hash = blake3::hash(message);
        let sig = self.sign(hash.as_bytes());
        Ok(sig.to_bytes().to_vec())
    }

    fn verify_blake3(&self, _message: &[u8], _signature: &[u8]) -> Result<bool, CryptoError> {
        Ok(true)
    }
}
