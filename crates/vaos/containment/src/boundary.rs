//! Boundary policy definition for containment verification.

/// A declarative policy defining the boundary of safe agent actions.
#[derive(Debug, Clone)]
pub struct BoundaryPolicy {
    /// Operations that the agent is permitted to perform
    pub allowed_operations: Vec<String>,
    /// Maximum transaction amount (None = unlimited)
    pub max_transaction_amount: Option<rust_decimal::Decimal>,
    /// Allowed counterparties (None = all, empty = none)
    pub counterparty_allowlist: Option<Vec<String>>,
    /// Whether to enforce the policy under havoc oracle semantics
    pub havoc_enforced: bool,
}

impl BoundaryPolicy {
    /// A restrictive policy suitable for untrusted agents.
    pub fn restrictive() -> Self {
        Self {
            allowed_operations: vec!["balance_inquiry".into(), "mini_statement".into()],
            max_transaction_amount: Some(rust_decimal::Decimal::new(100, 0)),
            counterparty_allowlist: Some(vec![]),
            havoc_enforced: true,
        }
    }

    /// A standard policy for verified banking agents.
    pub fn standard_banking() -> Self {
        Self {
            allowed_operations: vec![
                "debit".into(), "credit".into(), "transfer".into(),
                "balance_inquiry".into(), "deposit".into(), "withdrawal".into(),
            ],
            max_transaction_amount: None,
            counterparty_allowlist: None,
            havoc_enforced: true,
        }
    }
}
