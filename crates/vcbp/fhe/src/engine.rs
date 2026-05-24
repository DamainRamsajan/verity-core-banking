use tokio::sync::RwLock;

use super::types::{FheBackend, FheCiphertext, FhePlaintext, FheScheme, FheBenchmark};
use super::backends::{SoftwareBackend, HeraclesBackend, GpuBackend};
use super::errors::FheError;

pub struct FheEngine {
    backend: RwLock<FheBackend>,
    software: SoftwareBackend,
    heracles: Option<HeraclesBackend>,
    gpu: Option<GpuBackend>,
    config: FheConfig,
    stats: RwLock<FheStats>,
}

#[derive(Debug, Clone)]
pub struct FheConfig {
    pub preferred_scheme: FheScheme,
    pub max_noise_budget: u32,
    pub enable_benchmark: bool,
}

impl Default for FheConfig {
    fn default() -> Self {
        Self { preferred_scheme: FheScheme::Bgv, max_noise_budget: 128, enable_benchmark: true }
    }
}

#[derive(Debug, Default, Clone)]
pub struct FheStats {
    pub encryptions: u64,
    pub decryptions: u64,
    pub homomorphic_adds: u64,
    pub homomorphic_muls: u64,
    pub avg_latency_us: f64,
}

impl FheEngine {
    pub fn new(config: FheConfig) -> Self {
        let detected = Self::detect_best_backend();
        Self {
            backend: RwLock::new(detected),
            software: SoftwareBackend::new(),
            heracles: if detected == FheBackend::IntelHeracles { Some(HeraclesBackend::new()) } else { None },
            gpu: if detected == FheBackend::Gpu { Some(GpuBackend::new()) } else { None },
            config,
            stats: RwLock::new(FheStats::default()),
        }
    }

    fn detect_best_backend() -> FheBackend {
        if std::path::Path::new("/dev/heracles0").exists() { FheBackend::IntelHeracles }
        else if std::env::var("CUDA_VISIBLE_DEVICES").is_ok() { FheBackend::Gpu }
        else { FheBackend::Software }
    }

    pub async fn encrypt_balance(&self, amount: rust_decimal::Decimal) -> Result<FheCiphertext, FheError> {
        let mut stats = self.stats.write().await;
        stats.encryptions += 1;
        let plaintext = FhePlaintext {
            value_type: super::types::FheValueType::Decimal { precision: 28, scale: 8 },
            data: amount.to_string().into_bytes(),
        };
        let backend = *self.backend.read().await;
        match backend {
            FheBackend::Software => self.software.encrypt(&plaintext, self.config.preferred_scheme),
            FheBackend::IntelHeracles => self.heracles.as_ref().unwrap().encrypt(&plaintext, self.config.preferred_scheme),
            FheBackend::Gpu => self.gpu.as_ref().unwrap().encrypt(&plaintext, self.config.preferred_scheme),
            FheBackend::Auto => unreachable!(),
        }
    }

    pub async fn add_encrypted(&self, a: &FheCiphertext, b: &FheCiphertext) -> Result<FheCiphertext, FheError> {
        let mut stats = self.stats.write().await;
        stats.homomorphic_adds += 1;
        if a.scheme != b.scheme { return Err(FheError::SchemeMismatch { a: a.scheme, b: b.scheme }); }
        let backend = *self.backend.read().await;
        match backend {
            FheBackend::Software => self.software.add(a, b),
            FheBackend::IntelHeracles => self.heracles.as_ref().unwrap().add(a, b),
            FheBackend::Gpu => self.gpu.as_ref().unwrap().add(a, b),
            FheBackend::Auto => unreachable!(),
        }
    }

    pub async fn benchmark(&self) -> Result<Vec<FheBenchmark>, FheError> {
        let mut results = Vec::new();
        results.push(self.software.benchmark_add(self.config.preferred_scheme)?);
        if let Some(heracles) = &self.heracles {
            results.push(heracles.benchmark_add(self.config.preferred_scheme)?);
        }
        if let Some(gpu) = &self.gpu {
            results.push(gpu.benchmark_add(self.config.preferred_scheme)?);
        }
        Ok(results)
    }
}
