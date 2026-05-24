#[derive(Debug, thiserror::Error)]
pub enum MarketplaceError {
    #[error("Insufficient stake: required {required}, provided {provided}")]
    InsufficientStake { required: rust_decimal::Decimal, provided: rust_decimal::Decimal },

    #[error("Listing not found: {0}")]
    ListingNotFound(uuid::Uuid),

    #[error("Agent not staked: {0:?}")]
    AgentNotStaked(vaos_core::types::AgentId),

    #[error("Challenge not found: {0}")]
    ChallengeNotFound(uuid::Uuid),

    #[error("Escrow not found: {0}")]
    EscrowNotFound(uuid::Uuid),
}
