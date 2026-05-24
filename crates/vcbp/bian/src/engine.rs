use std::sync::Arc;
use tokio::sync::RwLock;
use super::domain::{ServiceDomain, DomainOperation, DomainResult};
use super::registry::DomainRegistry;
use super::errors::DomainError;

pub struct BianDomainEngine {
    registry: Arc<RwLock<DomainRegistry>>,
}

impl BianDomainEngine {
    pub fn new() -> Self { Self { registry: Arc::new(RwLock::new(DomainRegistry::new())) } }
    pub async fn register_domain(&self, domain: Arc<dyn ServiceDomain>) -> Result<(), DomainError> {
        self.registry.write().await.register(domain)
    }
    pub async fn execute(&self, op: &DomainOperation) -> Result<DomainResult, DomainError> {
        let reg = self.registry.read().await;
        let domain = reg.get(&op.domain_id).ok_or_else(|| DomainError::DomainNotFound(op.domain_id.clone()))?;
        if !domain.supports_operation(&op.operation_type) {
            return Err(DomainError::UnsupportedOperation { domain: op.domain_id.clone(), operation: op.operation_type.clone() });
        }
        domain.execute(op).await
    }
    pub async fn list_domains(&self) -> Vec<String> { self.registry.read().await.list() }
}
