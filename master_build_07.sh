#!/bin/bash
set -e

echo "============================================"
echo "  MASTER BUILD 07 – Block 6: Human‑Agent Interaction Plane"
echo "============================================"

# -------------------------------------------------------
# 1. haip/claim — Cognitive Load‑Aware Agent Interface
# -------------------------------------------------------
cat > crates/haip/claim/Cargo.toml << 'CEOF'
[package]
name = "haip-claim"
version.workspace = true
edition.workspace = true
license.workspace = true
description = "Verity HAIP — Cognitive Load-Aware Agent Interface (CLAIM)"

[dependencies]
vaos-core = { path = "../../vaos/core" }
tokio.workspace = true
serde.workspace = true
serde_json.workspace = true
tracing.workspace = true
thiserror.workspace = true
uuid.workspace = true
chrono.workspace = true
async-trait.workspace = true
CEOF

cat > crates/haip/claim/src/lib.rs << 'RSEOF'
//! # Verity HAIP — Cognitive Load‑Aware Agent Interface (CLAIM)
//!
//! Manages human cognitive load by ensuring agents operate on a cognitive
//! budget model. Applies Hick's law, Miller's law, and default bias to
//! minimise cognitive friction.
//!
//! Source: ARC42 v20.0 Addendum v16.0 §A‑1

pub mod engine;
pub mod budget;
pub mod types;
pub mod errors;

pub use engine::ClaimEngine;
pub use budget::CognitiveBudget;
pub use types::{CognitiveAction, Presentation, CognitiveCost};
pub use errors::ClaimError;
RSEOF

cat > crates/haip/claim/src/types.rs << 'RSEOF'
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
RSEOF

cat > crates/haip/claim/src/budget.rs << 'RSEOF'
#[derive(Debug, Clone)]
pub struct CognitiveBudget {
    pub daily_limit: u32,
    pub remaining: u32,
}

impl CognitiveBudget {
    pub fn new(daily_limit: u32) -> Self { Self { daily_limit, remaining: daily_limit } }
    pub fn consume(&mut self, credits: u32) { self.remaining = self.remaining.saturating_sub(credits); }
    pub fn reset(&mut self, limit: u32) { self.daily_limit = limit; self.remaining = limit; }
}
RSEOF

cat > crates/haip/claim/src/engine.rs << 'RSEOF'
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;
use super::types::{CognitiveAction, Presentation, CognitiveCost, DefaultOption};
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
RSEOF

cat > crates/haip/claim/src/errors.rs << 'RSEOF'
#[derive(Debug, thiserror::Error)]
pub enum ClaimError {
    #[error("Cognitive budget exceeded: {remaining} remaining, {needed} needed")]
    CognitiveBudgetExceeded { remaining: u32, needed: u32 },
}
RSEOF

echo "  ✓ haip/claim — CLAIM"

# -------------------------------------------------------
# 2. haip/eta — Emotional Trust Architecture
# -------------------------------------------------------
cat > crates/haip/eta/Cargo.toml << 'CEOF'
[package]
name = "haip-eta"
version.workspace = true
edition.workspace = true
license.workspace = true
description = "Verity HAIP — Emotional Trust Architecture (ETA)"

[dependencies]
vaos-core = { path = "../../vaos/core" }
tokio.workspace = true
serde.workspace = true
serde_json.workspace = true
tracing.workspace = true
thiserror.workspace = true
uuid.workspace = true
chrono.workspace = true
async-trait.workspace = true
CEOF

cat > crates/haip/eta/src/lib.rs << 'RSEOF'
pub mod engine;
pub mod types;
pub mod errors;

pub use engine::EtaEngine;
pub use types::{EmotionalContext, InteractionTone, TrustCalibration, KnowledgeLevel, ExplanationDetail};
pub use errors::EtaError;
RSEOF

cat > crates/haip/eta/src/types.rs << 'RSEOF'
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
RSEOF

cat > crates/haip/eta/src/engine.rs << 'RSEOF'
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
RSEOF

