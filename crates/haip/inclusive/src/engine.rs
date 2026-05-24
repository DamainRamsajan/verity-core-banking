use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::types::{AccessibilityProfile, AccessibilityFeature, ComplianceLevel};
use super::validator::AccessibilityValidator;
use super::errors::InclusiveError;

/// Central inclusive design engine.
pub struct InclusiveEngine {
    profiles: RwLock<HashMap<Uuid, AccessibilityProfile>>,
    validator: AccessibilityValidator,
}

impl InclusiveEngine {
    pub fn new() -> Self {
        Self {
            profiles: RwLock::new(HashMap::new()),
            validator: AccessibilityValidator::new(),
        }
    }

    /// Register a user's accessibility profile.
    #[tracing::instrument(name = "inclusive.register", level = "info", skip(self))]
    pub async fn register_profile(
        &self,
        profile: AccessibilityProfile,
    ) -> Result<(), InclusiveError> {
        let mut profiles = self.profiles.write().await;
        profiles.insert(profile.user_id, profile.clone());

        // Validate that the profile can be served
        self.validator.validate(&profile)?;

        tracing::info!(user_id = %profile.user_id, features = ?profile.features, "Accessibility profile registered");
        Ok(())
    }

    /// Check that a generated interface meets the user's accessibility requirements.
    #[tracing::instrument(name = "inclusive.check", level = "debug", skip(self))]
    pub async fn check_interface(
        &self,
        user_id: Uuid,
        interface_compliance: ComplianceLevel,
    ) -> Result<bool, InclusiveError> {
        let profiles = self.profiles.read().await;
        let profile = profiles.get(&user_id)
            .ok_or(InclusiveError::ProfileNotFound(user_id))?;

        // GABI‑enhanced requires at least AA + specific features
        if profile.features.contains(&AccessibilityFeature::SimplifiedUI)
            && interface_compliance != ComplianceLevel::GabiEnhanced {
            return Ok(false);
        }

        // WCAG 2.2 AAA requires all features satisfied
        if profile.features.contains(&AccessibilityFeature::ScreenReader)
            && interface_compliance != ComplianceLevel::AAA {
            return Ok(false);
        }

        Ok(true)
    }
}
