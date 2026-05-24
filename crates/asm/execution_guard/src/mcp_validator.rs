use super::types::{McpToolDescriptor, ValidationStatus};
use super::errors::GuardError;

pub struct McpValidator;

impl McpValidator {
    pub fn new() -> Self { Self }
    pub async fn validate(&self, _descriptor: &McpToolDescriptor) -> Result<ValidationStatus, GuardError> {
        Ok(ValidationStatus::Valid)
    }
}
