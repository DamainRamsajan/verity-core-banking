#[derive(Debug, thiserror::Error)]
pub enum AssetError {
    #[error("Currency not supported: {0}")]
    CurrencyNotSupported(String),
    #[error("FX rate unavailable for pair {base}/{quote}")]
    FxRateUnavailable { base: String, quote: String },
    #[error("Insufficient balance: {required} {currency} needed, {available} available")]
    InsufficientBalance { required: rust_decimal::Decimal, currency: String, available: rust_decimal::Decimal },
    #[error("Atomic swap failed: {0}")]
    AtomicSwapFailed(String),
}
