use super::errors::EdgeError;

/// Reservation pool — pre‑reserved liquidity for offline spending.
///
/// Implements the Crunchfish Governed Offline Payments pattern:
/// - Risk is borne by the issuer of the offline wallet, not the payee
/// - Offline spending cannot exceed the reservation
/// - On reconnection, consumed reservation is reconciled
pub struct ReservationPool {
    limit: rust_decimal::Decimal,
    consumed: rust_decimal::Decimal,
}

impl ReservationPool {
    pub fn new(limit: rust_decimal::Decimal) -> Self {
        Self { limit, consumed: rust_decimal::Decimal::ZERO }
    }

    /// Consume from the reservation for an offline transaction.
    pub fn consume(&mut self, amount: rust_decimal::Decimal) -> Result<(), EdgeError> {
        if self.consumed + amount > self.limit {
            return Err(EdgeError::OfflineLimitExceeded {
                limit: self.limit,
                attempted: amount,
                remaining: self.limit - self.consumed,
            });
        }
        self.consumed += amount;
        Ok(())
    }

    /// Replenish the reservation on sync.
    pub fn replenish(&mut self) {
        self.consumed = rust_decimal::Decimal::ZERO;
    }
}
