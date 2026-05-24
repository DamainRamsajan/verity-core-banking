use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique BIAN Service Domain identifier (matches BIAN v14.0 codes)
pub type BianDomainId = String;

/// An operation within a BIAN service domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainOperation {
    pub operation_id: Uuid,
    pub domain_id: BianDomainId,
    pub operation_type: String,
    pub payload: serde_json::Value,
    pub capability_token: Option<vaos_core::types::CapabilityToken>,
}

/// Result of a domain operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainResult {
    pub status: DomainStatus,
    pub data: serde_json::Value,
    pub events: Vec<DomainEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DomainStatus {
    Success,
    Rejected,
    Pending,
}

/// An event emitted by a domain operation (for event‑sourcing)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainEvent {
    pub event_type: String,
    pub aggregate_id: String,
    pub payload: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// The core trait for any BIAN service domain.
#[async_trait]
pub trait ServiceDomain: Send + Sync {
    /// Unique BIAN domain ID
    fn domain_id(&self) -> BianDomainId;
    /// Execute a domain operation
    async fn execute(&self, op: &DomainOperation) -> Result<DomainResult, super::DomainError>;
    /// Check if this domain can handle a specific operation type
    fn supports_operation(&self, operation_type: &str) -> bool;
}
