#[cfg(test)]
mod tests {
    use haip_dashboard::*;

    #[tokio::test]
    async fn test_set_and_check_boundaries() {
        let engine = engine::DashboardEngine::new();
        let agent = vaos_core::types::AgentId::new();
        let boundary = types::AgentBoundary {
            agent_id: agent,
            spending_limit: rust_decimal::Decimal::new(500, 0),
            approval_threshold: rust_decimal::Decimal::new(1000, 0),
            allowed_operations: vec!["debit".into(), "balance_inquiry".into()],
            ..Default::default()
        };
        engine.set_boundaries(agent, boundary).await.unwrap();
        let ok = engine.check_action(agent, "debit", Some(rust_decimal::Decimal::new(200, 0)), None).await.unwrap();
        assert!(ok);
        let bad = engine.check_action(agent, "wire_transfer", Some(rust_decimal::Decimal::new(200, 0)), None).await.unwrap();
        assert!(!bad);
    }
}
