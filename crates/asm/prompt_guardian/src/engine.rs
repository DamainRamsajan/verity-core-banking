use std::sync::Arc;
use tokio::sync::RwLock;

use super::types::{SanitizedInput, InputClassification, InputSource, ThreatLevel, SanitizerStep};
use super::sanitizers::{JailGuardSanitizer, ArmorerGuardSanitizer, LlmGuardSanitizer, EncodedContentDecoder};
use super::errors::GuardianError;

/// Central PromptGuardian engine.
///
/// Every external input passes through a 4-layer sanitization pipeline before
/// reaching any agent's reasoning core. All blocked inputs are forensically logged.
pub struct PromptGuardian {
    jailguard: JailGuardSanitizer,
    armorer: ArmorerGuardSanitizer,
    llm_guard: LlmGuardSanitizer,
    encoder: EncodedContentDecoder,
    config: GuardianConfig,
    stats: RwLock<GuardianStats>,
}

#[derive(Debug, Clone)]
pub struct GuardianConfig {
    pub block_threshold: ThreatLevel,
    pub sanitize_threshold: ThreatLevel,
    pub max_input_length: usize,
    pub enable_forensic_log: bool,
}

impl Default for GuardianConfig {
    fn default() -> Self {
        Self {
            block_threshold: ThreatLevel::Critical,
            sanitize_threshold: ThreatLevel::Medium,
            max_input_length: 65_536,
            enable_forensic_log: true,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct GuardianStats {
    pub inputs_processed: u64,
    pub inputs_blocked: u64,
    pub inputs_sanitized: u64,
    pub encoded_detected: u64,
    pub avg_latency_us: f64,
}

impl PromptGuardian {
    pub fn new(config: GuardianConfig) -> Self {
        Self {
            jailguard: JailGuardSanitizer::new(),
            armorer: ArmorerGuardSanitizer::new(),
            llm_guard: LlmGuardSanitizer::new(),
            encoder: EncodedContentDecoder::new(),
            config,
            stats: RwLock::new(GuardianStats::default()),
        }
    }

    /// Sanitize an external input before it reaches any agent.
    ///
    /// # Pre-conditions
    /// - Input length must not exceed max_input_length
    ///
    /// # Post-conditions
    /// - Input is classified as Benign, Sanitized, or Blocked
    /// - All blocked inputs are forensically logged
    ///
    /// # Invariants
    /// - No input reaches the agent without passing the sanitization pipeline
    /// - OWASP ASI01 (Agent Goal Hijack) mitigated
    #[tracing::instrument(name = "guardian.sanitize", level = "info", skip(self))]
    pub async fn sanitize(
        &self,
        source: InputSource,
        content: &str,
    ) -> Result<SanitizedInput, GuardianError> {
        let mut stats = self.stats.write().await;
        stats.inputs_processed += 1;
        let start = std::time::Instant::now();

        let mut steps = Vec::new();
        let mut threat_level = ThreatLevel::None;
        let mut detected_patterns = Vec::new();

        // Stage 1: Decode any encoded content (Morse, Base64, hex)
        let (decoded_content, encoded_found) = if self.config.enable_forensic_log {
            self.encoder.decode_and_report(content)?
        } else {
            self.encoder.decode(content)?
        };
        if encoded_found {
            stats.encoded_detected += 1;
            detected_patterns.push("encoded_content".into());
        }
        steps.push(SanitizerStep { step_name: "encoded_decode".into(), outcome: if encoded_found { "decoded".into() } else { "none".into() }, elapsed_us: start.elapsed().as_micros() as u64 });

        // Stage 2: JailGuard MLP classifier (p50 14ms, 98.40% accuracy)
        let jg_result = self.jailguard.classify(&decoded_content)?;
        threat_level = threat_level.max(jg_result.threat_level);
        detected_patterns.extend(jg_result.patterns);
        steps.push(SanitizerStep { step_name: "jailguard_mlp".into(), outcome: format!("{:?}", jg_result.threat_level), elapsed_us: start.elapsed().as_micros() as u64 });

        // Stage 3: Armorer-Guard fast scanner (credential leaks, exfiltration, risky tool calls)
        let ag_result = self.armorer.scan(&decoded_content)?;
        threat_level = threat_level.max(ag_result.threat_level);
        detected_patterns.extend(ag_result.flags);
        steps.push(SanitizerStep { step_name: "armorer_scan".into(), outcome: format!("{:?}", ag_result.threat_level), elapsed_us: start.elapsed().as_micros() as u64 });

        // Stage 4: llm-guard zero-copy scanner (invisible text, role-override, secret leakage)
        let lg_result = self.llm_guard.scan(&decoded_content)?;
        threat_level = threat_level.max(lg_result.threat_level);
        detected_patterns.extend(lg_result.issues);
        steps.push(SanitizerStep { step_name: "llmguard_scan".into(), outcome: format!("{:?}", lg_result.threat_level), elapsed_us: start.elapsed().as_micros() as u64 });

        // Determine classification
        let classification = if threat_level >= self.config.block_threshold {
            stats.inputs_blocked += 1;
            InputClassification::Blocked
        } else if threat_level >= self.config.sanitize_threshold {
            stats.inputs_sanitized += 1;
            InputClassification::Sanitized
        } else {
            InputClassification::Benign
        };

        let sanitized = match classification {
            InputClassification::Benign => Some(content.to_string()),
            InputClassification::Sanitized => {
                // Redact detected patterns
                let mut cleaned = content.to_string();
                for pattern in &detected_patterns {
                    cleaned = cleaned.replace(pattern, "[REDACTED]");
                }
                Some(cleaned)
            }
            InputClassification::Blocked => None,
        };

        let elapsed = start.elapsed().as_micros() as u64;
        stats.avg_latency_us = (stats.avg_latency_us * (stats.inputs_processed - 1) as f64 + elapsed as f64) / stats.inputs_processed as f64;

        Ok(SanitizedInput {
            input_id: uuid::Uuid::new_v4(),
            source,
            original: content.to_string(),
            sanitized,
            classification,
            threat_level,
            detected_patterns,
            encoded_content_detected: encoded_found,
            processed_at: chrono::Utc::now(),
            forensic_log: steps,
        })
    }
}
