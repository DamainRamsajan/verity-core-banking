use super::super::types::{SkillSubmission, StageResult, VettingStage, StageStatus};
use super::super::errors::VetError;

pub struct SemanticScanner;

impl SemanticScanner {
    pub fn new() -> Self { Self }

    pub async fn scan(&self, _submission: &SkillSubmission) -> Result<StageResult, VetError> {
        Ok(StageResult {
            stage: VettingStage::SemanticScan,
            status: StageStatus::Passed,
            findings: vec![],
            elapsed_ms: 1,
        })
    }
}