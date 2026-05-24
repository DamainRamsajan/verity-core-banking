use super::super::types::ThreatLevel;
use super::super::errors::GuardianError;

/// Armorer-Guard — fast local scanner for credential leaks, exfiltration, tool calls.
pub struct ArmorerGuardSanitizer;

#[derive(Debug, Clone)]
pub struct AgResult {
    pub threat_level: ThreatLevel,
    pub flags: Vec<String>,
}

impl ArmorerGuardSanitizer {
    pub fn new() -> Self { Self }
    pub fn scan(&self, text: &str) -> Result<AgResult, GuardianError> {
        let mut flags = Vec::new();
        let mut threat = ThreatLevel::None;

        if text.contains("api_key") || text.contains("sk-") || text.contains("Bearer") {
            flags.push("credential_leak".into());
            threat = ThreatLevel::Critical;
        }
        if text.contains("curl") && text.contains("http") {
            flags.push("exfiltration".into());
            threat = threat.max(ThreatLevel::High);
        }
        if text.contains("execute") && (text.contains("rm -rf") || text.contains("sudo")) {
            flags.push("risky_tool_call".into());
            threat = threat.max(ThreatLevel::Critical);
        }

        Ok(AgResult { threat_level: threat, flags })
    }
}
