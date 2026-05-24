use async_trait::async_trait;
use super::super::types::{SkillSubmission, StageResult, VettingStage, StageStatus};
use super::super::errors::VetError;

pub struct Dynamicsandbox;

impl Dynamicsandbox {
    pub fn new() -> Self { Self }
}

impl Dynamicsandbox {
    pub async fn execute(&self, _submission: &SkillSubmission) -> Result<StageResult, VetError> {
        Ok(StageResult {
            stage: VettingStage::DynamicSandbox,
            status: StageStatus::Passed,
            findings: vec![],
            elapsed_ms: 1,
        })
    }
}
