use std::collections::HashMap;
use vaos_core::types::CapScope;

pub struct TokenOntology {
    required_scopes: HashMap<String, Vec<CapScope>>,
}

impl TokenOntology {
    pub fn new() -> Self {
        let mut ont = Self { required_scopes: HashMap::new() };
        ont.add("debit", CapScope { operations: vec!["debit:account".into()], account_ids: vec![], amount_limit: None, counterparty_allowlist: None });
        ont.add("credit", CapScope { operations: vec!["credit:account".into()], account_ids: vec![], amount_limit: None, counterparty_allowlist: None });
        ont.add("wire_transfer", CapScope { operations: vec!["wire:transfer".into()], account_ids: vec![], amount_limit: Some(rust_decimal::Decimal::new(10000,0)), counterparty_allowlist: None });
        ont.add("wire_transfer_dual", CapScope { operations: vec!["approval:level_2".into()], account_ids: vec![], amount_limit: None, counterparty_allowlist: None });
        ont.add("loan_approval", CapScope { operations: vec!["loan:approve".into()], account_ids: vec![], amount_limit: None, counterparty_allowlist: None });
        ont.add("loan_approval_dual", CapScope { operations: vec!["risk:signoff".into()], account_ids: vec![], amount_limit: None, counterparty_allowlist: None });
        ont.add("gl_posting", CapScope { operations: vec!["gl:post".into()], account_ids: vec![], amount_limit: None, counterparty_allowlist: None });
        ont
    }

    fn add(&mut self, key: &str, scope: CapScope) {
        self.required_scopes.entry(key.to_string()).or_default().push(scope);
    }

    pub fn get_required_scopes(&self, op: &str) -> Option<&Vec<CapScope>> {
        self.required_scopes.get(op)
    }

    pub fn requires_dual_control(&self, op: &str) -> bool {
        self.required_scopes.get(op).map(|s| s.len() > 1).unwrap_or(false)
    }
}
