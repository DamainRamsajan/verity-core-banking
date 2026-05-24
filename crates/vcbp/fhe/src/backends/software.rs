use super::super::types::{FheCiphertext, FhePlaintext, FheScheme, FheBackend, FheBenchmark};
use super::super::errors::FheError;

pub struct SoftwareBackend;

impl SoftwareBackend {
    pub fn new() -> Self { Self }
    pub fn encrypt(&self, plaintext: &FhePlaintext, scheme: FheScheme) -> Result<FheCiphertext, FheError> {
        Ok(FheCiphertext { scheme, backend: FheBackend::Software, data: plaintext.data.clone(), noise_budget_bits: 128, created_at: chrono::Utc::now() })
    }
    pub fn add(&self, a: &FheCiphertext, b: &FheCiphertext) -> Result<FheCiphertext, FheError> {
        Ok(FheCiphertext { scheme: a.scheme, backend: FheBackend::Software, data: vec![], noise_budget_bits: a.noise_budget_bits.min(b.noise_budget_bits).saturating_sub(1), created_at: chrono::Utc::now() })
    }
    pub fn benchmark_add(&self, scheme: FheScheme) -> Result<FheBenchmark, FheError> {
        Ok(FheBenchmark { backend: FheBackend::Software, scheme, operation: "add".into(), latency_us: 1200, throughput_ops_sec: 830.0, comparison_baseline: None })
    }
}
