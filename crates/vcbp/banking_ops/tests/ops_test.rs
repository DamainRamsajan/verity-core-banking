#[cfg(test)]
mod tests {
    use vcbp_banking_ops::*;

    #[tokio::test]
    async fn test_token_ontology() {
        let ont = tokens::TokenOntology::new();
        let scopes = ont.get_required_scopes("debit").unwrap();
        assert_eq!(scopes.len(), 1);
        assert!(ont.requires_dual_control("wire_transfer"));
        assert!(!ont.requires_dual_control("debit"));
    }

    #[tokio::test]
    async fn test_dual_control_enforcement() {
        let engine = engine::BankingOpsEngine::new();
        let op = operations::WireTransferOp {
            id: uuid::Uuid::new_v4(),
            from_account: uuid::Uuid::new_v4(),
            to_account: uuid::Uuid::new_v4(),
            amount: rust_decimal::Decimal::new(20_000, 0),
            currency: "USD".into(),
            initiator: vaos_core::types::AgentId::new(),
        };
        let banking_op = operations::BankingOperation::WireTransfer(op);
        let result = engine.execute(&banking_op, &[]).await;
        assert!(result.is_err());
    }
}
