#[derive(Debug, thiserror::Error)]
pub enum MarketplaceError {
    #[error("Insufficient stake: required {required}, provided {provided}")]
    InsufficientStake { required: rust_decimal::Decimal, provided: rust_decimal::Decimal },
}
