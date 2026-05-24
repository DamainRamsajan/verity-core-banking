use std::sync::Arc;
use tokio::sync::RwLock;

use super::types::{SandboxConfig, SandboxResult, SandboxBackend, McpToolDescriptor, ValidationStatus, SecurityEvent};
use super::backends::KavachBackend;
use super::mcp_validator::McpValidator;
use super::trajectory::TrajectoryAnalyzer;
use super::errors::GuardError;

pub struct ExecutionGuard {
    sandbox: KavachBackend,
    mcp_validator: McpValidator,
    trajectory: TrajectoryAnalyzer,
    config: GuardConfig,
    stats: RwLock<GuardStats>,
}

#[derive(Debug, Clone)]
pub struct GuardConfig {
    pub default_backend: SandboxBackend,
    pub max_runtime_ms: u64,
    pub max_memory_mb: u64,
    pub enable_trajectory_analysis: bool,
    pub boiling_frog_threshold: f64,
}

impl Default for GuardConfig {
    fn default() -> Self {
        Self { default_backend: SandboxBackend::Auto, max_runtime_ms: 30_000, max_memory_mb: 512, enable_trajectory_analysis: true, boiling_frog_threshold: 0.7 }
    }
}

#[derive(Debug, Default, Clone)]
pub struct GuardStats {
    pub executions: u64,
    pub blocked: u64,
    pub mcp_validations: u64,
    pub boiling_frog_detections: u64,
}

impl ExecutionGuard {
    pub fn new(config: GuardConfig) -> Self {
        Self {
            sandbox: KavachBackend::new(),
            mcp_validator: McpValidator::new(),
            trajectory: TrajectoryAnalyzer::new(config.boiling_frog_threshold),
            config,
            stats: RwLock::new(GuardStats::default()),
        }
    }

    /// Execute agent-generated code in a mandatory sandbox.
    #[tracing::instrument(name = "execguard.execute", level = "info", skip(self))]
    pub async fn execute(&self, code: &[u8], language: &str, sandbox_config: &SandboxConfig) -> Result<SandboxResult, GuardError> {
        let mut stats = self.stats.write().await;
        stats.executions += 1;

        // No fallback to unsandboxed — mandatory gVisor or equivalent
        let result = self.sandbox.run(code, language, sandbox_config).await?;

        // Trajectory analysis for Boiling the Frog detection
        if self.config.enable_trajectory_analysis {
            let cumulative_risk = self.trajectory.analyze(&result);
            if cumulative_risk > self.config.boiling_frog_threshold {
                stats.boiling_frog_detections += 1;
                tracing::warn!(cumulative_risk, "Boiling the Frog pattern detected");
            }
        }

        if !result.security_events.is_empty() {
            stats.blocked += 1;
            return Err(GuardError::SecurityViolation(result.security_events));
        }

        Ok(result)
    }

    /// Validate an MCP tool descriptor against the signed registry.
    #[tracing::instrument(name = "execguard.validate_mcp", level = "info", skip(self))]
    pub async fn validate_mcp_tool(&self, descriptor: &McpToolDescriptor) -> Result<ValidationStatus, GuardError> {
        let mut stats = self.stats.write().await;
        stats.mcp_validations += 1;
        self.mcp_validator.validate(descriptor).await
    }
}
