//! # Verity ASM — Financial Invariants Monitor (FIM)
//!
//! Companion service watching all agent-submitted ledger transactions.
//! Verifies that no agent has modified system parameters (credit limits,
//! fee structures) without a signed, human-approved policy change.
//!
//! Source: ARC42 v20.0 Addendum v17.0 §A-17

pub mod engine;
pub mod invariants;
pub mod types;
pub mod errors;

pub use engine::FinancialInvariantsMonitor;
pub use types::{ParameterChange, PolicyAuthorization, InvariantCheck};
pub use errors::FimError;
