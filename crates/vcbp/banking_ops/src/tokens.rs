use std::collections::HashMap;
use vaos_core::types::{CapScope, CapabilityToken, AgentId, TokenId};

/// Maps banking operations to required capability token scopes.
///
/// The ontology defines exactly which tokens are required for each
/// banking operation type. Tokens are unforgeable (PASETO v4 signed)
/// and delegation‑depth‑limited.
pub struct TokenOntology {
    /// Required token scopes per operation type
    required_scopes: HashMap<String, Vec<CapScope>>,
}

impl TokenOntology {
    pub fn new() -> Self {
        let mut ont = Self { required_scopes: HashMap::new() };

        // Debit operations require a debit token with per‑account scope
        ont.add_requirement("debit", CapScope {
            operations: vec!["debit:account".into()],
            account_ids: vec![],
            amount_limit: None,
            counterparty_allowlist: None,
        });

        // Credit operations
        ont.add_requirement("credit", CapScope {
            operations: vec!["credit:account".into()],
            account_ids: vec![],
            amount_limit: None,
            counterparty_allowlist: None,
        });

        // Wire transfers require TWO tokens (dual‑control) if >$10K
        ont.add_requirement("wire_transfer", CapScope {
            operations: vec!["wire:transfer".into()],
            account_ids: vec![],
            amount_limit: Some(rust_decimal::Decimal::new(10_000, 0)),
            counterparty_allowlist: None,
        });
        ont.add_requirement("wire_transfer_dual", CapScope {
            operations: vec!["approval:level_2".into()],
            account_ids: vec![],
            amount_limit: None,
            counterparty_allowlist: None,
        });

        // Loan approvals require two tokens (four‑eyes principle)
        ont.add_requirement("loan_approval", CapScope {
            operations: vec!["loan:approve".into()],
            account_ids: vec![],
            amount_limit: None,
            counterparty_allowlist: None,
        });
        ont.add_requirement("loan_approval_dual", CapScope {
            operations: vec!["risk:signoff".into()],
            account_ids: vec![],
            amount_limit: None,
            counterparty_allowlist: None,
        });

        // GL posting
        ont.add_requirement("gl_posting", CapScope {
            operations: vec!["gl:post".into()],
            account_ids: vec![],
            amount_limit: None,
            counterparty_allowlist: None,
        });

        // Regulatory filing
        ont.add_requirement("regulatory_filing", CapScope {
            operations: vec!["regulatory:file".into()],
            account_ids: vec![],
            amount_limit: None,
            counterparty_allowlist: None,
        });

        ont
    }

    fn add_requirement(&mut self, key: &str, scope: CapScope) {
        self.required_scopes.entry(key.to_string()).or_default().push(scope);
    }

    /// Get the required token scopes for an operation type.
    pub fn get_required_scopes(&self, operation_type: &str) -> Option<&Vec<CapScope>> {
        self.required_scopes.get(operation_type)
    }

    /// Whether this operation type requires dual‑control (multiple tokens).
    pub fn requires_dual_control(&self, operation_type: &str) -> bool {
        self.required_scopes.get(operation_type)
            .map(|s| s.len() > 1)
            .unwrap_or(false)
    }
}
