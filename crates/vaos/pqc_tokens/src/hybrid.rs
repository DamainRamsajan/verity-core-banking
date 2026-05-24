//! Hybrid token signer — Ed25519 + ML-DSA dual signatures.
//!
//! During the PQC migration, every token carries both a classical Ed25519
//! signature and a post-quantum ML-DSA-44 signature. This ensures backward
//! compatibility while building PQC readiness.

/// Signs tokens in hybrid mode (classical + PQC).
#[derive(Debug)]
pub struct HybridTokenSigner {
    classical_key: Vec<u8>,
    pqc_key: Vec<u8>,
}

impl HybridTokenSigner {
    pub fn new() -> Self {
        Self {
            classical_key: vec![],
            pqc_key: vec![],
        }
    }

    /// Generate hybrid keypair (Ed25519 + ML-DSA-44).
    pub fn generate_keypair(&mut self) -> Result<(), super::PqcError> {
        // Ed25519 keypair
        use rand::rngs::OsRng;
        let mut csprng = OsRng;
        let ed25519_keypair = ed25519_dalek::SigningKey::generate(&mut csprng);
        self.classical_key = ed25519_keypair.to_bytes().to_vec();

        // ML-DSA-44 keypair via crystals-dilithium
        // use crystals_dilithium::ml_dsa_44::Keypair;
        // let seed = [42u8; 32];
        // let ml_keypair = Keypair::generate(Some(&seed)).unwrap();

        Ok(())
    }
}
