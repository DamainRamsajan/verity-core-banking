#[cfg(test)]
mod tests {
    use haip_claim::*;
    use haip_eta::*;
    use haip_dashboard::*;
    use haip_inclusive::*;

    #[tokio::test]
    async fn test_claim_autonomous() {
        let engine = engine::ClaimEngine::new(engine::ClaimConfig::default());
        let user = uuid::Uuid::new_v4();
        let action = types::CognitiveAction {
            id: uuid::Uuid::new_v4(),
            agent_id: vaos_core::types::AgentId::new(),
            description: "Balance inquiry".into(),
            cognitive_cost: types::CognitiveCost::Passive,
            risk_severity: 5,
            defaults: vec![],
        };
        let pres = engine.present(user, action).await.unwrap();
        assert!(matches!(pres, types::Presentation::Autonomous));
    }

    #[tokio::test]
    async fn test_eta_stress_detection() {
        let engine = engine::EtaEngine::new();
        let user = uuid::Uuid::new_v4();
        let cal = engine.adapt(user, "overdraft").await.unwrap();
        assert_eq!(cal.recommended_tone, types::InteractionTone::Supportive);
    }

    #[tokio::test]
    async fn test_dashboard_boundaries() {
        let engine = engine::DashboardEngine::new();
        let agent = vaos_core::types::AgentId::new();
        let boundary = types::AgentBoundary {
            agent_id: agent,
            spending_limit: rust_decimal::Decimal::new(500, 0),
            approval_threshold: rust_decimal::Decimal::new(1000, 0),
            allowed_operations: vec!["debit".into(), "balance_inquiry".into()],
        };
        engine.set_boundaries(agent, boundary).await.unwrap();
        let ok = engine.check_action(agent, "debit", Some(rust_decimal::Decimal::new(200, 0))).await.unwrap();
        assert!(ok);
    }

    #[tokio::test]
    async fn test_inclusive_registration() {
        let engine = engine::InclusiveEngine::new();
        let user = uuid::Uuid::new_v4();
        let profile = types::AccessibilityProfile {
            user_id: user,
            features: vec![types::AccessibilityFeature::ScreenReader],
            language: "en".into(),
            offline_preferred: false,
        };
        engine.register_profile(profile).await.unwrap();
        let ok = engine.check_interface(user, types::ComplianceLevel::AAA).await.unwrap();
        assert!(ok);
    }
}
