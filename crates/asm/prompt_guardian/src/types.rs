use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Classification of an external input after sanitization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputClassification {
    /// Input is safe — passed to agent unchanged
    Benign,
    /// Input contained injection — sanitized version passed
    Sanitized,
    /// Input is malicious — blocked entirely
    Blocked,
}

/// Threat level assigned by the detection pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ThreatLevel {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

/// An external input that has been sanitized by PromptGuardian.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanitizedInput {
    pub input_id: Uuid,
    pub source: InputSource,
    pub original: String,
    pub sanitized: Option<String>,
    pub classification: InputClassification,
    pub threat_level: ThreatLevel,
    pub detected_patterns: Vec<String>,
    pub encoded_content_detected: bool,
    pub processed_at: chrono::DateTime<chrono::Utc>,
    pub forensic_log: Vec<SanitizerStep>,
}

/// Source of an external input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputSource {
    UserMessage,
    TransactionMemo,
    Email,
    WebPage,
    File,
    ToolOutput,
    AgentToAgent,
}

/// A single step in the sanitization pipeline (for forensic audit).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanitizerStep {
    pub step_name: String,
    pub outcome: String,
    pub elapsed_us: u64,
}
