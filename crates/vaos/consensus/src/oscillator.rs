//! Kuramoto oscillator network — models the neuroscientific binding problem.
//!
//! Source: ORCHID protocol (arXiv:2605.09782)

/// Kuramoto oscillator network for distributed phase synchronization.
#[derive(Debug)]
pub struct KuramotoOscillator {
    node_count: usize,
    phases: Vec<f64>,
    natural_frequencies: Vec<f64>,
    coupling_strength: f64,
    iteration: u64,
}

impl KuramotoOscillator {
    pub fn new(node_count: usize) -> Self {
        use rand::Rng;
        let mut rng = rand::rngs::OsRng;

        Self {
            node_count,
            phases: (0..node_count).map(|_| rng.gen_range(0.0..std::f64::consts::TAU)).collect(),
            natural_frequencies: (0..node_count)
                .map(|_| rng.gen_range(-1.0..1.0))
                .collect(),
            coupling_strength: 2.0,
            iteration: 0,
        }
    }

    /// Evolve the oscillator network one timestep.
    ///
    /// Returns the order parameter r(t) ∈ [0, 1].
    pub fn evolve(&mut self) -> Result<f64, super::ConsensusError> {
        let n = self.node_count as f64;
        let k = self.coupling_strength;
        let dt = 0.01;

        // Compute mean phase
        let mut sum_sin = 0.0;
        let mut sum_cos = 0.0;
        for &phase in &self.phases {
            sum_sin += phase.sin();
            sum_cos += phase.cos();
        }
        let r = (sum_sin.powi(2) + sum_cos.powi(2)).sqrt() / n;

        // Update phases via Kuramoto ODE
        let mean_phase = sum_sin.atan2(sum_cos);
        for i in 0..self.node_count {
            self.phases[i] += dt * (
                self.natural_frequencies[i] + k * r * (mean_phase - self.phases[i]).sin()
            );
        }

        self.iteration += 1;
        Ok(r)
    }
}
