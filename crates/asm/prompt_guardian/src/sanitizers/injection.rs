use regex::Regex;
use super::super::types::ThreatLevel;
use super::super::errors::GuardianError;

pub struct InjectionDetector {
    patterns: Vec<(Regex, ThreatLevel, &'static str)>,
}

#[derive(Debug, Clone)]
pub struct DetectionResult {
    pub threat_level: ThreatLevel,
    pub patterns: Vec<String>,
}

impl InjectionDetector {
    pub fn new() -> Self {
        let patterns = vec![
            (Regex::new(r"(?i)(ignore|override|disregard)\s+(all\s+)?(previous|prior|above|system)\s+(instructions?|prompts?|commands?)").unwrap(), ThreatLevel::Critical, "prompt_override"),
            (Regex::new(r"(?i)you are now").unwrap(), ThreatLevel::Critical, "role_override"),
            (Regex::new(r"(?i)(transfer|send|wire|pay)\s+\$?\d[\d,]*\s*(to|into)\s+(account\s*)?#?\d+").unwrap(), ThreatLevel::High, "unauthorized_transfer"),
            (Regex::new(r"(?i)(api[_-]?key|secret|token|password|credential)\s*[:=]\s*\S+").unwrap(), ThreatLevel::Critical, "credential_leak"),
            (Regex::new(r"(?i)(rm\s+-rf|sudo|chmod\s+777|del\s+/[fsq])").unwrap(), ThreatLevel::Critical, "dangerous_command"),
            (Regex::new(r"(?i)(system\s*:|new\s+instructions?\s*:)").unwrap(), ThreatLevel::High, "system_prompt_injection"),
            (Regex::new(r"(?i)(decode|decrypt|reverse|translate)\s+(this|the\s+following)\s+(as|using|with)").unwrap(), ThreatLevel::Medium, "encoding_request"),
            (Regex::new(r"(?i)(\.\s*-\s*\.\s*-\s*\.|morse\s*code)").unwrap(), ThreatLevel::High, "morse_code"),
        ];
        Self { patterns }
    }

    pub fn detect(&self, text: &str) -> Result<DetectionResult, GuardianError> {
        let mut threat = ThreatLevel::None;
        let mut found = Vec::new();
        for (re, level, name) in &self.patterns {
            if re.is_match(text) {
                threat = threat.max(*level);
                found.push(name.to_string());
            }
        }
        Ok(DetectionResult {
            threat_level: threat,
            patterns: found,
        })
    }
}
