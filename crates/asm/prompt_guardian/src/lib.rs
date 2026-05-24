//! # Verity ASM — PromptGuardian Input Sanitization
//!
//! Filters and sanitises all external inputs before they reach any agent's
//! reasoning core. Implements the PromptGuard 4-layer framework (Nature
//! Scientific Reports, Jan 2026): input filtering, structured formatting,
//! output validation, and adaptive response refinement.
//!
//! ## Detection Engines
//! - **JailGuard** v1.0: pure-Rust MLP classifier, 98.40% accuracy,
//!   p50 14ms CPU inference, 1.5MB embedded model
//! - **llm-guard**: zero-copy scanners for invisible text, role-override,
//!   secret leakage, token limit
//! - **Armorer-Guard**: fast local scanner for prompt injection,
//!   credential leaks, exfiltration, risky tool calls
//!
//! ## Encoded Content Detection
//! Morse code, Base64, hex, and other encoding schemes are decoded and
//! re-analyzed before reaching the agent. The Bankr/Grok attack (Morse code
//! social media prompt injection, April 2026) is specifically defended.
//!
//! Source: ARC42 v20.0 Addendum v17.0 §A-10

pub mod engine;
pub mod sanitizers;
pub mod types;
pub mod errors;

pub use engine::PromptGuardian;
pub use types::{InputClassification, SanitizedInput, ThreatLevel};
pub use errors::GuardianError;
