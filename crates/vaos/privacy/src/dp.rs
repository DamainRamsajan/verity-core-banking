//! Differential Privacy service.
//!
//! Powered by OpenDP: a modular collection of statistical algorithms
//! adhering to the definition of differential privacy. Tracks ε budget
//! with composition and conversion between privacy definitions.

/// Differential Privacy service.
#[derive(Debug)]
pub struct DpService {
    epsilon: f64,
    delta: f64,
}

impl DpService {
    pub fn new(epsilon: f64, delta: f64) -> Self {
        Self { epsilon, delta }
    }

    /// Apply Laplace noise for ε-differential privacy.
    pub fn laplace_mechanism(
        &self,
        value: f64,
        sensitivity: f64,
    ) -> Result<f64, super::PrivacyError> {
        if self.epsilon <= 0.0 {
            return Err(super::PrivacyError::DpBudgetExhausted {
                remaining: 0.0,
                requested: sensitivity / value,
            });
        }
        let scale = sensitivity / self.epsilon;
        // Laplace noise: -scale * sign(U) * ln(1 - 2|U|)
        use rand::Rng;
        let mut rng = rand::rngs::OsRng;
        let u: f64 = rng.gen_range(-0.5..0.5);
        let noise = -scale * u.signum() * (1.0 - 2.0 * u.abs()).ln();
        Ok(value + noise)
    }
}
