use super::super::types::{SkillSubmission, StageResult, VettingStage, StageStatus};
use super::super::errors::VetError;

pub struct StaticAnalyzer;

impl StaticAnalyzer {
    pub fn new() -> Self { Self }

    pub async fn analyze(&self, _submission: &SkillSubmission) -> Result<StageResult, VetError> {
        Ok(StageResult {
            stage: VettingStage::StaticAnalysis,
            status: StageStatus::Passed,
            findings: vec![],
            elapsed_ms: 1,
        })
    }
}