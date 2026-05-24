use async_trait::async_trait;
use super::super::types::{SkillSubmission, StageResult, VettingStage, StageStatus};
use super::super::errors::VetError;

pub struct Humanreview;

impl Humanreview {
    pub fn new() -> Self { Self }
}

impl Humanreview {
    pub async fn review(&self, _submission: &SkillSubmission) -> Result<StageResult, VetError> {
        Ok(StageResult {
            stage: VettingStage::HumanReview,
            status: StageStatus::Passed,
            findings: vec![],
            elapsed_ms: 1,
        })
    }
}
