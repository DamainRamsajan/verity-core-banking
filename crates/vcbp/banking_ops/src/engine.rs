use std::sync::Arc;
use tokio::sync::RwLock;
use super::operations::BankingOperation;
use super::tokens::TokenOntology;
use super::dual_control::DualControlEnforcer;
use super::errors::BankingOpsError;
use vaos_core::types::CapabilityToken;

pub struct BankingOpsEngine {
    ontology: TokenOntology,
    dual_control_threshold: rust_decimal::Decimal,
    stats: RwLock<BankingOpsStats>,
}

#[derive(Debug, Default, Clone)]
pub struct BankingOpsStats {
    pub operations_processed: u64,
    pub dual_control_checks: u64,
    pub operations_rejected: u64,
}

impl BankingOpsEngine {
    pub fn new() -> Self {
        Self {
            ontology: TokenOntology::new(),
            dual_control_threshold: rust_decimal::Decimal::new(10000, 0),
            stats: RwLock::new(BankingOpsStats::default()),
        }
    }

    pub async fn execute(&self, operation: &BankingOperation, tokens: &[CapabilityToken]) -> Result<(), BankingOpsError> {
        let op_type = operation.operation_type();
        let mut stats = self.stats.write().await;
        let required_scopes = self.ontology.get_required_scopes(op_type)
            .ok_or_else(|| BankingOpsError::UnsupportedOperation(op_type.to_string()))?;

        if tokens.len() < required_scopes.len() {
            stats.operations_rejected += 1;
            return Err(BankingOpsError::DualControlRequired { operation: op_type.to_string(), required: required_scopes.len(), provided: tokens.len() });
        }

        if required_scopes.len() > 1 {
            DualControlEnforcer::verify(tokens, required_scopes.len())?;
            stats.dual_control_checks += 1;
        }

        if operation.amount() >= self.dual_control_threshold && tokens.len() < 2 {
            stats.operations_rejected += 1;
            return Err(BankingOpsError::DualControlRequired { operation: op_type.to_string(), required: 2, provided: tokens.len() });
        }

        stats.operations_processed += 1;
        Ok(())
    }
}
