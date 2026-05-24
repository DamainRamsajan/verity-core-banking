//! # Verity Core Banking — Agent Marketplace
//!
//! Decentralised marketplace for AI agents with Token‑Curated Registry (TCR),
//! stake‑gated listing, slashing for misbehaviour, and cryptographic reputation.
//!
//! ## Architecture
//! - **TCR**: agents stake to be listed; challenged listings risk slashing
//! - **Staking/Slashing**: economic security — malicious agents lose their stake
//! - **Reputation**: Bayesian scoring from on‑chain behaviour, portable
//!   credentials across protocols (ERC‑8004 aligned)
//! - **Escrow**: on‑chain escrow for agent‑to‑agent payments
//!
//! ## References
//! - AgentGate — stake‑gated action microservice
//! - AgentProof — ERC‑8004 on‑chain reputation, 21+ chains
//! - Verifiable Reputation Staking (April 2026)
//! - CHEESE Agent Marketplace — on‑chain escrow
//!
//! Source: ARC42 v20.0 §3 VCBP Agent Marketplace

pub mod registry;
pub mod staking;
pub mod reputation;
pub mod escrow;
pub mod types;
pub mod errors;

pub use registry::TokenCuratedRegistry;
pub use staking::{StakingPool, SlashingCondition};
pub use reputation::ReputationEngine;
pub use escrow::EscrowEngine;
pub use types::{AgentListing, ListingStatus, ReputationScore};
pub use errors::MarketplaceError;
