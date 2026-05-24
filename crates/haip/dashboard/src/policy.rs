use super::types::AgentBoundary;

/// Delegation policy manager.
pub struct DelegationPolicy;

impl DelegationPolicy {
    pub fn new() -> Self { Self }

    /// Validate that an action conforms to the delegation policy.
    pub fn validate(
        boundary: &AgentBoundary,
        action: &str,
        amount: Option<rust_decimal::Decimal>,
    ) -> bool {
        if !boundary.allowed_operations.contains(&action.to_string()) {
            return false;
        }
        if let (Some(amt), limit) = (amount, boundary.approval_threshold) {
            if amt > limit { return false; }
        }
        true
    }
}
