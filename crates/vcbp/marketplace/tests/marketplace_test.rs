#[cfg(test)]
mod tests {
    use vcbp_marketplace::*;

    #[tokio::test]
    async fn test_agent_listing() {
        let config = registry::RegistryConfig::default();
        let tcr = registry::TokenCuratedRegistry::new(config);
        let listing = types::AgentListing {
            listing_id: uuid::Uuid::new_v4(),
            agent_id: vaos_core::types::AgentId::new(),
            name: "Fraud Detection Agent".into(),
            description: "Real‑time GNN fraud detection".into(),
            capabilities: vec!["fraud_detection".into()],
            stake_amount: rust_decimal::Decimal::new(1_000, 0),
            status: types::ListingStatus::Pending,
            reputation: types::ReputationScore::new(),
            listed_at: chrono::Utc::now(),
            challenges: vec![],
        };
        let result = tcr.apply_listing(listing).await.unwrap();
        assert_eq!(result.status, types::ListingStatus::Pending);
    }

    #[test]
    fn test_reputation_bayesian_update() {
        let mut score = types::ReputationScore::new();
        score.update(true);
        score.update(true);
        score.update(true);
        score.update(false);
        assert!(score.mean > 0.5);
    }
}
