use std::collections::HashMap;
use std::sync::Arc;
use super::domain::{ServiceDomain, BianDomainId};
use super::errors::DomainError;

/// Registry of all BIAN service domains
pub struct DomainRegistry {
    domains: HashMap<BianDomainId, Arc<dyn ServiceDomain>>,
}

impl DomainRegistry {
    pub fn new() -> Self {
        Self { domains: HashMap::new() }
    }

    pub fn register(
        &mut self,
        domain: Arc<dyn ServiceDomain>,
    ) -> Result<(), DomainError> {
        let id = domain.domain_id();
        if self.domains.contains_key(&id) {
            return Err(DomainError::DomainAlreadyRegistered(id));
        }
        self.domains.insert(id, domain);
        Ok(())
    }

    pub fn get(&self, id: &BianDomainId) -> Option<&Arc<dyn ServiceDomain>> {
        self.domains.get(id)
    }

    pub fn list(&self) -> Vec<String> {
        self.domains.keys().cloned().collect()
    }
}
