use super::types::AssetClass;

/// ISO 4217 currency registry with asset classification.
pub struct CurrencyRegistry {
    fiat_currencies: std::collections::HashSet<String>,
    digital_assets: std::collections::HashSet<String>,
    precious_metals: std::collections::HashSet<String>,
}

impl CurrencyRegistry {
    pub fn new() -> Self {
        let mut reg = Self {
            fiat_currencies: std::collections::HashSet::new(),
            digital_assets: std::collections::HashSet::new(),
            precious_metals: std::collections::HashSet::new(),
        };
        for c in &["USD", "EUR", "GBP", "JPY", "CHF", "CAD", "AUD", "CNY", "INR"] {
            reg.fiat_currencies.insert(c.to_string());
        }
        for c in &["BTC", "ETH", "USDC", "USDT", "JPM"] {
            reg.digital_assets.insert(c.to_string());
        }
        for c in &["XAU", "XAG", "XPT"] {
            reg.precious_metals.insert(c.to_string());
        }
        reg
    }

    pub fn classify(&self, currency: &str) -> AssetClass {
        if self.fiat_currencies.contains(currency) { AssetClass::FiatCurrency }
        else if self.digital_assets.contains(currency) { AssetClass::DigitalAsset }
        else if self.precious_metals.contains(currency) { AssetClass::PreciousMetal }
        else { AssetClass::TokenizedDeposit }
    }
}
