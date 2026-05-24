use std::collections::HashSet;
use tokio::sync::RwLock;
use super::types::ParameterChange;
use super::errors::FimError;

pub struct FinancialInvariantsMonitor {
    protected_parameters: RwLock<HashSet<String>>,
}

impl FinancialInvariantsMonitor {
    pub fn new() -> Self {
        let mut params = HashSet::new();
        params.insert("credit_limit".into());
        params.insert("fee_structure".into());
        params.insert("interest_rate_base".into());
        params.insert("routing_rules".into());
        Self { protected_parameters: RwLock::new(params) }
    }

    pub async fn check_transaction(&self, changes: &[ParameterChange]) -> Result<(), FimError> {
        let protected = self.protected_parameters.read().await;
        for change in changes {
            if protected.contains(&change.parameter_name) && !change.authorized {
                return Err(FimError::InvariantViolation {
                    parameter: change.parameter_name.clone(),
                    reason: "Unauthorized parameter mutation without signed policy change".into(),
                });
            }
        }
        Ok(())
    }
}
