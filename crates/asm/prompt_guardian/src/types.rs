use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputClassification {
    Benign,
    Sanitized,
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ThreatLevel {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanitizerStep {
    pub step_name: String,
    pub outcome: String,
    pub elapsed_us: u64,
}
