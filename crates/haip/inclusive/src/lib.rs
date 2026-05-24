//! # Verity HAIP — Inclusive Design System Backend
//!
//! Ensures Verity interfaces are usable by all populations: elderly users
//! (GABI‑validated), low‑literacy users, non‑native speakers, users with
//! disabilities (WCAG 2.2 AAA), and low‑connectivity environments.
//!
//! ## Standards
//! - **GABI Guide** (ICSE 2026 Distinguished Paper): large touch targets ≥48dp,
//!   high contrast ≥7:1, plain language ≤Grade 8, fear‑of‑errors barrier
//! - **WCAG 2.2 AAA**: all criteria including 2.5.8 (Target Size Minimum)
//! - **HKMA eight core principles** for elderly‑friendly banking
//! - **Accessibility Tree**: AI agents navigate via same tree as screen readers
//!   (~85% task success on accessible vs ~50% on inaccessible)
//!
//! Source: ARC42 v20.0 Addendum v16.0 §A-4

pub mod engine;
pub mod validator;
pub mod profile;
pub mod types;
pub mod errors;

pub use engine::InclusiveEngine;
pub use validator::AccessibilityValidator;
pub use profile::AccessibilityProfile;
pub use types::{AccessibilityFeature, ComplianceLevel};
pub use errors::InclusiveError;
