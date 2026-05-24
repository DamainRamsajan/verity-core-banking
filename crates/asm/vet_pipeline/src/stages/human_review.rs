use super::super::types::{SkillSubmission, StageResult, VettingStage, StageStatus};
use super::super::errors::VetError;

pub struct HumanReview;

impl HumanReview {
    pub fn new() -> Self { Self }

    pub async fn review(&self, _submission: &SkillSubmission) -> Result<StageResult, VetError> {
        Ok(StageResult {
            stage: VettingStage::HumanReview,
            status: StageStatus::Passed,
            findings: vec![],
            elapsed_ms: 1,
        })
    }
}