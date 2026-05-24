use async_trait::async_trait;
use std::sync::Arc;
use crate::domain::{ServiceDomain, DomainOperation, DomainResult, DomainStatus, BianDomainId};
use crate::errors::DomainError;

/// BIAN Lending Service Domain (SD‑LEND)
pub struct LendingDomain;

impl LendingDomain {
    pub fn new() -> Arc<Self> { Arc::new(Self) }
}

#[async_trait]
impl ServiceDomain for LendingDomain {
    fn domain_id(&self) -> BianDomainId { "Lending".into() }

    async fn execute(&self, op: &DomainOperation) -> Result<DomainResult, DomainError> {
        match op.operation_type.as_str() {
            "originate" | "underwrite" | "disburse" => {
                Ok(DomainResult {
                    status: DomainStatus::Success,
                    data: serde_json::json!({"loan_id": uuid::Uuid::new_v4().to_string()}),
                    events: vec![],
                })
            }
            _ => Err(DomainError::UnsupportedOperation {
                domain: self.domain_id(),
                operation: op.operation_type.clone(),
            }),
        }
    }

    fn supports_operation(&self, op: &str) -> bool {
        matches!(op, "originate" | "underwrite" | "disburse")
    }
}
