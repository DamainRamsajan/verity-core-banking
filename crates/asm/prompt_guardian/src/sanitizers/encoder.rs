use super::super::errors::GuardianError;

pub struct EncodedContentDecoder;

impl EncodedContentDecoder {
    pub fn new() -> Self { Self }

    pub fn decode(&self, text: &str) -> Result<(String, bool), GuardianError> {
        let mut decoded = text.to_string();
        let mut found = false;

        // Base64 detection: try to decode if it looks like base64
        if text.len() % 4 == 0 && text.len() > 8 && text.chars().all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=') {
            if let Ok(bytes) = base64_decode(text) {
                if let Ok(s) = String::from_utf8(bytes) {
                    if s.chars().any(|c| c.is_alphabetic()) {
                        decoded = s;
                        found = true;
                    }
                }
            }
        }

        // Hex detection
        if !found && text.len() % 2 == 0 && text.chars().all(|c| c.is_ascii_hexdigit()) && text.len() > 8 {
            if let Ok(bytes) = hex::decode(text) {
                if let Ok(s) = String::from_utf8(bytes) {
                    if s.chars().any(|c| c.is_alphabetic()) {
                        decoded = s;
                        found = true;
                    }
                }
            }
        }

        // Morse detection (dots and dashes)
        if text.chars().filter(|c| *c == '.' || *c == '-').count() as f64 > text.len() as f64 * 0.3 {
            found = true;
        }

        Ok((decoded, found))
    }
}

fn base64_decode(input: &str) -> Result<Vec<u8>, ()> {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.decode(input).map_err(|_| ())
}
