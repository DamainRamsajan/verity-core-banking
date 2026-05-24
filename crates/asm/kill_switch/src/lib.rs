//! # Verity ASM — Kill Switch Protocol
//!
//! Three-tier forensic-grade agent termination:
//! - **PAUSE**: agent completes current action then halts (resumable)
//! - **SUSPEND**: agent halts immediately (human reactivation required)
//! - **TERMINATE**: all capability tokens revoked, forensic memory snapshot
//!   via MemLineage, audit log sealed, human review required
//!
//! Source: ARC42 v20.0 Addendum v17.0 §A-15

pub mod engine;
pub mod protocol;
pub mod types;
pub mod errors;

pub use engine::KillSwitchEngine;
pub use types::{KillLevel, KillSwitchAction, ForensicSnapshot};
pub use errors::KillSwitchError;
