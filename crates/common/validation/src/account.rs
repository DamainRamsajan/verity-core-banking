use super::errors::ValidationError;

/// Account identifier validator.
pub struct AccountValidator;

impl AccountValidator {
    pub fn new() -> Self { Self }

    /// Validate an account identifier format.
    pub fn validate_account_id(&self, id: &str) -> Result<(), ValidationError> {
        if id.is_empty() || id.len() > 64 {
            return Err(ValidationError::InvalidAccountId(id.to_string()));
        }
        if !id.chars().all(|c| c.is_alphanumeric() || c == '-') {
            return Err(ValidationError::InvalidAccountId(id.to_string()));
        }
        Ok(())
    }
}
