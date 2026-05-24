use std::collections::HashMap;
use tokio::sync::RwLock;
use super::types::{EmotionalContext, InteractionTone, TrustCalibration, KnowledgeLevel, ExplanationDetail};
use super::errors::EtaError;

pub struct EtaEngine {
    user_profiles: RwLock<HashMap<uuid::Uuid, KnowledgeLevel>>,
}

impl EtaEngine {
    pub fn new() -> Self { Self { user_profiles: RwLock::new(HashMap::new()) } }

    pub async fn adapt(
        &self,
        user_id: uuid::Uuid,
        transaction_type: &str,
    ) -> Result<TrustCalibration, EtaError> {
        let profiles = self.user_profiles.read().await;
        let knowledge = profiles.get(&user_id).copied().unwrap_or(KnowledgeLevel::Medium);

        let context = match transaction_type {
            "overdraft" | "declined_payment" | "unexpected_fee" => EmotionalContext::FinancialStress,
            "flagged_transaction" | "new_device_login" | "large_transfer" => EmotionalContext::SecurityAnxiety,
            "mortgage_application" | "first_investment" | "savings_goal" => EmotionalContext::LifeMilestone,
            _ => EmotionalContext::Routine,
        };

        let tone = match context {
            EmotionalContext::FinancialStress | EmotionalContext::SecurityAnxiety => InteractionTone::Supportive,
            EmotionalContext::LifeMilestone => InteractionTone::Encouraging,
            EmotionalContext::Routine => InteractionTone::Neutral,
        };

        let explanation = match knowledge {
            KnowledgeLevel::Low => ExplanationDetail::Concrete,
            KnowledgeLevel::Medium => ExplanationDetail::Balanced,
            KnowledgeLevel::High => ExplanationDetail::Abstract,
        };

        Ok(TrustCalibration { user_knowledge_level: knowledge, recommended_tone: tone, explanation_detail: explanation })
    }

    pub async fn update_knowledge_level(&self, user_id: uuid::Uuid, level: KnowledgeLevel) {
        self.user_profiles.write().await.insert(user_id, level);
    }
}
