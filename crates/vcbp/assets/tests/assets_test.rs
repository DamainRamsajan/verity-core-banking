#[cfg(test)]
mod tests {
    use vcbp_assets::*;

    #[tokio::test]
    async fn test_update_position() {
        let engine = engine::MultiAssetEngine::new();
        let account = uuid::Uuid::new_v4();
        let pos = engine.update_position(account, "USD", rust_decimal::Decimal::new(1000, 0)).await.unwrap();
        assert_eq!(pos.currency_code, "USD");
        assert_eq!(pos.balance, rust_decimal::Decimal::new(1000, 0));
    }

    #[tokio::test]
    async fn test_fx_rate() {
        let engine = engine::MultiAssetEngine::new();
        let rate = engine.get_fx_rate("EUR", "USD").await.unwrap();
        assert_eq!(rate.base, "EUR");
    }
}
