use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A banking product compiled from ASL source code.
///
/// Products are immutable once compiled — any change requires
/// re‑compilation and re‑verification of all safety invariants.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankingProduct {
    /// Unique product identifier
    pub id: Uuid,
    /// Product name (e.g., "Premium Checking")
    pub name: String,
    /// ASL source code that defined this product
    pub asl_source: String,
    /// Compiled bytecode for seedvm execution
    pub bytecode: Vec<u8>,
    /// Regulatory invariants verified at compile time
    pub verified_invariants: Vec<String>,
    /// Version of the ASL compiler used
    pub compiler_version: String,
    /// When the product was compiled
    pub compiled_at: chrono::DateTime<chrono::Utc>,
    /// Temporal contracts enforced by the product
    pub temporal_contracts: Vec<super::TemporalContract>,
    /// Whether the product passed all verification
    pub verified: bool,
}

impl BankingProduct {
    /// Verify that the product satisfies all declared invariants.
    /// Returns Ok if all checks pass, or a ProductError with details.
    pub fn verify(&self) -> Result<(), super::ProductError> {
        if !self.verified {
            return Err(super::ProductError::VerificationFailed(
                "Product has not been verified".into()
            ));
        }

        // Check temporal contracts
        for contract in &self.temporal_contracts {
            contract.verify()?;
        }

        Ok(())
    }
}
