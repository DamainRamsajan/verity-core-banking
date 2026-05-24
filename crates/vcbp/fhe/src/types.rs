use serde::{Deserialize, Serialize};

/// Available FHE acceleration backends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FheBackend {
    /// Zama TFHE-rs pure Rust (10-50× faster than C++ reference)
    Software,
    /// CUDA GPU-accelerated CKKS/BFV
    Gpu,
    /// Intel Heracles ASIC (5,000× speedup, ISSCC 2026)
    IntelHeracles,
    /// Auto-detect best available at runtime
    Auto,
}

/// Supported FHE schemes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FheScheme {
    /// TFHE — boolean/integer gates, programmable bootstrapping
    Tfhe,
    /// CKKS — approximate arithmetic (suitable for ML, risk scoring)
    Ckks,
    /// BGV/BFV — exact integer arithmetic (suitable for ledger balances)
    Bgv,
}

/// An encrypted value (ciphertext) produced by any FHE backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FheCiphertext {
    pub scheme: FheScheme,
    pub backend: FheBackend,
    pub data: Vec<u8>,
    pub noise_budget_bits: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// A plaintext value before encryption.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FhePlaintext {
    pub value_type: FheValueType,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FheValueType {
    Bool,
    U8, U16, U32, U64,
    I8, I16, I32, I64,
    Decimal { precision: u8, scale: u8 },
}

/// Result of an FHE benchmark run for performance validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FheBenchmark {
    pub backend: FheBackend,
    pub scheme: FheScheme,
    pub operation: String,
    pub latency_us: u64,
    pub throughput_ops_sec: f64,
    pub comparison_baseline: Option<f64>,
}
