pub mod engine;
pub mod types;
pub mod errors;

pub use engine::EtaEngine;
pub use types::{EmotionalContext, InteractionTone, TrustCalibration, KnowledgeLevel, ExplanationDetail};
pub use errors::EtaError;
