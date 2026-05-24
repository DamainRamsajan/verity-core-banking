use uuid::Uuid;
use super::errors::MarketplaceError;

/// On‑chain escrow for agent‑to‑agent payments.
///
/// Follows CHEESE Agent Marketplace model: requesters escrow funds,
/// providers complete work, funds are released on delivery acceptance.
pub struct EscrowEngine {
    active_escrows: std::sync::RwLock<Vec<EscrowContract>>,
}

#[derive(Debug, Clone)]
pub struct EscrowContract {
    pub escrow_id: Uuid,
    pub requester: vaos_core::types::AgentId,
    pub provider: vaos_core::types::AgentId,
    pub amount: rust_decimal::Decimal,
    pub status: EscrowStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscrowStatus {
    Funded,
    InProgress,
    Delivered,
    Accepted,
    Disputed,
    Released,
    Refunded,
}

impl EscrowEngine {
    pub fn new() -> Self { Self { active_escrows: std::sync::RwLock::new(Vec::new()) } }

    /// Create a new escrow contract.
    pub fn create_escrow(
        &self,
        requester: vaos_core::types::AgentId,
        provider: vaos_core::types::AgentId,
        amount: rust_decimal::Decimal,
    ) -> Result<EscrowContract, MarketplaceError> {
        let contract = EscrowContract {
            escrow_id: Uuid::new_v4(),
            requester,
            provider,
            amount,
            status: EscrowStatus::Funded,
        };
        self.active_escrows.write().unwrap().push(contract.clone());
        Ok(contract)
    }

    /// Release escrow to the provider.
    pub fn release(&self, escrow_id: Uuid) -> Result<(), MarketplaceError> {
        let mut escrows = self.active_escrows.write().unwrap();
        let escrow = escrows.iter_mut()
            .find(|e| e.escrow_id == escrow_id)
            .ok_or(MarketplaceError::EscrowNotFound(escrow_id))?;
        escrow.status = EscrowStatus::Released;
        Ok(())
    }
}
