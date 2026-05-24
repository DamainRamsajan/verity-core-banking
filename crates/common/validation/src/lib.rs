//! # Verity Common — Validation Utilities
//!
//! Shared validation functions used across all VCBP and VAOS crates.
//! Covers ISO 4217 currency codes, BIAN domain identifiers, regulatory
//! constraint validation, and account identifier formats.
//!
//! Source: ARC42 v20.0 §3 (all component contracts)

pub mod currency;
pub mod bian;
pub mod regulatory;
pub mod account;
pub mod types;
pub mod errors;

pub use types::{ValidationResult, ValidationContext};
pub use errors::ValidationError;
