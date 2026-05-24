use std::collections::HashSet;
use tokio::sync::RwLock;
use super::types::{ParameterChange, PolicyAuthorization, InvariantCheck};
use super::errors::FimError;

pub struct FinancialInvariantsMonitor {
    protected_parameters: RwLock<HashSet<String>>,
    config: FimConfig,
    stats: RwLock<FimStats>,
}

#[derive(Debug, Clone)]
pub struct FimConfig { pub halt_on_violation: bool, pub require_policy_signature: bool }

impl Default for FimConfig {
    fn default() -> Self { Self { halt_on_violation: true, require_policy_signature: true } }
}

#[derive(Debug, Default, Clone)]
pub struct FimStats { pub transactions_checked: u64, pub violations_detected: u64 }

impl FinancialInvariantsMonitor {
    pub fn new(config: FimConfig) -> Self {
        let mut params = HashSet::new();
        params.insert("credit_limit".into());
        params.insert("fee_structure".into());
        params.insert("interest_rate_base".into());
        params.insert("routing_rules".into());
        Self { protected_parameters: RwLock::new(params), config, stats: RwLock::new(FimStats::default()) }
    }

    pub async fn check_transaction(&self, params: &[ParameterChange]) -> Result<(), FimError> {
        let mut stats = self.stats.write().await;
        stats.transactions_checked += 1;
        let protected = self.protected_parameters.read().await;
        for change in params {
            if protected.contains(&change.parameter_name) && !change.authorized {
                stats.violations_detected += 1;
                return Err(FimError::InvariantViolation { parameter: change.parameter_name.clone(), reason: "Unauthorized parameter mutation without signed policy change".into() });
            }
        }
        Ok(())
    }
}
