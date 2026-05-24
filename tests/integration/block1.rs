#[cfg(test)]
mod tests {
    use vcbp_ledger::MerkleLedger;
    use vcbp_banking_ops::BankingOpsEngine;
    use vaos_core::types::CapabilityToken;

    #[tokio::test]
    async fn test_ledger_append_and_balance() {
        let ledger = MerkleLedger::new(vcbp_ledger::merkle_ledger::LedgerConfig::default());
        let tx = vcbp_ledger::Transaction {
            id: uuid::Uuid::new_v4(),
            correlation_id: uuid::Uuid::new_v4(),
            entries: vec![
                vcbp_ledger::Entry {
                    account_id: uuid::Uuid::new_v4(),
                    amount: rust_decimal::Decimal::new(100, 0),
                    currency: "USD".into(),
                    entry_type: vcbp_ledger::EntryType::Debit,
                    compliance_tags: vec![],
                },
                vcbp_ledger::Entry {
                    account_id: uuid::Uuid::new_v4(),
                    amount: rust_decimal::Decimal::new(-100, 0),
                    currency: "USD".into(),
                    entry_type: vcbp_ledger::EntryType::Credit,
                    compliance_tags: vec![],
                },
            ],
            timestamp: chrono::Utc::now(),
            agent_id: None,
            capability_token_id: None,
            metadata: serde_json::Value::Null,
        };
        let proof = ledger.append(tx).await.unwrap();
        assert!(!proof.merkle_root.is_empty());
    }

    #[tokio::test]
    async fn test_banking_ops_dual_control() {
        let engine = BankingOpsEngine::new();
        let token = CapabilityToken::test_token();
        let op = vcbp_banking_ops::operations::BankingOperation::WireTransfer(
            vcbp_banking_ops::operations::WireTransferOp {
                id: uuid::Uuid::new_v4(),
                from_account: uuid::Uuid::new_v4(),
                to_account: uuid::Uuid::new_v4(),
                amount: rust_decimal::Decimal::new(20000, 0),
                currency: "USD".into(),
                initiator: vaos_core::types::AgentId::new(),
            }
        );
        let result = engine.execute(&op, &[token]).await;
        assert!(result.is_err());
    }
}
