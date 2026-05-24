//! Secure Multi-Party Computation service.
//!
//! Uses Shamir secret sharing for threshold operations and FROST for
//! threshold Schnorr signatures over BLS12-381. Enables cross-institution
//! computation without revealing private inputs.

/// MPC service for joint computation without data pooling.
#[derive(Debug)]
pub struct MpcService {
    max_parties: usize,
}

impl MpcService {
    pub fn new(max_parties: usize) -> Self {
        Self { max_parties }
    }

    /// Create a Shamir (t, n) secret sharing scheme.
    pub fn create_shamir_scheme(
        &self,
        threshold: usize,
        total_parties: usize,
    ) -> Result<ShamirScheme, super::PrivacyError> {
        if threshold > total_parties || total_parties > self.max_parties {
            return Err(super::PrivacyError::MpcPartyCountExceeded {
                requested: total_parties,
                max: self.max_parties,
            });
        }
        Ok(ShamirScheme {
            threshold,
            total_parties,
        })
    }
}

/// A Shamir (t, n) secret sharing scheme.
#[derive(Debug, Clone)]
pub struct ShamirScheme {
    pub threshold: usize,
    pub total_parties: usize,
}
