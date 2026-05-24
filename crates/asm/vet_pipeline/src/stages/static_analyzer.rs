use async_trait::async_trait;
use super::super::types::{SkillSubmission, StageResult, VettingStage, StageStatus};
use super::super::errors::VetError;

pub struct Staticanalyzer;

impl Staticanalyzer {
    pub fn new() -> Self { Self }
}

impl Staticanalyzer {
    pub async fn analyze(&self, _submission: &SkillSubmission) -> Result<StageResult, VetError> {
        Ok(StageResult {
            stage: VettingStage::StaticAnalysis,
            status: StageStatus::Passed,
            findings: vec![],
            elapsed_ms: 1,
        })
    }
}
