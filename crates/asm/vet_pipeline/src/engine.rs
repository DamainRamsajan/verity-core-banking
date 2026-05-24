use tokio::sync::RwLock;

use super::types::{SkillSubmission, VettingResult, StageStatus, StageResult};
use super::stages::{StaticAnalyzer, DynamicSandbox, SemanticScanner, HumanReview};
use super::errors::VetError;

#[allow(dead_code)]
pub struct VetPipeline {
    static_analyzer: StaticAnalyzer,
    dynamic_sandbox: DynamicSandbox,
    semantic_scanner: SemanticScanner,
    human_review: HumanReview,
    config: VetConfig,
    stats: RwLock<VetStats>,
}

#[derive(Debug, Clone)]
pub struct VetConfig {
    pub require_all_stages: bool,
    pub auto_pass_semantic: bool,
    pub human_review_threshold: u8,
}

impl Default for VetConfig {
    fn default() -> Self { Self { require_all_stages: true, auto_pass_semantic: false, human_review_threshold: 7 } }
}

#[derive(Debug, Default, Clone)]
pub struct VetStats { pub submissions: u64, pub approved: u64, pub rejected: u64 }

impl VetPipeline {
    pub fn new(config: VetConfig) -> Self {
        Self {
            static_analyzer: StaticAnalyzer::new(),
            dynamic_sandbox: DynamicSandbox::new(),
            semantic_scanner: SemanticScanner::new(),
            human_review: HumanReview::new(),
            config,
            stats: RwLock::new(VetStats::default()),
        }
    }

    #[tracing::instrument(name = "vetpipeline.vet", level = "info", skip(self))]
    pub async fn vet(&self, submission: &SkillSubmission) -> Result<VettingResult, VetError> {
        let mut stats = self.stats.write().await;
        stats.submissions += 1;
        let mut stages = Vec::new();

        // Stage 1: Static Analysis
        let s1 = self.static_analyzer.analyze(submission).await?;
        stages.push(s1.clone());
        if s1.status == StageStatus::Failed { stats.rejected += 1; return Ok(self.result(submission.submission_id, StageStatus::Failed, stages)); }

        // Stage 2: Dynamic Sandbox
        let s2 = self.dynamic_sandbox.execute(submission).await?;
        stages.push(s2.clone());
        if s2.status == StageStatus::Failed { stats.rejected += 1; return Ok(self.result(submission.submission_id, StageStatus::Failed, stages)); }

        // Stage 3: Semantic Scan
        let s3 = self.semantic_scanner.scan(submission).await?;
        stages.push(s3.clone());
        if s3.status == StageStatus::Failed { stats.rejected += 1; return Ok(self.result(submission.submission_id, StageStatus::Failed, stages)); }

        // Stage 4: Human Review (mandatory for high-risk)
        let s4 = self.human_review.review(submission).await?;
        stages.push(s4.clone());

        let overall = if s4.status == StageStatus::Failed { StageStatus::Failed } else { StageStatus::Passed };
        if overall == StageStatus::Passed { stats.approved += 1; } else { stats.rejected += 1; }

        Ok(self.result(submission.submission_id, overall, stages))
    }

    fn result(&self, id: uuid::Uuid, status: StageStatus, stages: Vec<StageResult>) -> VettingResult {
        VettingResult { submission_id: id, overall_status: status, stages, signed: status == StageStatus::Passed }
    }
}
