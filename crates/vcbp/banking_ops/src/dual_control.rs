use vaos_core::types::{CapabilityToken, AgentId};
use super::errors::BankingOpsError;
use std::collections::HashSet;

pub struct DualControlEnforcer;

impl DualControlEnforcer {
    pub fn verify(tokens: &[CapabilityToken], required_count: usize) -> Result<(), BankingOpsError> {
        if tokens.len() < required_count {
            return Err(BankingOpsError::DualControlRequired { operation: String::new(), required: required_count, provided: tokens.len() });
        }
        let principals: HashSet<AgentId> = tokens.iter().map(|t| t.issued_by).collect();
        if principals.len() < required_count {
            return Err(BankingOpsError::DualControlPrincipalsViolation { required: required_count, distinct_principals: principals.len() });
        }
        Ok(())
    }
}
