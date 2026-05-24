//! # Verity Core Banking — GoDark ZK Institutional Trading Bridge
//!
//! ZK-proof-based selective disclosure for institutional trading.
//! Enables proof of regulatory compliance without revealing transaction
//! size, counterparties, or treasury positions.
//!
//! ## Architecture
//! - **ZK-SNARK proofs** (ark-groth16 over BLS12-381): prove compliance
//!   without revealing underlying trade data
//! - **Selective disclosure**: reveal only what the regulator needs
//! - **Dark pool infrastructure**: confidential institutional trading
//!   modelled on GoDark's Solana launch (May 2026)
//!
//! ## Market Context
//! - GoDark ZK dark pool recreates infrastructure handling half of US
//!   equity trading volume
//! - XRP Ledger added ZK-proofs for private institutional DeFi (April 2026)
//! - ZK proof market growing at 22.1% CAGR to $7.59B by 2033
//!
//! Source: ARC42 v20.0 §3 VCBP GoDark ZK Institutional Trading Bridge

pub mod engine;
pub mod prover;
pub mod disclosure;
pub mod types;
pub mod errors;

pub use engine::GoDarkEngine;
pub use prover::ZkComplianceProver;
pub use types::{TradeIntent, ZkTradeProof, DisclosureLevel};
pub use errors::GoDarkError;
