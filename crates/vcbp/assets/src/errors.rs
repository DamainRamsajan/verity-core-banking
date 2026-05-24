#[derive(Debug, thiserror::Error)]
pub enum AssetError {
    #[error("Currency not supported: {0}")]
    CurrencyNotSupported(String),
    #[error("FX rate unavailable for pair {base}/{quote}")]
    FxRateUnavailable { base: String, quote: String },
}
