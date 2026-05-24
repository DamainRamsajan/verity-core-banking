use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveAction {
    pub id: Uuid,
    pub agent_id: vaos_core::types::AgentId,
    pub description: String,
    pub cognitive_cost: CognitiveCost,
    pub risk_severity: u8,
    pub defaults: Vec<DefaultOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultOption {
    pub label: String,
    pub value: serde_json::Value,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Presentation {
    EditConfirm {
        action: CognitiveAction,
        default_choice: DefaultOption,
    },
    FullEngagement {
        action: CognitiveAction,
        options: Vec<DefaultOption>,
    },
    Autonomous,
}
