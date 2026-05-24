#[cfg(test)]
mod tests {
    use common_validation::*;

    #[test]
    fn test_currency_valid() {
        let v = currency::CurrencyValidator::new();
        assert!(v.is_valid_alpha("USD"));
        assert!(!v.is_valid_alpha("ZZZ"));
    }

    #[test]
    fn test_regulatory_interest_rate() {
        let v = regulatory::RegulatoryValidator::new();
        assert!(v.validate_interest_rate(rust_decimal::Decimal::new(25, 1)).is_ok());
        assert!(v.validate_interest_rate(rust_decimal::Decimal::new(-1, 0)).is_err());
    }
}
