//! # Verity HAIP — Emotional Trust Architecture (ETA)
//!
//! Embeds emotional intelligence into agent interactions. Detects high‑stress
//! money moments and adapts the interface tone from clinical to supportive,
//! providing clear resolution pathways.
//!
//! ## Emotional Contexts
//! - **Financial Stress**: overdraft, declined payment, unexpected fee
//! - **Security Anxiety**: flagged transaction, new device login, large transfer
//! - **Life Milestone**: mortgage application, first investment, savings goal
//! - **Routine**: balance check, bill pay
//!
//! ## Principles
//! - **Apple Principle**: agent never deviates from stated plan without informing user
//! - **Construal Level Theory**: low‑knowledge users get concrete explanations;
//!   high‑knowledge users get abstract summaries
//! - **Anthropomorphism Calibration**: trust increases with cognitive+affective trust,
//!   but low‑knowledge users show inverse cognitive trust effect
//!
//! Source: ARC42 v20.0 Addendum v16.0 §A-2

pub mod engine;
pub mod classifier;
pub mod tone;
pub mod types;
pub mod errors;

pub use engine::EtaEngine;
pub use classifier::EmotionClassifier;
pub use tone::ToneAdapter;
pub use types::{EmotionalContext, InteractionTone, TrustCalibration};
pub use errors::EtaError;
