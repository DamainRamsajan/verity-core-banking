//! # Verity ASM — RAMPART CI/CD Automated Adversarial Testing
//!
//! Embeds RAMPART (Risk Assessment and Measurement Platform for Agentic
//! Red Teaming, Microsoft open-sourced May 20, 2026) into CI/CD.
//! Every build is attacked against OWASP Agentic Top 10 test cases.
//!
//! Source: ARC42 v20.0 Addendum v17.0 §A-18

pub mod engine;
pub mod tests;
pub mod types;
pub mod errors;

pub use engine::RampartEngine;
pub use types::{RampartTest, RampartResult, OwasCategory};
pub use errors::RampartError;
