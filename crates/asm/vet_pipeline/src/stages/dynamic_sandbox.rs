use super::super::types::{SkillSubmission, StageResult, VettingStage, StageStatus};
use super::super::errors::VetError;

pub struct DynamicSandbox;

impl DynamicSandbox {
    pub fn new() -> Self { Self }

    pub async fn execute(&self, _submission: &SkillSubmission) -> Result<StageResult, VetError> {
        Ok(StageResult {
            stage: VettingStage::DynamicSandbox,
            status: StageStatus::Passed,
            findings: vec![],
            elapsed_ms: 1,
        })
    }
}