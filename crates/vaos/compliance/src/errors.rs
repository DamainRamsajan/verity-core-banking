//! Error types for the Lean-Agent Compliance Verifier.

#[derive(Debug, thiserror::Error)]
pub enum ComplianceError {
    #[error("Regulatory domain not supported: {0}")]
    DomainNotSupported(String),

    #[error("Compliance violation: action {action} in domain {domain}: {counterexample}")]
    ComplianceViolation {
        action: uuid::Uuid,
        domain: String,
        counterexample: String,
    },

    #[error("Proof timeout: {0}ms exceeded")]
    ProofTimeout(u64),

    #[error("Axiom library stale — regulatory change detected in domain: {0}")]
    AxiomStale(String),

    #[error("Lean 4 kernel error: {0}")]
    KernelError(String),

    #[error("Formalization failed: {0}")]
    FormalizationFailed(String),
}
