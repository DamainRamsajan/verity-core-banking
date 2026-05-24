use async_trait::async_trait;
use std::sync::Arc;
use crate::domain::{ServiceDomain, DomainOperation, DomainResult, DomainStatus, BianDomainId};
use crate::errors::DomainError;

/// BIAN Current Account Service Domain (SD‑CA)
pub struct CurrentAccountDomain;

impl CurrentAccountDomain {
    pub fn new() -> Arc<Self> { Arc::new(Self) }
}

#[async_trait]
impl ServiceDomain for CurrentAccountDomain {
    fn domain_id(&self) -> BianDomainId { "CurrentAccount".into() }

    async fn execute(&self, op: &DomainOperation) -> Result<DomainResult, DomainError> {
        match op.operation_type.as_str() {
            "credit" | "debit" | "balance_inquiry" => {
                Ok(DomainResult {
                    status: DomainStatus::Success,
                    data: serde_json::json!({"message": format!("{} processed", op.operation_type)}),
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
        matches!(op, "credit" | "debit" | "balance_inquiry")
    }
}
