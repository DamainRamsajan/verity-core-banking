use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::types::{CognitiveAction, Presentation, DefaultOption};
use super::budget::CognitiveBudget;
use super::decision::DecisionPresenter;
use super::errors::ClaimError;

/// Central CLAIM engine.
pub struct ClaimEngine {
    /// Per‑user cognitive budgets
    budgets: RwLock<HashMap<Uuid, CognitiveBudget>>,
    presenter: DecisionPresenter,
    config: ClaimConfig,
}

#[derive(Debug, Clone)]
pub struct ClaimConfig {
    pub daily_budget: u32,
    pub autonomous_threshold: u32,
    pub auto_approve_ratio: f64, // 0.80 = 80% edit‑confirm
}

impl Default for ClaimConfig {
    fn default() -> Self {
        Self {
            daily_budget: 200,
            autonomous_threshold: 5,
            auto_approve_ratio: 0.80,
        }
    }
}

impl ClaimEngine {
    pub fn new(config: ClaimConfig) -> Self {
        Self {
            budgets: RwLock::new(HashMap::new()),
            presenter: DecisionPresenter::new(),
            config,
        }
    }

    /// Decide how to present an agent action to a human.
    ///
    /// Returns Autonomous (agent handles it), EditConfirm (human edits default),
    /// or FullEngagement (high‑stakes manual decision).
    #[tracing::instrument(name = "claim.present", level = "info", skip(self))]
    pub async fn present(
        &self,
        user_id: Uuid,
        action: CognitiveAction,
    ) -> Result<Presentation, ClaimError> {
        let mut budgets = self.budgets.write().await;
        let budget = budgets.entry(user_id).or_insert_with(|| CognitiveBudget::new(self.config.daily_budget));

        // 1. Check budget
        if budget.remaining < action.cognitive_cost.credits() {
            // Defer non‑urgent; escalate urgent
            if action.risk_severity > 70 {
                return Ok(Presentation::FullEngagement {
                    action: action.clone(),
                    options: action.defaults.clone(),
                });
            } else {
                return Err(ClaimError::CognitiveBudgetExceeded {
                    remaining: budget.remaining,
                    needed: action.cognitive_cost.credits(),
                });
            }
        }

        // 2. Deduct budget
        budget.consume(action.cognitive_cost.credits());

        // 3. Determine presentation level
        if action.cognitive_cost.credits() <= self.config.autonomous_threshold {
            // Agent can handle autonomously
            return Ok(Presentation::Autonomous);
        }

        // 4. 80/20 rule: high‑stakes = manual, low‑stakes = edit‑confirm
        if action.risk_severity > 70 || action.cognitive_cost.credits() >= 50 {
            Ok(Presentation::FullEngagement {
                action: action.clone(),
                options: self.presenter.chunk_options(&action.defaults, 7),
            })
        } else {
            let default = action.defaults.iter()
                .find(|o| o.is_default)
                .cloned()
                .unwrap_or(DefaultOption {
                    label: "Approve".into(),
                    value: serde_json::Value::Null,
                    is_default: true,
                });
            Ok(Presentation::EditConfirm {
                action: action.clone(),
                default_choice: default,
            })
        }
    }

    /// Reset daily budgets (called at midnight).
    pub async fn reset_budgets(&self) {
        let mut budgets = self.budgets.write().await;
        for budget in budgets.values_mut() {
            budget.reset(self.config.daily_budget);
        }
        tracing::info!("Cognitive budgets reset");
    }
}
