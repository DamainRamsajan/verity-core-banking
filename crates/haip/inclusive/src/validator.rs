use super::types::AccessibilityProfile;
use super::errors::InclusiveError;

/// Validates that a profile can be served by the infrastructure.
pub struct AccessibilityValidator;

impl AccessibilityValidator {
    pub fn new() -> Self { Self }

    pub fn validate(&self, profile: &AccessibilityProfile) -> Result<(), InclusiveError> {
        // Check for incompatible feature combinations
        if profile.features.contains(&super::types::AccessibilityFeature::OfflineMode)
            && !profile.offline_preferred {
            return Err(InclusiveError::IncompatibleFeatures(
                "Offline mode must be paired with offline_preferred flag".into()
            ));
        }
        Ok(())
    }
}
