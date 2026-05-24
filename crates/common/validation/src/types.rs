use serde::{Deserialize, Serialize};

/// Result of a validation check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub passed: bool,
    pub rule_name: String,
    pub message: Option<String>,
    pub evidence: Option<String>,
}

/// Context for a validation operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationContext {
    pub domain: String,
    pub operation: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ValidationResult {
    pub fn pass(rule_name: &str) -> Self {
        Self { passed: true, rule_name: rule_name.to_string(), message: None, evidence: None }
    }
    pub fn fail(rule_name: &str, reason: &str) -> Self {
        Self { passed: false, rule_name: rule_name.to_string(), message: Some(reason.to_string()), evidence: None }
    }
}
