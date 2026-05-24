//! # Verity HAIP — Cognitive Load‑Aware Agent Interface (CLAIM)
//!
//! Manages human cognitive load by ensuring agents operate on a cognitive
//! budget model. Applies Hick's law, Miller's law, and default bias to
//! minimise cognitive friction.
//!
//! Source: ARC42 v20.0 Addendum v16.0 §A‑1

pub mod engine;
pub mod budget;
pub mod types;
pub mod errors;

pub use engine::ClaimEngine;
pub use budget::CognitiveBudget;
pub use types::{CognitiveAction, Presentation, CognitiveCost};
pub use errors::ClaimError;
