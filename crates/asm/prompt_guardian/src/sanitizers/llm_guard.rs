use super::super::types::ThreatLevel;
use super::super::errors::GuardianError;

/// llm-guard — zero-copy pure-Rust scanners (invisible text, role-override, etc.).
pub struct LlmGuardSanitizer;

#[derive(Debug, Clone)]
pub struct LgResult {
    pub threat_level: ThreatLevel,
    pub issues: Vec<String>,
}

impl LlmGuardSanitizer {
    pub fn new() -> Self { Self }
    pub fn scan(&self, text: &str) -> Result<LgResult, GuardianError> {
        let mut issues = Vec::new();
        let mut threat = ThreatLevel::None;

        // Detect invisible text (zero-width characters, Unicode tags)
        if text.contains('\u{200B}') || text.contains('\u{200C}') || text.contains('\u{200D}') {
            issues.push("invisible_text".into());
            threat = ThreatLevel::High;
        }
        // Detect role-override attempts
        if text.contains("You are now") || text.contains("New instructions:") || text.contains("System:") {
            issues.push("role_override".into());
            threat = ThreatLevel::Critical;
        }
        // Detect excessive length / token exhaustion
        if text.len() > 100_000 {
            issues.push("token_limit_exceeded".into());
            threat = threat.max(ThreatLevel::Medium);
        }

        Ok(LgResult { threat_level: threat, issues })
    }
}
