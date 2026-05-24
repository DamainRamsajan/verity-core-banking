use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;
use super::types::{AccessibilityProfile, ComplianceLevel};
use super::errors::InclusiveError;

pub struct InclusiveEngine {
    profiles: RwLock<HashMap<Uuid, AccessibilityProfile>>,
}

impl InclusiveEngine {
    pub fn new() -> Self { Self { profiles: RwLock::new(HashMap::new()) } }

    pub async fn register_profile(&self, profile: AccessibilityProfile) -> Result<(), InclusiveError> {
        let mut profiles = self.profiles.write().await;
        profiles.insert(profile.user_id, profile);
        Ok(())
    }

    pub async fn check_interface(&self, user_id: Uuid, compliance: ComplianceLevel) -> Result<bool, InclusiveError> {
        let profiles = self.profiles.read().await;
        let profile = profiles.get(&user_id).ok_or(InclusiveError::ProfileNotFound(user_id))?;
        if profile.features.contains(&super::types::AccessibilityFeature::ScreenReader) && compliance != ComplianceLevel::AAA {
            return Ok(false);
        }
        Ok(true)
    }
}
