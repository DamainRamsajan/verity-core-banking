use super::errors::EdgeError;

pub struct ReservationPool {
    limit: rust_decimal::Decimal,
    consumed: rust_decimal::Decimal,
}

impl ReservationPool {
    pub fn new(limit: rust_decimal::Decimal) -> Self { Self { limit, consumed: rust_decimal::Decimal::ZERO } }

    pub fn consume(&mut self, amount: rust_decimal::Decimal) -> Result<(), EdgeError> {
        if self.consumed + amount > self.limit {
            return Err(EdgeError::OfflineLimitExceeded { limit: self.limit, attempted: amount, remaining: self.limit - self.consumed });
        }
        self.consumed += amount;
        Ok(())
    }
}
