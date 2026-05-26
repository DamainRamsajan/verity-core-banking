#[cfg(test)]
mod tests {
    use vaos_evolution::*;

    #[tokio::test]
    async fn test_fggm_rejects_unsafe_proposal() {
        let engine = engine::EvolutionEngine::new(engine::EvolutionConfig::default());

        let proposal = types::EvolutionProposal {
            proposal_id: uuid::Uuid::new_v4(),
            agent_id: vaos_core::types::AgentId::new(),
            description: "Optimise payment routing without capability checks".into(),
            proposed_code: "fn route_payment() { /* no token validation */ }".into(),
            safety_invariants: vec!["P3".into()],
            performance_metrics: serde_json::json!({}),
            proposed_at: chrono::Utc::now(),
        };

        let cert = engine.submit(proposal).await.unwrap();
        assert!(!cert.verified);
        assert!(cert.counterexample.is_some());
    }

    #[tokio::test]
    async fn test_fggm_accepts_safe_proposal() {
        let engine = engine::EvolutionEngine::new(engine::EvolutionConfig::default());

        let proposal = types::EvolutionProposal {
            proposal_id: uuid::Uuid::new_v4(),
            agent_id: vaos_core::types::AgentId::new(),
            description: "Improve fraud detection with capability token validation".into(),
            proposed_code: "fn detect_fraud(capability_token) { validate(capability_token); }".into(),
            safety_invariants: vec!["P3".into()],
            performance_metrics: serde_json::json!({}),
            proposed_at: chrono::Utc::now(),
        };

        let cert = engine.submit(proposal).await.unwrap();
        assert!(cert.verified);
    }

    #[tokio::test]
    async fn test_all_eight_invariants_loaded() {
        let contracts = contract::SafetyContract::all_invariants();
        assert_eq!(contracts.len(), 8);
        let p1 = contracts.iter().find(|c| c.contract_id == "P1").unwrap();
        assert!(p1.is_hard_constraint);
    }
}