cat > crates/haip/eta/src/errors.rs << 'RSEOF'
#[derive(Debug, thiserror::Error)]
pub enum EtaError { #[error("Classification failed")] ClassificationFailed }
RSEOF

echo "  ✓ haip/eta — ETA"

# -------------------------------------------------------
# 3. haip/dashboard — Delegative Governance Dashboard Backend
# -------------------------------------------------------
cat > crates/haip/dashboard/Cargo.toml << 'CEOF'
[package]
name = "haip-dashboard"
version.workspace = true
edition.workspace = true
license.workspace = true
description = "Verity HAIP — Delegative Governance Dashboard Backend"

[dependencies]
vaos-core = { path = "../../vaos/core" }
tokio.workspace = true
serde.workspace = true
serde_json.workspace = true
tracing.workspace = true
thiserror.workspace = true
uuid.workspace = true
chrono.workspace = true
rust_decimal.workspace = true
async-trait.workspace = true
CEOF

cat > crates/haip/dashboard/src/lib.rs << 'RSEOF'
pub mod engine;
pub mod types;
pub mod errors;

pub use engine::DashboardEngine;
pub use types::{AgentBoundary, ActivityEvent, OverrideAction};
pub use errors::DashboardError;
RSEOF

cat > crates/haip/dashboard/src/types.rs << 'RSEOF'
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vaos_core::types::AgentId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentBoundary {
    pub agent_id: AgentId,
    pub spending_limit: rust_decimal::Decimal,
    pub approval_threshold: rust_decimal::Decimal,
    pub allowed_operations: Vec<String>,
}

impl Default for AgentBoundary {
    fn default() -> Self {
        Self {
            agent_id: AgentId::new(),
            spending_limit: rust_decimal::Decimal::new(1000, 0),
            approval_threshold: rust_decimal::Decimal::new(500, 0),
            allowed_operations: vec!["balance_inquiry".into()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityEvent {
    pub event_id: Uuid,
    pub agent_id: AgentId,
    pub action: String,
    pub amount: Option<rust_decimal::Decimal>,
    pub risk_score: f64,
    pub within_boundary: bool,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OverrideAction { Approve, Reject, RevokeToken, SuspendAgent, TerminateAgent }
RSEOF

cat > crates/haip/dashboard/src/engine.rs << 'RSEOF'
use std::collections::HashMap;
use tokio::sync::RwLock;
use vaos_core::types::AgentId;
use super::types::{AgentBoundary, ActivityEvent, OverrideAction};
use super::errors::DashboardError;

pub struct DashboardEngine {
    policies: RwLock<HashMap<AgentId, AgentBoundary>>,
    activity_feed: RwLock<Vec<ActivityEvent>>,
}

impl DashboardEngine {
    pub fn new() -> Self { Self { policies: RwLock::new(HashMap::new()), activity_feed: RwLock::new(Vec::new()) } }

    pub async fn set_boundaries(&self, agent_id: AgentId, boundary: AgentBoundary) -> Result<(), DashboardError> {
        self.policies.write().await.insert(agent_id, boundary);
        Ok(())
    }

    pub async fn check_action(
        &self,
        agent_id: AgentId,
        action: &str,
        amount: Option<rust_decimal::Decimal>,
    ) -> Result<bool, DashboardError> {
        let policies = self.policies.read().await;
        let boundary = policies.get(&agent_id).ok_or(DashboardError::AgentNotConfigured(agent_id))?;
        if !boundary.allowed_operations.iter().any(|op| op == action) { return Ok(false); }
        if let (Some(amt), limit) = (amount, boundary.spending_limit) {
            if amt > limit { return Ok(false); }
        }
        Ok(true)
    }

    pub async fn record_activity(&self, event: ActivityEvent) {
        self.activity_feed.write().await.push(event);
    }
}
RSEOF

cat > crates/haip/dashboard/src/errors.rs << 'RSEOF'
#[derive(Debug, thiserror::Error)]
pub enum DashboardError {
    #[error("Agent not configured: {0:?}")]
    AgentNotConfigured(vaos_core::types::AgentId),
}
RSEOF

echo "  ✓ haip/dashboard — Delegative Governance"

# -------------------------------------------------------
# 4. haip/inclusive — Inclusive Design System Backend
# -------------------------------------------------------
cat > crates/haip/inclusive/Cargo.toml << 'CEOF'
[package]
name = "haip-inclusive"
version.workspace = true
edition.workspace = true
license.workspace = true
description = "Verity HAIP — Inclusive Design System Backend"

[dependencies]
vaos-core = { path = "../../vaos/core" }
tokio.workspace = true
serde.workspace = true
serde_json.workspace = true
tracing.workspace = true
thiserror.workspace = true
uuid.workspace = true
chrono.workspace = true
async-trait.workspace = true
CEOF

cat > crates/haip/inclusive/src/lib.rs << 'RSEOF'
pub mod engine;
pub mod types;
pub mod errors;

pub use engine::InclusiveEngine;
pub use types::{AccessibilityProfile, AccessibilityFeature, ComplianceLevel};
pub use errors::InclusiveError;
RSEOF

cat > crates/haip/inclusive/src/types.rs << 'RSEOF'
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityProfile {
    pub user_id: Uuid,
    pub features: Vec<AccessibilityFeature>,
    pub language: String,
    pub offline_preferred: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessibilityFeature {
    LargeText, HighContrast, ScreenReader, VoiceInput,
    SimplifiedUI, PlainLanguage, ReducedMotion, KeyboardOnly,
    SwitchControl, OfflineMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceLevel { A, AA, AAA, GabiEnhanced }
RSEOF

cat > crates/haip/inclusive/src/engine.rs << 'RSEOF'
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;
use super::types::{AccessibilityProfile, ComplianceLevel};
use super::errors::InclusiveError;

pub struct InclusiveEngine {
    profiles: RwLock<HashMap<Uuid, AccessibilityProfile>>,
}

impl InclusiveEngine {
    pub fn new() -> Self { Self { profiles: RwLock::new(HashMap::new()) } }

    pub async fn register_profile(&self, profile: AccessibilityProfile) -> Result<(), InclusiveError> {
        let mut profiles = self.profiles.write().await;
        profiles.insert(profile.user_id, profile);
        Ok(())
    }

    pub async fn check_interface(&self, user_id: Uuid, compliance: ComplianceLevel) -> Result<bool, InclusiveError> {
        let profiles = self.profiles.read().await;
        let profile = profiles.get(&user_id).ok_or(InclusiveError::ProfileNotFound(user_id))?;
        if profile.features.contains(&super::types::AccessibilityFeature::ScreenReader) && compliance != ComplianceLevel::AAA {
            return Ok(false);
        }
        Ok(true)
    }
}
RSEOF

cat > crates/haip/inclusive/src/errors.rs << 'RSEOF'
#[derive(Debug, thiserror::Error)]
pub enum InclusiveError {
    #[error("Accessibility profile not found: {0}")]
    ProfileNotFound(uuid::Uuid),
    #[error("Compliance level insufficient")]
    ComplianceInsufficient,
}
RSEOF

echo "  ✓ haip/inclusive — Inclusive Design"

# -------------------------------------------------------
# Integration tests for Block 6
# -------------------------------------------------------
mkdir -p tests/integration
cat > tests/integration/block6.rs << 'RSEOF'
#[cfg(test)]
mod tests {
    use haip_claim::*;
    use haip_eta::*;
    use haip_dashboard::*;
    use haip_inclusive::*;

    #[tokio::test]
    async fn test_claim_autonomous() {
        let engine = engine::ClaimEngine::new(engine::ClaimConfig::default());
        let user = uuid::Uuid::new_v4();
        let action = types::CognitiveAction {
            id: uuid::Uuid::new_v4(),
            agent_id: vaos_core::types::AgentId::new(),
            description: "Balance inquiry".into(),
            cognitive_cost: types::CognitiveCost::Passive,
            risk_severity: 5,
            defaults: vec![],
        };
        let pres = engine.present(user, action).await.unwrap();
        assert!(matches!(pres, types::Presentation::Autonomous));
    }

    #[tokio::test]
    async fn test_eta_stress_detection() {
        let engine = engine::EtaEngine::new();
        let user = uuid::Uuid::new_v4();
        let cal = engine.adapt(user, "overdraft").await.unwrap();
        assert_eq!(cal.recommended_tone, types::InteractionTone::Supportive);
    }

    #[tokio::test]
    async fn test_dashboard_boundaries() {
        let engine = engine::DashboardEngine::new();
        let agent = vaos_core::types::AgentId::new();
        let boundary = types::AgentBoundary {
            agent_id: agent,
            spending_limit: rust_decimal::Decimal::new(500, 0),
            approval_threshold: rust_decimal::Decimal::new(1000, 0),
            allowed_operations: vec!["debit".into(), "balance_inquiry".into()],
        };
        engine.set_boundaries(agent, boundary).await.unwrap();
        let ok = engine.check_action(agent, "debit", Some(rust_decimal::Decimal::new(200, 0))).await.unwrap();
        assert!(ok);
    }

    #[tokio::test]
    async fn test_inclusive_registration() {
        let engine = engine::InclusiveEngine::new();
        let user = uuid::Uuid::new_v4();
        let profile = types::AccessibilityProfile {
            user_id: user,
            features: vec![types::AccessibilityFeature::ScreenReader],
            language: "en".into(),
            offline_preferred: false,
        };
        engine.register_profile(profile).await.unwrap();
        let ok = engine.check_interface(user, types::ComplianceLevel::AAA).await.unwrap();
        assert!(ok);
    }
}
RSEOF

echo "  ✓ Integration tests"

# -------------------------------------------------------
# Compilation check
# -------------------------------------------------------
echo ""
echo "============================================"
echo "  Verifying Block 6 compilation"
echo "============================================"
cargo check -p haip-claim -p haip-eta -p haip-dashboard -p haip-inclusive 2>&1
echo ""
echo "✅ MASTER BUILD 07 COMPLETE"
echo "   Next: cargo test --workspace"
echo "   Then: git commit -m 'feat: Block 6 Human-Agent Interaction Plane complete'"