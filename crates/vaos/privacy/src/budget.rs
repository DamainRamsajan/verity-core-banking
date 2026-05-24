//! Privacy budget tracking — ε and δ consumption over time.

/// Tracks the remaining differential privacy budget.
#[derive(Debug, Clone)]
pub struct PrivacyBudget {
    pub total_epsilon: f64,
    pub total_delta: f64,
    pub remaining_epsilon: f64,
    pub remaining_delta: f64,
    pub total_consumed: f64,
}

impl PrivacyBudget {
    pub fn new(epsilon: f64, delta: f64) -> Self {
        Self {
            total_epsilon: epsilon,
            total_delta: delta,
            remaining_epsilon: epsilon,
            remaining_delta: delta,
            total_consumed: 0.0,
        }
    }

    /// Percentage of budget consumed.
    pub fn consumed_pct(&self) -> f64 {
        if self.total_epsilon == 0.0 { 100.0 }
        else { (self.total_consumed / self.total_epsilon) * 100.0 }
    }
}
