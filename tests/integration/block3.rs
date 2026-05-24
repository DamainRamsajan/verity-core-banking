#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use vcbp_bian::*;
    use vcbp_product_engine::*;
    use vcbp_payments::*;
    use vcbp_reporting::*;

    #[tokio::test]
    async fn test_bian_engine_register_and_execute() {
        let engine = engine::BianDomainEngine::new();
        let domain = Arc::new(domains::current_account::CurrentAccountDomain);
        engine.register_domain(domain).await.unwrap();
        let op = domain::DomainOperation {
            operation_id: uuid::Uuid::new_v4(),
            domain_id: "CurrentAccount".into(),
            operation_type: "credit".into(),
            payload: serde_json::json!({"amount": 500}),
        };
        let result = engine.execute(&op).await.unwrap();
        assert_eq!(result.status, domain::DomainStatus::Success);
    }

    #[tokio::test]
    async fn test_product_compilation() {
        let compiler = compiler::AslProductCompiler::new();
        let product = compiler.compile("product CheckingAccount { ... }", "Checking").unwrap();
        assert!(product.verified);
        assert!(!product.verified_invariants.is_empty());
    }

    #[tokio::test]
    async fn test_payment_engine_with_rails() {
        let engine = engine::PaymentEngine::new();
        let fednow = Arc::new(rails::FedNowRail::new());
        engine.register_rail(fednow).await.unwrap();
        let payment = rail::Payment {
            id: uuid::Uuid::new_v4(),
            from_account: uuid::Uuid::new_v4(),
            to_account: "123456789".into(),
            amount: rust_decimal::Decimal::new(500, 0),
            currency: "USD".into(),
            rail_type: rail::RailType::FedNow,
            priority: rail::PaymentPriority::High,
            metadata: serde_json::Value::Null,
        };
        let receipt = engine.send(&payment).await.unwrap();
        assert_eq!(receipt.status, rail::PaymentStatus::Accepted);
    }

    #[tokio::test]
    async fn test_call_report_generation() {
        let reporter = reporter::RegulatoryReporter::new();
        let report = reporter.generate_call_report(chrono::NaiveDate::from_ymd_opt(2026, 3, 31).unwrap()).await.unwrap();
        assert_eq!(report.period_end.to_string(), "2026-03-31");
        let zk = reporter.generate_zk_proof(&uuid::Uuid::new_v4()).await.unwrap();
        assert!(!zk.proof_bytes.is_empty());
    }
}
