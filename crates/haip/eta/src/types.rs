use serde::{Deserialize, Serialize};

/// The emotional salience of a financial moment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmotionalContext {
    FinancialStress,
    SecurityAnxiety,
    LifeMilestone,
    Routine,
}

/// The tone to use in the interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteractionTone {
    Supportive,
    Reassuring,
    Encouraging,
    Neutral,
}

/// Trust calibration based on anthropomorphism research.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustCalibration {
    pub user_knowledge_level: KnowledgeLevel,
    pub recommended_tone: InteractionTone,
    pub explanation_detail: ExplanationDetail,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KnowledgeLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExplanationDetail {
    Concrete,
    Balanced,
    Abstract,
}
