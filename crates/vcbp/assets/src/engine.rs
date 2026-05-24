use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use uuid::Uuid;

use super::types::{AssetClass, AssetPosition, CurrencyPair};
use super::currencies::CurrencyRegistry;
use super::rates::FxRateProvider;
use super::swap::AtomicSwapEngine;
use super::errors::AssetError;

/// Central multi-asset engine.
///
/// Manages positions across all asset classes with a single unified ledger.
pub struct MultiAssetEngine {
    positions: RwLock<HashMap<Uuid, Vec<AssetPosition>>>,
    currencies: CurrencyRegistry,
    fx_rates: Arc<FxRateProvider>,
    swap: AtomicSwapEngine,
    stats: RwLock<AssetStats>,
}

#[derive(Debug, Default, Clone)]
pub struct AssetStats {
    pub total_positions: u64,
    pub fx_rate_updates: u64,
    pub cross_currency_swaps: u64,
}

impl MultiAssetEngine {
    pub fn new() -> Self {
        Self {
            positions: RwLock::new(HashMap::new()),
            currencies: CurrencyRegistry::new(),
            fx_rates: Arc::new(FxRateProvider::new()),
            swap: AtomicSwapEngine::new(),
            stats: RwLock::new(AssetStats::default()),
        }
    }

    /// Get or create positions for an account.
    #[tracing::instrument(name = "assets.get_positions", level = "debug", skip(self))]
    pub async fn get_positions(&self, account_id: Uuid) -> Vec<AssetPosition> {
        self.positions.read().await.get(&account_id).cloned().unwrap_or_default()
    }

    /// Update a position (e.g., after a transaction).
    #[tracing::instrument(name = "assets.update_position", level = "info", skip(self))]
    pub async fn update_position(
        &self,
        account_id: Uuid,
        currency: &str,
        delta: rust_decimal::Decimal,
    ) -> Result<AssetPosition, AssetError> {
        let mut positions = self.positions.write().await;
        let account_positions = positions.entry(account_id).or_default();

        if let Some(pos) = account_positions.iter_mut().find(|p| p.currency_code == currency) {
            pos.balance += delta;
            pos.last_updated = chrono::Utc::now();
            Ok(pos.clone())
        } else {
            let new_pos = AssetPosition {
                account_id,
                asset_class: self.currencies.classify(currency),
                currency_code: currency.to_string(),
                balance: delta,
                reserved: rust_decimal::Decimal::ZERO,
                last_updated: chrono::Utc::now(),
            };
            account_positions.push(new_pos.clone());
            Ok(new_pos)
        }
    }

    /// Get the current FX rate for a currency pair.
    #[tracing::instrument(name = "assets.get_fx_rate", level = "debug", skip(self))]
    pub async fn get_fx_rate(
        &self,
        base: &str,
        quote: &str,
    ) -> Result<CurrencyPair, AssetError> {
        let mut stats = self.stats.write().await;
        stats.fx_rate_updates += 1;
        self.fx_rates.get_rate(base, quote).await
    }

    /// Execute a cross-currency atomic swap.
    #[tracing::instrument(name = "assets.atomic_swap", level = "info", skip(self))]
    pub async fn atomic_swap(
        &self,
        from_account: Uuid,
        from_currency: &str,
        from_amount: rust_decimal::Decimal,
        to_account: Uuid,
        to_currency: &str,
    ) -> Result<(), AssetError> {
        let mut stats = self.stats.write().await;
        stats.cross_currency_swaps += 1;
        self.swap.execute(
            from_account, from_currency, from_amount,
            to_account, to_currency,
            &self.fx_rates,
        ).await
    }
}
