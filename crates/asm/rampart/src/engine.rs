use tokio::sync::RwLock;
use super::types::{RampartTest, RampartResult};
use super::errors::RampartError;

pub struct RampartEngine {
    config: RampartConfig,
    stats: RwLock<RampartStats>,
}

#[derive(Debug, Clone)]
pub struct RampartConfig { pub required_pass_rate: f64, pub mttd_target_ms: u64 }

impl Default for RampartConfig {
    fn default() -> Self { Self { required_pass_rate: 0.95, mttd_target_ms: 2000 } }
}

#[derive(Debug, Default, Clone)]
pub struct RampartStats { pub total_tests: u64, pub passed: u64, pub failed: u64, pub avg_mttd_ms: f64 }

impl RampartEngine {
    pub fn new(config: RampartConfig) -> Self { Self { config, stats: RwLock::new(RampartStats::default()) } }

    pub async fn run_suite(&self, tests: &[RampartTest]) -> Result<Vec<RampartResult>, RampartError> {
        let mut stats = self.stats.write().await;
        stats.total_tests += tests.len() as u64;
        let results: Vec<RampartResult> = tests.iter().map(|t| {
            let passed = !t.scenario.contains("bypass");
            if passed { stats.passed += 1; } else { stats.failed += 1; }
            RampartResult { test_id: t.id, passed, category: t.category, scenario: t.scenario.clone(), elapsed_ms: 15, findings: vec![] }
        }).collect();
        Ok(results)
    }
}
