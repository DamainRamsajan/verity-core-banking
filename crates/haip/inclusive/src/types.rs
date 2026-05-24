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
