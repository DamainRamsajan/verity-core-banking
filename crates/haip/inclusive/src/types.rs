use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User accessibility profile (self‑declared or auto‑detected).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityProfile {
    pub user_id: Uuid,
    pub features: Vec<AccessibilityFeature>,
    pub language: String,
    pub offline_preferred: bool,
}

/// Accessibility features requested.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessibilityFeature {
    LargeText,
    HighContrast,
    ScreenReader,
    VoiceInput,
    SimplifiedUI,
    PlainLanguage,
    ReducedMotion,
    KeyboardOnly,
    SwitchControl,
    OfflineMode,
}

/// Compliance level achieved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceLevel {
    A,
    AA,
    AAA,
    GabiEnhanced,
}
