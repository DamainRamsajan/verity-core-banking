use std::sync::Arc;
use tokio::sync::RwLock;

use super::domain::{ServiceDomain, DomainOperation, DomainResult};
use super::registry::DomainRegistry;
use super::errors::DomainError;

/// Central BIAN domain engine – routes operations to registered domains
pub struct BianDomainEngine {
    registry: Arc<RwLock<DomainRegistry>>,
}

impl BianDomainEngine {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(RwLock::new(DomainRegistry::new())),
        }
    }

    /// Register a BIAN service domain
    pub async fn register_domain(
        &self,
        domain: Arc<dyn ServiceDomain>,
    ) -> Result<(), DomainError> {
        let mut reg = self.registry.write().await;
        reg.register(domain)
    }

    /// Route a domain operation to the appropriate service domain
    #[tracing::instrument(name = "bian.execute", level = "info", skip(self))]
    pub async fn execute(
        &self,
        op: &DomainOperation,
    ) -> Result<DomainResult, DomainError> {
        let reg = self.registry.read().await;
        let domain = reg.get(&op.domain_id)
            .ok_or_else(|| DomainError::DomainNotFound(op.domain_id.clone()))?;

        // Check that the domain supports this operation
        if !domain.supports_operation(&op.operation_type) {
            return Err(DomainError::UnsupportedOperation {
                domain: op.domain_id.clone(),
                operation: op.operation_type.clone(),
            });
        }

        domain.execute(op).await
    }

    /// List all registered domains
    pub async fn list_domains(&self) -> Vec<String> {
        let reg = self.registry.read().await;
        reg.list()
    }
}
