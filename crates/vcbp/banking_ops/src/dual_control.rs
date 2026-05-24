use vaos_core::types::{CapabilityToken, AgentId};
use super::errors::BankingOpsError;

/// Enforces the four‑eyes principle as a structural invariant.
///
/// For operations requiring dual‑control (wire transfers >$10K,
/// loan approvals, GL postings), two capability tokens from
/// **different principals** must be presented. This is enforced
/// at the VM level — not a configurable policy.
pub struct DualControlEnforcer;

impl DualControlEnforcer {
    /// Verify that dual‑control requirements are satisfied.
    ///
    /// # Pre‑conditions
    /// - At least two tokens must be provided
    /// - Tokens must be issued to different principals
    ///
    /// # Post‑conditions
    /// - Returns Ok if dual‑control is satisfied
    /// - Returns DualControlRequired if tokens are from the same principal
    ///   or insufficient tokens are provided
    pub fn verify(
        tokens: &[CapabilityToken],
        required_count: usize,
    ) -> Result<(), BankingOpsError> {
        if tokens.len() < required_count {
            return Err(BankingOpsError::DualControlRequired {
                operation: "unknown".into(),
                required: required_count,
                provided: tokens.len(),
            });
        }

        // Verify that tokens are from distinct principals
        let principals: std::collections::HashSet<AgentId> = tokens
            .iter()
            .map(|t| t.issued_by)
            .collect();

        if principals.len() < required_count {
            return Err(BankingOpsError::DualControlPrincipalsViolation {
                required: required_count,
                distinct_principals: principals.len(),
            });
        }

        Ok(())
    }
}
