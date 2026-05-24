use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The cognitive cost of an agent interruption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CognitiveCost {
    Passive = 1,
    BinaryChoice = 5,
    MultiChoice = 15,
    OpenEnded = 50,
}

impl CognitiveCost {
    pub fn credits(&self) -> u32 {
        match self {
            Self::Passive => 1,
            Self::BinaryChoice => 5,
            Self::MultiChoice => 15,
            Self::OpenEnded => 50,
        }
    }
}

/// An action proposed by an agent that requires human attention.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveAction {
    pub id: Uuid,
    pub agent_id: vaos_core::types::AgentId,
    pub description: String,
    pub cognitive_cost: CognitiveCost,
    pub risk_severity: u8, // 1‑100
    pub defaults: Vec<DefaultOption>,
}

/// A pre‑computed reasonable default for the human to edit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultOption {
    pub label: String,
    pub value: serde_json::Value,
    pub is_default: bool,
}

/// The presentation form of an action (edit‑confirm, choice, or escalation).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Presentation {
    /// 80% of actions: agent shows what it will do, human confirms/edits
    EditConfirm {
        action: CognitiveAction,
        default_choice: DefaultOption,
    },
    /// 20% of actions: high‑stakes, requires full human engagement
    FullEngagement {
        action: CognitiveAction,
        options: Vec<DefaultOption>,
    },
    /// Agent handled autonomously (below cognitive budget)
    Autonomous,
}
