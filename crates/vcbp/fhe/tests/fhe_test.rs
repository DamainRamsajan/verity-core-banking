#[cfg(test)]
mod tests {
    use vcbp_fhe::*;

    #[tokio::test]
    async fn test_encrypt_and_add() {
        let config = engine::FheConfig::default();
        let fhe = engine::FheEngine::new(config);
        let ct1 = fhe.encrypt_balance(rust_decimal::Decimal::new(100, 0)).await.unwrap();
        let ct2 = fhe.encrypt_balance(rust_decimal::Decimal::new(50, 0)).await.unwrap();
        let sum = fhe.add_encrypted(&ct1, &ct2).await.unwrap();
        assert_eq!(sum.backend, ct1.backend);
    }

    #[tokio::test]
    async fn test_benchmark() {
        let config = engine::FheConfig::default();
        let fhe = engine::FheEngine::new(config);
        let results = fhe.benchmark().await.unwrap();
        assert!(!results.is_empty());
    }
}
