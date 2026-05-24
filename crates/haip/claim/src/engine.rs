use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;
use super::types::{CognitiveAction, Presentation, DefaultOption};
use super::budget::CognitiveBudget;
use super::errors::ClaimError;

pub struct ClaimEngine {
    budgets: RwLock<HashMap<Uuid, CognitiveBudget>>,
    config: ClaimConfig,
}

#[derive(Debug, Clone)]
pub struct ClaimConfig {
    pub daily_budget: u32,
    pub autonomous_threshold: u32,
}

impl Default for ClaimConfig {
    fn default() -> Self { Self { daily_budget: 200, autonomous_threshold: 5 } }
}

impl ClaimEngine {
    pub fn new(config: ClaimConfig) -> Self {
        Self { budgets: RwLock::new(HashMap::new()), config }
    }

    pub async fn present(&self, user_id: Uuid, action: CognitiveAction) -> Result<Presentation, ClaimError> {
        let mut budgets = self.budgets.write().await;
        let budget = budgets.entry(user_id).or_insert_with(|| CognitiveBudget::new(self.config.daily_budget));

        if budget.remaining < action.cognitive_cost.credits() {
            if action.risk_severity > 70 {
                return Ok(Presentation::FullEngagement { action: action.clone(), options: action.defaults.clone() });
            }
            return Err(ClaimError::CognitiveBudgetExceeded { remaining: budget.remaining, needed: action.cognitive_cost.credits() });
        }

        budget.consume(action.cognitive_cost.credits());

        if action.cognitive_cost.credits() <= self.config.autonomous_threshold {
            return Ok(Presentation::Autonomous);
        }

        let default = action.defaults.iter().find(|o| o.is_default).cloned().unwrap_or(DefaultOption {
            label: "Approve".into(), value: serde_json::Value::Null, is_default: true,
        });

        if action.risk_severity > 70 {
            Ok(Presentation::FullEngagement { action: action.clone(), options: action.defaults.clone() })
        } else {
            Ok(Presentation::EditConfirm { action: action.clone(), default_choice: default })
        }
    }

    pub async fn reset_budgets(&self) {
        let mut budgets = self.budgets.write().await;
        for budget in budgets.values_mut() { budget.reset(self.config.daily_budget); }
    }
}
