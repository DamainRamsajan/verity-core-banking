use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type BianDomainId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainOperation {
    pub operation_id: Uuid,
    pub domain_id: BianDomainId,
    pub operation_type: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainResult {
    pub status: DomainStatus,
    pub data: serde_json::Value,
    pub events: Vec<DomainEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DomainStatus { Success, Rejected, Pending }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainEvent {
    pub event_type: String,
    pub aggregate_id: String,
    pub payload: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait ServiceDomain: Send + Sync {
    fn domain_id(&self) -> BianDomainId;
    async fn execute(&self, op: &DomainOperation) -> Result<DomainResult, super::DomainError>;
    fn supports_operation(&self, operation_type: &str) -> bool;
}
