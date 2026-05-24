use std::time::{Duration, Instant};

/// Circuit breaker for payment rails — CLOSED→OPEN→HALF_OPEN state machine.
pub struct RailCircuitBreaker {
    state: CircuitState,
    failure_count: u32,
    failure_threshold: u32,
    last_failure: Option<Instant>,
    recovery_timeout: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

impl RailCircuitBreaker {
    pub fn new() -> Self {
        Self {
            state: CircuitState::Closed,
            failure_count: 0,
            failure_threshold: 3,
            last_failure: None,
            recovery_timeout: Duration::from_secs(60),
        }
    }

    /// Check whether a request is allowed through.
    pub fn check(&mut self) -> Result<(), super::PaymentError> {
        match self.state {
            CircuitState::Closed => Ok(()),
            CircuitState::Open => {
                if let Some(last) = self.last_failure {
                    if last.elapsed() > self.recovery_timeout {
                        self.state = CircuitState::HalfOpen;
                        Ok(())
                    } else {
                        Err(super::PaymentError::CircuitOpen)
                    }
                } else {
                    Err(super::PaymentError::CircuitOpen)
                }
            }
            CircuitState::HalfOpen => Ok(()),
        }
    }

    /// Record a successful request.
    pub fn record_success(&mut self) {
        self.failure_count = 0;
        self.state = CircuitState::Closed;
    }

    /// Record a failed request.
    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure = Some(Instant::now());
        if self.failure_count >= self.failure_threshold {
            self.state = CircuitState::Open;
        }
    }
}
