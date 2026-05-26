#[cfg(test)]
mod tests {
    use vcbp_fido::*;

    #[tokio::test]
    async fn test_issue_and_verify_mandate() {
        let engine = engine::FidoEngine::new(engine::FidoConfig::default());
        let agent = vaos_core::types::AgentId::new();

        let scope = types::MandateScope {
            max_amount: rust_decimal::Decimal::new(1000, 0),
            currency: "USD".into(),
            counterparty_allowlist: vec!["merchant-123".into()],
            frequency_limit: Some(10),
            action_types: vec!["payment".into()],
        };

        let mandate = engine
            .issue_mandate(agent, "user-456", scope)
            .await
            .unwrap();

        let ok = engine
            .verify_payment(&mandate.mandate_id, rust_decimal::Decimal::new(500, 0), "USD", "merchant-123", "payment")
            .await
            .unwrap();
        assert!(ok);

        let not_ok = engine
            .verify_payment(&mandate.mandate_id, rust_decimal::Decimal::new(2000, 0), "USD", "merchant-123", "payment")
            .await
            .unwrap();
        assert!(!not_ok);
    }
}
