#[cfg(test)]
mod tests {
    use vcbp_edge::*;

    #[tokio::test]
    async fn test_offline_transaction() {
        let config = types::EdgeConfig::default();
        let runtime = runtime::EdgeRuntime::new(config);
        let tx = types::OfflineTransaction {
            id: uuid::Uuid::new_v4(),
            from_account: uuid::Uuid::new_v4(),
            to_account: "recipient".into(),
            amount: rust_decimal::Decimal::new(500, 0),
            currency: "USD".into(),
            timestamp: chrono::Utc::now(),
            signature: vec![],
            synced: false,
        };
        runtime.process_transaction(tx).await.unwrap();
    }
}
