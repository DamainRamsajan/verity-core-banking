use std::collections::HashMap;
use tokio::sync::RwLock;

use super::types::{EmotionalContext, InteractionTone, TrustCalibration, KnowledgeLevel, ExplanationDetail};
use super::classifier::EmotionClassifier;
use super::tone::ToneAdapter;
use super::errors::EtaError;

/// Central ETA engine.
pub struct EtaEngine {
    classifier: EmotionClassifier,
    adapter: ToneAdapter,
    user_profiles: RwLock<HashMap<uuid::Uuid, KnowledgeLevel>>,
}

impl EtaEngine {
    pub fn new() -> Self {
        Self {
            classifier: EmotionClassifier::new(),
            adapter: ToneAdapter::new(),
            user_profiles: RwLock::new(HashMap::new()),
        }
    }

    /// Classify the emotional context of a transaction and return the appropriate tone.
    #[tracing::instrument(name = "eta.adapt", level = "info", skip(self))]
    pub async fn adapt(
        &self,
        user_id: uuid::Uuid,
        transaction_type: &str,
        amount: Option<rust_decimal::Decimal>,
    ) -> Result<TrustCalibration, EtaError> {
        let context = self.classifier.classify(transaction_type, amount);

        let profiles = self.user_profiles.read().await;
        let knowledge = profiles.get(&user_id).copied().unwrap_or(KnowledgeLevel::Medium);

        let tone = match context {
            EmotionalContext::FinancialStress | EmotionalContext::SecurityAnxiety => {
                InteractionTone::Supportive
            }
            EmotionalContext::LifeMilestone => InteractionTone::Encouraging,
            EmotionalContext::Routine => InteractionTone::Neutral,
        };

        let explanation = match knowledge {
            KnowledgeLevel::Low => ExplanationDetail::Concrete,
            KnowledgeLevel::Medium => ExplanationDetail::Balanced,
            KnowledgeLevel::High => ExplanationDetail::Abstract,
        };

        tracing::debug!(?context, ?tone, ?knowledge, "Emotional context adapted");

        Ok(TrustCalibration {
            user_knowledge_level: knowledge,
            recommended_tone: tone,
            explanation_detail: explanation,
        })
    }

    /// Update a user's financial knowledge level for trust calibration.
    pub async fn update_knowledge_level(
        &self,
        user_id: uuid::Uuid,
        level: KnowledgeLevel,
    ) {
        self.user_profiles.write().await.insert(user_id, level);
    }
}
