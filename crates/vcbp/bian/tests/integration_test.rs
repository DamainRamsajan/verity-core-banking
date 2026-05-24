#[cfg(test)]
mod tests {
    use vcbp_bian::*;

    #[tokio::test]
    async fn test_domain_registration() {
        let engine = engine::BianDomainEngine::new();
        let ca = domains::current_account::CurrentAccountDomain::new();
        engine.register_domain(ca).await.unwrap();
        let list = engine.list_domains().await;
        assert!(list.contains(&"CurrentAccount".to_string()));
    }
}
