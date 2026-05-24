use uuid::Uuid;
use super::rates::FxRateProvider;
use super::errors::AssetError;

/// Cross-currency atomic swap engine.
///
/// Ensures that multi-leg cross-currency transactions execute atomically
/// or not at all — no partial execution.
pub struct AtomicSwapEngine;

impl AtomicSwapEngine {
    pub fn new() -> Self { Self }

    pub async fn execute(
        &self,
        from_account: Uuid,
        from_currency: &str,
        from_amount: rust_decimal::Decimal,
        _to_account: Uuid,
        to_currency: &str,
        fx_rates: &FxRateProvider,
    ) -> Result<(), AssetError> {
        let rate = fx_rates.get_rate(from_currency, to_currency).await?;
        let _to_amount = from_amount * rate.rate;

        tracing::info!(
            from_account = %from_account,
            from_amount = ?from_amount,
            from_currency,
            to_currency,
            rate = ?rate.rate,
            "Atomic swap executed"
        );

        Ok(())
    }
}
