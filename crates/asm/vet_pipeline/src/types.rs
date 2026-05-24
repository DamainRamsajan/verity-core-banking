use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillSubmission {
    pub submission_id: Uuid,
    pub agent_id: vaos_core::types::AgentId,
    pub name: String,
    pub description: String,
    pub skill_md: String,
    pub executable_payload: Vec<u8>,
    pub submitted_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VettingStage { StaticAnalysis, DynamicSandbox, SemanticScan, HumanReview }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StageStatus { Pending, Passed, Failed, Skipped }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VettingResult {
    pub submission_id: Uuid,
    pub overall_status: StageStatus,
    pub stages: Vec<StageResult>,
    pub signed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageResult {
    pub stage: VettingStage,
    pub status: StageStatus,
    pub findings: Vec<String>,
    pub elapsed_ms: u64,
}
