use super::types::SandboxResult;

pub struct TrajectoryAnalyzer { threshold: f64, cumulative_risk: f64, turn_count: u64 }

impl TrajectoryAnalyzer {
    pub fn new(threshold: f64) -> Self { Self { threshold, cumulative_risk: 0.0, turn_count: 0 } }
    pub fn analyze(&mut self, result: &SandboxResult) -> f64 {
        self.turn_count += 1;
        let turn_risk = result.security_events.len() as f64 * 0.1;
        self.cumulative_risk += turn_risk;
        self.cumulative_risk
    }
}
