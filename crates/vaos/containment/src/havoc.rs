//! Havoc Oracle — models the AI as an unconstrained oracle.
//!
//! Under havoc oracle semantics, the AI can produce ANY element of the
//! typed action space. The containment layer must enforce the boundary
//! policy for every possible AI output.

use std::collections::HashSet;
use vaos_core::types::AgentAction;
use uuid::Uuid;

/// Generates all possible actions in a typed action space for
/// containment verification testing.
#[derive(Debug)]
pub struct HavocOracle {
    operations: Vec<String>,
    amounts: Vec<rust_decimal::Decimal>,
    agents: Vec<vaos_core::types::AgentId>,
}

impl HavocOracle {
    pub fn new(
        operations: Vec<String>,
        amounts: Vec<rust_decimal::Decimal>,
        agents: Vec<vaos_core::types::AgentId>,
    ) -> Self {
        Self { operations, amounts, agents }
    }

    /// Generate all possible actions in the action space.
    /// For small spaces, this is exhaustive; for large spaces,
    /// boundary-value sampling is used.
    pub fn generate_all(&self) -> Vec<AgentAction> {
        let mut actions = Vec::new();
        for op in &self.operations {
            for &amount in &self.amounts {
                for &agent in &self.agents {
                    actions.push(AgentAction {
                        id: Uuid::new_v4(),
                        initiator: agent,
                        action_type: op.clone(),
                        amount,
                        involved_agents: vec![agent],
                        payload: serde_json::Value::Null,
                        timestamp: chrono::Utc::now(),
                    });
                }
            }
        }
        actions
    }

    pub fn cardinality(&self) -> usize {
        self.operations.len() * self.amounts.len() * self.agents.len()
    }
}
