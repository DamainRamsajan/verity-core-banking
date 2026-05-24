//! # Verity Core Banking — Systemic Risk Engine
//!
//! IMF/ECB multilayer contagion model with five propagation channels.
//! Integrates with DFAST/CCAR stress testing for SIB identification.
//!
//! ## Propagation Channels
//! 1. **Counterparty exposures** — direct interbank lending
//! 2. **Short-term funding / roll-over risk** — liquidity contagion
//! 3. **Securities cross-holdings** — mark-to-market amplification
//! 4. **Common-asset fire-sale spillovers** — deleveraging spirals
//! 5. **NBFI market risk amplification** — shadow banking contagion
//!
//! ## References
//! - IMF WP/26/xx (Feb 2026): Risk Propagation with NBFI Amplification
//! - ECB multilayer interbank model (Feb 2026): Granular 4-channel
//! - Gai-Kapadia (2010): Default cascade simulation
//! - SIB identification under dynamic credit easing (April 2026)
//!
//! Source: ARC42 v20.0 §3 VCBP Systemic Risk Engine

pub mod engine;
pub mod models;
pub mod cascade;
pub mod sib;
pub mod types;
pub mod errors;

pub use engine::SystemicRiskEngine;
pub use cascade::GaiKapadiaSimulator;
pub use sib::SibIdentifier;
pub use types::{FinancialNetwork, ContagionResult, RiskChannel};
pub use errors::RiskError;
