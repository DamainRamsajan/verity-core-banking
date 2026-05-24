use async_trait::async_trait;
use crate::domain::{ServiceDomain, DomainOperation, DomainResult, DomainStatus, BianDomainId, DomainEvent};
use crate::errors::DomainError;

pub struct GeneralLedgerDomain;

impl GeneralLedgerDomain {
    pub fn domain_id_str() -> BianDomainId { "GeneralLedger".to_string() }
}

#[async_trait]
impl ServiceDomain for GeneralLedgerDomain {
    fn domain_id(&self) -> BianDomainId { Self::domain_id_str() }
    async fn execute(&self, op: &DomainOperation) -> Result<DomainResult, DomainError> {
        Ok(DomainResult {
            status: DomainStatus::Success,
            data: serde_json::json!({"domain": self.domain_id(), "op": op.operation_type}),
            events: vec![DomainEvent {
                event_type: op.operation_type.clone(),
                aggregate_id: op.domain_id.clone(),
                payload: op.payload.clone(),
                timestamp: chrono::Utc::now(),
            }],
        })
    }
    fn supports_operation(&self, _op: &str) -> bool { true }
}
