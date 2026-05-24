#[cfg(test)]
mod tests {
    use haip_claim::*;

    #[tokio::test]
    async fn test_autonomous_threshold() {
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
    async fn test_budget_exhaustion() {
        let mut config = engine::ClaimConfig::default();
        config.daily_budget = 3;
        let engine = engine::ClaimEngine::new(config);
        let user = uuid::Uuid::new_v4();
        let action = types::CognitiveAction {
            id: uuid::Uuid::new_v4(),
            agent_id: vaos_core::types::AgentId::new(),
            description: "High‑risk wire".into(),
            cognitive_cost: types::CognitiveCost::BinaryChoice,
            risk_severity: 80,
            defaults: vec![],
        };
        // First use consumes 5 (exceeds budget but high risk → FullEngagement)
        let pres = engine.present(user, action.clone()).await.unwrap();
        assert!(matches!(pres, types::Presentation::FullEngagement { .. }));
        // Second use: budget 0, risk low → error
        let low_risk = types::CognitiveAction {
            risk_severity: 10, ..action.clone()
        };
        let err = engine.present(user, low_risk).await.unwrap_err();
        assert!(matches!(err, errors::ClaimError::CognitiveBudgetExceeded { .. }));
    }
}
