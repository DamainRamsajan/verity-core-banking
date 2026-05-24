use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankingProduct {
    pub id: Uuid,
    pub name: String,
    pub asl_source: String,
    pub bytecode: Vec<u8>,
    pub verified_invariants: Vec<String>,
    pub compiler_version: String,
    pub compiled_at: chrono::DateTime<chrono::Utc>,
    pub temporal_contracts: Vec<super::TemporalContract>,
    pub verified: bool,
}

impl BankingProduct {
    pub fn verify(&self) -> Result<(), super::ProductError> {
        if !self.verified {
            return Err(super::ProductError::VerificationFailed("Product has not been verified".into()));
        }
        for contract in &self.temporal_contracts {
            contract.verify()?;
        }
        Ok(())
    }
}
