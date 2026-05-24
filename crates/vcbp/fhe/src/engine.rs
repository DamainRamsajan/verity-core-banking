use std::sync::Arc;
use tokio::sync::RwLock;

use super::types::{FheBackend, FheCiphertext, FhePlaintext, FheScheme, FheBenchmark};
use super::backends::{SoftwareBackend, HeraclesBackend, GpuBackend};
use super::errors::FheError;

/// Central FHE engine — routes operations to the optimal backend.
///
/// Implements the strategy pattern: a unified interface over software,
/// GPU, and ASIC backends, with automatic detection and fallback.
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
        tracing::info!(?detected, "FHE backend detected");

        Self {
            backend: RwLock::new(detected),
            software: SoftwareBackend::new(),
            heracles: if detected == FheBackend::IntelHeracles { Some(HeraclesBackend::new()) } else { None },
            gpu: if detected == FheBackend::Gpu { Some(GpuBackend::new()) } else { None },
            config,
            stats: RwLock::new(FheStats::default()),
        }
    }

    /// Auto-detect the best available FHE backend at runtime.
    fn detect_best_backend() -> FheBackend {
        // 1. Check for Intel Heracles ASIC via PCIe device ID
        if std::path::Path::new("/dev/heracles0").exists() {
            return FheBackend::IntelHeracles;
        }
        // 2. Check for CUDA GPU
        if std::env::var("CUDA_VISIBLE_DEVICES").is_ok()
            || std::path::Path::new("/dev/nvidia0").exists() {
            return FheBackend::Gpu;
        }
        // 3. Fallback to pure-Rust TFHE-rs
        FheBackend::Software
    }

    /// Encrypt a plaintext balance for confidential ledger operations.
    #[tracing::instrument(name = "fhe.encrypt", level = "info", skip(self))]
    pub async fn encrypt_balance(
        &self,
        amount: rust_decimal::Decimal,
    ) -> Result<FheCiphertext, FheError> {
        let mut stats = self.stats.write().await;
        stats.encryptions += 1;

        let plaintext = FhePlaintext {
            value_type: super::types::FheValueType::Decimal { precision: 28, scale: 8 },
            data: amount.to_string().into_bytes(),
        };

        let backend = *self.backend.read().await;
        let ct = match backend {
            FheBackend::Software => self.software.encrypt(&plaintext, self.config.preferred_scheme)?,
            FheBackend::IntelHeracles => self.heracles.as_ref()
                .ok_or(FheError::BackendNotAvailable(FheBackend::IntelHeracles))?
                .encrypt(&plaintext, self.config.preferred_scheme)?,
            FheBackend::Gpu => self.gpu.as_ref()
                .ok_or(FheError::BackendNotAvailable(FheBackend::Gpu))?
                .encrypt(&plaintext, self.config.preferred_scheme)?,
            FheBackend::Auto => unreachable!(),
        };

        Ok(ct)
    }

    /// Homomorphically add two encrypted balances.
    #[tracing::instrument(name = "fhe.add", level = "debug", skip(self))]
    pub async fn add_encrypted(
        &self,
        a: &FheCiphertext,
        b: &FheCiphertext,
    ) -> Result<FheCiphertext, FheError> {
        let mut stats = self.stats.write().await;
        stats.homomorphic_adds += 1;

        if a.scheme != b.scheme {
            return Err(FheError::SchemeMismatch { a: a.scheme, b: b.scheme });
        }

        let backend = *self.backend.read().await;
        match backend {
            FheBackend::Software => self.software.add(a, b),
            FheBackend::IntelHeracles => self.heracles.as_ref()
                .ok_or(FheError::BackendNotAvailable(FheBackend::IntelHeracles))?
                .add(a, b),
            FheBackend::Gpu => self.gpu.as_ref()
                .ok_or(FheError::BackendNotAvailable(FheBackend::Gpu))?
                .add(a, b),
            FheBackend::Auto => unreachable!(),
        }
    }

    /// Homomorphically multiply (e.g., interest calculation on encrypted balance).
    #[tracing::instrument(name = "fhe.mul", level = "debug", skip(self))]
    pub async fn mul_encrypted(
        &self,
        a: &FheCiphertext,
        scalar: f64,
    ) -> Result<FheCiphertext, FheError> {
        let mut stats = self.stats.write().await;
        stats.homomorphic_muls += 1;

        let backend = *self.backend.read().await;
        match backend {
            FheBackend::Software => self.software.mul_scalar(a, scalar),
            FheBackend::IntelHeracles => self.heracles.as_ref()
                .ok_or(FheError::BackendNotAvailable(FheBackend::IntelHeracles))?
                .mul_scalar(a, scalar),
            FheBackend::Gpu => self.gpu.as_ref()
                .ok_or(FheError::BackendNotAvailable(FheBackend::Gpu))?
                .mul_scalar(a, scalar),
            FheBackend::Auto => unreachable!(),
        }
    }

    /// Run a benchmark comparing all available backends.
    pub async fn benchmark(&self) -> Result<Vec<FheBenchmark>, FheError> {
        let mut results = Vec::new();

        // Software benchmark
        let sw_result = self.software.benchmark_add(self.config.preferred_scheme)?;
        results.push(sw_result);

        // Heracles benchmark (if available)
        if let Some(heracles) = &self.heracles {
            let hw_result = heracles.benchmark_add(self.config.preferred_scheme)?;
            results.push(hw_result);
        }

        // GPU benchmark (if available)
        if let Some(gpu) = &self.gpu {
            let gpu_result = gpu.benchmark_add(self.config.preferred_scheme)?;
            results.push(gpu_result);
        }

        Ok(results)
    }
}
