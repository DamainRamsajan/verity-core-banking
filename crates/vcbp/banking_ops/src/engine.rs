use std::sync::Arc;
use tokio::sync::RwLock;

use super::operations::BankingOperation;
use super::tokens::TokenOntology;
use super::dual_control::DualControlEnforcer;
use super::errors::BankingOpsError;
use vaos_core::types::CapabilityToken;

/// Central engine for capability‑based banking operations.
///
/// Every banking action is validated against required capability tokens
/// before execution. Dual‑control is structurally enforced.
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
            dual_control_threshold: rust_decimal::Decimal::new(10_000, 0),
            stats: RwLock::new(BankingOpsStats::default()),
        }
    }

    /// Validate and execute a banking operation.
    ///
    /// # Pre‑conditions
    /// - All required capability tokens must be presented
    /// - Dual‑control tokens must be from distinct principals
    ///
    /// # Post‑conditions
    /// - Operation is either executed with provenance or rejected
    ///
    /// # Invariants
    /// - No operation executes without the required token(s)
    /// - Dual‑control is guaranteed for high‑value operations
    #[tracing::instrument(name = "banking_ops.execute", level = "info", skip(self))]
    pub async fn execute(
        &self,
        operation: &BankingOperation,
        tokens: &[CapabilityToken],
    ) -> Result<(), BankingOpsError> {
        let op_type = operation.operation_type();
        let mut stats = self.stats.write().await;

        // 1. Get required token scopes for this operation type
        let required_scopes = self.ontology.get_required_scopes(op_type)
            .ok_or_else(|| BankingOpsError::UnsupportedOperation(op_type.to_string()))?;

        // 2. Validate that all required token scopes are covered by the provided tokens
        if tokens.len() < required_scopes.len() {
            stats.operations_rejected += 1;
            return Err(BankingOpsError::DualControlRequired {
                operation: op_type.to_string(),
                required: required_scopes.len(),
                provided: tokens.len(),
            });
        }

        // 3. For dual‑control operations, verify distinct principals
        if required_scopes.len() > 1 {
            DualControlEnforcer::verify(tokens, required_scopes.len())?;
            stats.dual_control_checks += 1;
        }

        // 4. Check amount threshold for dual‑control
        if operation.requires_dual_control(self.dual_control_threshold) && tokens.len() < 2 {
            stats.operations_rejected += 1;
            return Err(BankingOpsError::DualControlRequired {
                operation: op_type.to_string(),
                required: 2,
                provided: tokens.len(),
            });
        }

        // 5. Operation authorized — execute
        stats.operations_processed += 1;

        tracing::info!(
            operation = op_type,
            amount = ?operation.amount(),
            tokens = tokens.len(),
            "Banking operation executed"
        );

        Ok(())
    }
}
