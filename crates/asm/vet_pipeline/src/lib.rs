//! # Verity ASM — VetPipeline Marketplace Skill Vetting
//!
//! Four-stage vetting pipeline for agent skills: Static Analysis → Dynamic
//! Sandbox → Semantic Payload Scan → Human Review. Skills failing any
//! stage are rejected with forensic detail.
//!
//! ## Stages
//! 1. **Static Analysis**: CodeQL for code patterns, NL payload scanning
//! 2. **Dynamic Sandbox**: Execution with honeytokens, monitored for I/O
//! 3. **Semantic Scanner**: Fine-tuned transformer detecting hidden
//!    instructions in unstructured natural language (trained on SCH
//!    and Trojan Hippo examples)
//! 4. **Human Review**: Mandatory for financial operations, code generation,
//!    system configuration
//!
//! Source: ARC42 v20.0 Addendum v17.0 §A-13

pub mod engine;
pub mod stages;
pub mod types;
pub mod errors;

pub use engine::VetPipeline;
pub use types::{SkillSubmission, VettingResult, VettingStage, StageStatus};
pub use errors::VetError;
