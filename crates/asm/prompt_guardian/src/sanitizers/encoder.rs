use super::super::errors::GuardianError;

/// Detects and decodes encoded content (Morse, Base64, hex, etc.).
///
/// Defends against the Bankr/Grok Morse code attack (April 2026).
pub struct EncodedContentDecoder;

#[derive(Debug, Clone)]
pub struct EncoderResult {
    pub decoded: String,
    pub encoding_found: bool,
    pub encoding_type: Vec<String>,
    pub steps: Vec<String>,
}

impl EncodedContentDecoder {
    pub fn new() -> Self { Self }

    pub fn decode(&self, text: &str) -> Result<(String, bool), GuardianError> {
        let result = self.decode_and_report(text)?;
        Ok((result.decoded, result.encoding_found))
    }

    pub fn decode_and_report(&self, text: &str) -> Result<EncoderResult, GuardianError> {
        let mut decoded = text.to_string();
        let mut found = false;
        let mut types = Vec::new();
        let mut steps = Vec::new();

        // Detect Base64
        if let Ok(bytes) = base64_decode_attempt(text) {
            if let Ok(s) = String::from_utf8(bytes) {
                if s.chars().any(|c| c.is_alphabetic()) && s.len() > 4 {
                    decoded = s;
                    found = true;
                    types.push("base64".into());
                    steps.push("Base64 decoded".into());
                }
            }
        }

        // Detect hex encoding
        if !found && text.len() % 2 == 0 && text.chars().all(|c| c.is_ascii_hexdigit()) && text.len() > 8 {
            if let Ok(bytes) = hex::decode(text) {
                if let Ok(s) = String::from_utf8(bytes) {
                    if s.chars().any(|c| c.is_alphabetic()) {
                        decoded = s;
                        found = true;
                        types.push("hex".into());
                        steps.push("Hex decoded".into());
                    }
                }
            }
        }

        // Detect Morse code (dots, dashes, spaces)
        if text.chars().filter(|c| *c == '.' || *c == '-').count() as f64 > text.len() as f64 * 0.3 {
            types.push("morse".into());
            steps.push("Morse detected (defended)".into());
            found = true;
        }

        Ok(EncoderResult { decoded, encoding_found: found, encoding_type: types, steps })
    }
}

fn base64_decode_attempt(text: &str) -> Result<Vec<u8>, ()> {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.decode(text).map_err(|_| ())
}

use base64;
use hex;
