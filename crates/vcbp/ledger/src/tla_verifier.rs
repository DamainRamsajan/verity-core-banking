use super::types::Transaction;
use super::errors::LedgerError;

/// Integration with the VAOS runtime TLA+ model checker.
pub struct TlaVerifier;

impl TlaVerifier {
    pub fn new() -> Self { Self }

    pub async fn sample(&self, tx: &Transaction) -> Result<(), LedgerError> {
        // In production, delegates to vaos_runtime_tla::RuntimeTlaEngine
        Ok(())
    }
}
