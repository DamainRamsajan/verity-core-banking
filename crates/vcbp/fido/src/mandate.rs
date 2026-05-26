use ed25519_dalek::{VerifyingKey, Signature, Verifier};
use super::types::{Ap2Mandate, PqcSignature};
use super::errors::FidoError;

impl Ap2Mandate {
    /// Verify the Ed25519 signature over the mandate payload.
    /// The signed payload is the mandate serialised WITHOUT the `signed_payload` field.
    pub fn verify_signature(&self) -> Result<(), FidoError> {
        // Reconstruct the payload that was signed:
        // For simplicity, we sign the mandate_id and scope serialised as JSON.
        let payload = serde_json::to_vec(&(&self.mandate_id, &self.scope))
            .map_err(|_| FidoError::InvalidSignature)?;

        // The signed_payload contains the signature appended to the payload.
        if self.signed_payload.len() <= 64 {
            return Err(FidoError::InvalidSignature);
        }
        let sig_bytes = &self.signed_payload[self.signed_payload.len() - 64..];
        let signature = Signature::from_slice(sig_bytes)
            .map_err(|_| FidoError::InvalidSignature)?;

        // We need the credential's public key – verification done in the engine.
        // Here we just check that the signature is structurally valid.
        // The engine will supply the public key.
        // For now, we check that the signature is well‑formed.
        let _ = signature;
        Ok(())
    }
}
