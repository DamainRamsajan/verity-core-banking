use super::errors::ValidationError;

/// BIAN v14.0 Service Domain validator.
///
/// Validates that domain identifiers conform to the BIAN Service Landscape v14.0
/// (328 Service Domains) naming conventions.
pub struct BianValidator;

impl BianValidator {
    pub fn new() -> Self { Self }

    /// Validate a BIAN Service Domain identifier.
    pub fn validate_domain_id(&self, domain_id: &str) -> Result<(), ValidationError> {
        if domain_id.is_empty() || domain_id.len() > 128 {
            return Err(ValidationError::InvalidBianDomain(domain_id.to_string()));
        }
        if !domain_id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(ValidationError::InvalidBianDomain(domain_id.to_string()));
        }
        Ok(())
    }

    /// Validate a BIAN operation type.
    pub fn validate_operation(&self, operation: &str) -> Result<(), ValidationError> {
        if operation.is_empty() || operation.len() > 64 {
            return Err(ValidationError::InvalidOperation(operation.to_string()));
        }
        Ok(())
    }
}
