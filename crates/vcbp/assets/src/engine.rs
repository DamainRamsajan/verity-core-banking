use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;
use super::types::{AssetPosition, CurrencyPair};
use super::errors::AssetError;

#[allow(dead_code)]
pub struct MultiAssetEngine {
    positions: RwLock<HashMap<Uuid, Vec<AssetPosition>>>,
    fx_rates: RwLock<HashMap<String, CurrencyPair>>,
}

impl MultiAssetEngine {
    pub fn new() -> Self {
        Self { positions: RwLock::new(HashMap::new()), fx_rates: RwLock::new(HashMap::new()) }
    }

    pub async fn update_position(&self, account_id: Uuid, currency: &str, delta: rust_decimal::Decimal) -> Result<AssetPosition, AssetError> {
        let mut positions = self.positions.write().await;
        let entry = positions.entry(account_id).or_default();
        if let Some(pos) = entry.iter_mut().find(|p| p.currency_code == currency) {
            pos.balance += delta;
            pos.last_updated = chrono::Utc::now();
            Ok(pos.clone())
        } else {
            let new_pos = AssetPosition {
                account_id,
                asset_class: super::types::AssetClass::FiatCurrency,
                currency_code: currency.to_string(),
                balance: delta,
                reserved: rust_decimal::Decimal::ZERO,
                last_updated: chrono::Utc::now(),
            };
            entry.push(new_pos.clone());
            Ok(new_pos)
        }
    }
}
