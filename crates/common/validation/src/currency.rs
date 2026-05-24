use std::collections::HashSet;
use super::errors::ValidationError;

/// ISO 4217 currency code validator.
pub struct CurrencyValidator {
    active_codes: HashSet<String>,
    numeric_codes: HashSet<String>,
}

impl CurrencyValidator {
    pub fn new() -> Self {
        let mut validator = Self {
            active_codes: HashSet::new(),
            numeric_codes: HashSet::new(),
        };
        let currencies = vec![
            ("USD", "840"), ("EUR", "978"), ("GBP", "826"), ("JPY", "392"),
            ("CHF", "756"), ("CAD", "124"), ("AUD", "036"), ("CNY", "156"),
            ("INR", "356"), ("BRL", "986"), ("MXN", "484"), ("KRW", "410"),
            ("SGD", "702"), ("HKD", "344"), ("SEK", "752"), ("NOK", "578"),
            ("DKK", "208"), ("NZD", "554"), ("ZAR", "710"), ("RUB", "643"),
        ];
        for (alpha, numeric) in currencies {
            validator.active_codes.insert(alpha.to_string());
            validator.numeric_codes.insert(numeric.to_string());
        }
        validator
    }

    pub fn is_valid_alpha(&self, code: &str) -> bool {
        code.len() == 3 && self.active_codes.contains(code)
    }

    pub fn is_valid_numeric(&self, code: &str) -> bool {
        self.numeric_codes.contains(code)
    }

    pub fn validate(&self, code: &str) -> Result<(), ValidationError> {
        if !self.is_valid_alpha(code) {
            return Err(ValidationError::InvalidCurrency(code.to_string()));
        }
        Ok(())
    }
}
