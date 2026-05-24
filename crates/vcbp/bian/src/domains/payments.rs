use async_trait::async_trait;
use std::sync::Arc;
use crate::domain::{ServiceDomain, DomainOperation, DomainResult, DomainStatus, BianDomainId};
use crate::errors::DomainError;

/// BIAN Payments Service Domain (SD‑PAY)
pub struct PaymentsDomain;

impl PaymentsDomain {
    pub fn new() -> Arc<Self> { Arc::new(Self) }
}

#[async_trait]
impl ServiceDomain for PaymentsDomain {
    fn domain_id(&self) -> BianDomainId { "Payments".into() }

    async fn execute(&self, op: &DomainOperation) -> Result<DomainResult, DomainError> {
        match op.operation_type.as_str() {
            "wire_transfer" | "ach" | "rtp" => {
                Ok(DomainResult {
                    status: DomainStatus::Success,
                    data: serde_json::json!({"payment_id": uuid::Uuid::new_v4().to_string()}),
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
        matches!(op, "wire_transfer" | "ach" | "rtp")
    }
}
