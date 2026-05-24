use tokio::sync::RwLock;

use super::types::{SanitizedInput, InputSource, InputClassification, ThreatLevel, SanitizerStep};
use super::sanitizers::{InjectionDetector, EncodedContentDecoder};
use super::errors::GuardianError;

pub struct PromptGuardian {
    injection_detector: InjectionDetector,
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
            max_input_length: 65536,
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
            injection_detector: InjectionDetector::new(),
            encoder: EncodedContentDecoder::new(),
            config,
            stats: RwLock::new(GuardianStats::default()),
        }
    }

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

        // Stage 1: Decode any encoded content
        let (decoded, encoded_found) = self.encoder.decode(content)?;
        if encoded_found {
            stats.encoded_detected += 1;
            detected_patterns.push("encoded_content".into());
        }
        steps.push(SanitizerStep {
            step_name: "encoded_decode".into(),
            outcome: if encoded_found { "decoded".into() } else { "none".into() },
            elapsed_us: start.elapsed().as_micros() as u64,
        });

        // Stage 2: Injection detection
        let det_result = self.injection_detector.detect(&decoded)?;
        threat_level = threat_level.max(det_result.threat_level);
        detected_patterns.extend(det_result.patterns);
        steps.push(SanitizerStep {
            step_name: "injection_detector".into(),
            outcome: format!("{:?}", det_result.threat_level),
            elapsed_us: start.elapsed().as_micros() as u64,
        });

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
                let mut cleaned = content.to_string();
                for pattern in &detected_patterns {
                    cleaned = cleaned.replace(pattern, "[REDACTED]");
                }
                Some(cleaned)
            }
            InputClassification::Blocked => None,
        };

        let elapsed = start.elapsed().as_micros() as u64;
        stats.avg_latency_us = (stats.avg_latency_us * (stats.inputs_processed - 1) as f64 + elapsed as f64)
            / stats.inputs_processed as f64;

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
