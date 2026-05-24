#[cfg(test)]
mod tests {
    use haip_inclusive::*;

    #[tokio::test]
    async fn test_register_and_check() {
        let engine = engine::InclusiveEngine::new();
        let user = uuid::Uuid::new_v4();
        let profile = types::AccessibilityProfile {
            user_id: user,
            features: vec![types::AccessibilityFeature::LargeText, types::AccessibilityFeature::ScreenReader],
            language: "en".into(),
            offline_preferred: false,
        };
        engine.register_profile(profile).await.unwrap();
        let ok = engine.check_interface(user, types::ComplianceLevel::AAA).await.unwrap();
        assert!(ok);
        let bad = engine.check_interface(user, types::ComplianceLevel::AA).await.unwrap();
        assert!(!bad);
    }
}
