use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmotionalContext { FinancialStress, SecurityAnxiety, LifeMilestone, Routine }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteractionTone { Supportive, Reassuring, Encouraging, Neutral }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KnowledgeLevel { Low, Medium, High }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExplanationDetail { Concrete, Balanced, Abstract }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustCalibration {
    pub user_knowledge_level: KnowledgeLevel,
    pub recommended_tone: InteractionTone,
    pub explanation_detail: ExplanationDetail,
}
