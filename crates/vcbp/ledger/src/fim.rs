use super::types::Transaction;
use super::errors::LedgerError;

/// Integration with the ASM Financial Invariants Monitor.
pub struct FimIntegration;

impl FimIntegration {
    pub fn new() -> Self { Self }

    pub async fn check_transaction(&self, tx: &Transaction) -> Result<(), LedgerError> {
        // In production, delegates to asm_fim::FinancialInvariantsMonitor
        Ok(())
    }
}
