//! # Verity Core Banking — Multi-Asset Merkle Ledger Extension
//!
//! Extends the Merkle Double-Entry Ledger to support multiple currencies,
//! digital assets, tokenized instruments, and tokenized deposits — all
//! tracked in the same cryptographic ledger.
//!
//! ## Supported Assets
//! - Fiat currencies (USD, EUR, GBP, JPY, CHF, etc.) per ISO 4217
//! - Tokenized deposits (JPM Coin via Canton Network, CBDC via Pontes)
//! - Digital assets (Bitcoin, Ethereum, stablecoins)
//! - Tokenized securities (bonds, equities)
//! - Precious metals (gold, silver — tokenized)
//!
//! ## Features
//! - FX rate feed integration with real-time cross-currency valuation
//! - Cross-currency atomic swaps (no partial execution)
//! - All assets share the same Merkle proof infrastructure
//! - FATF Travel Rule compliance tagging per asset class
//!
//! Source: ARC42 v20.0 §3 VCBP Multi-Asset Ledger Extension

pub mod engine;
pub mod currencies;
pub mod rates;
pub mod swap;
pub mod types;
pub mod errors;

pub use engine::MultiAssetEngine;
pub use types::{AssetClass, AssetPosition, CurrencyPair};
pub use errors::AssetError;
