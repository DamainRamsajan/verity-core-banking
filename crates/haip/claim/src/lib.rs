//! # Verity HAIP — Cognitive Load-Aware Agent Interface (CLAIM)
//!
//! Manages human cognitive load by ensuring agents operate on a cognitive
//! budget model. Agents only interrupt human supervisors when the cognitive
//! cost of the interruption is justified by the risk of inaction.
//!
//! ## Design Principles
//! - **Cognitive Credits**: passive = 1, binary choice = 5, open-ended = 50
//! - **Reasonable Default**: always present an edit‑confirm pattern
//!   (recognition is low load; creation is high load)
//! - **Hick's Law**: ≤3 options by default, progressive disclosure
//! - **Miller's Law**: chunk information into 7±2 items
//! - **Default Bias**: pre‑select safe defaults
//!
//! Source: ARC42 v20.0 Addendum v16.0 §A-1

pub mod engine;
pub mod budget;
pub mod decision;
pub mod types;
pub mod errors;

pub use engine::ClaimEngine;
pub use budget::CognitiveBudget;
pub use decision::DecisionPresenter;
pub use types::{CognitiveAction, Presentation, CognitiveCost};
pub use errors::ClaimError;
