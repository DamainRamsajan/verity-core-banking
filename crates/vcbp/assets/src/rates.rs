use super::types::CurrencyPair;
use super::errors::AssetError;

/// FX rate provider with configurable sources.
pub struct FxRateProvider {
    cache: std::sync::RwLock<std::collections::HashMap<String, CurrencyPair>>,
}

impl FxRateProvider {
    pub fn new() -> Self {
        Self { cache: std::sync::RwLock::new(std::collections::HashMap::new()) }
    }

    pub async fn get_rate(&self, base: &str, quote: &str) -> Result<CurrencyPair, AssetError> {
        let key = format!("{}/{}", base, quote);
        if let Some(rate) = self.cache.read().unwrap().get(&key) {
            return Ok(rate.clone());
        }

        // In production: call external FX rate feed (Bloomberg, Reuters, OANDA)
        let pair = CurrencyPair {
            base: base.to_string(),
            quote: quote.to_string(),
            rate: rust_decimal::Decimal::new(11, 1), // placeholder 1.1
            timestamp: chrono::Utc::now(),
            source: "ECB".into(),
        };

        self.cache.write().unwrap().insert(key, pair.clone());
        Ok(pair)
    }
}
