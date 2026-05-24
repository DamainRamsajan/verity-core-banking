use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FheBackend {
    Software,
    Gpu,
    IntelHeracles,
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FheScheme { Tfhe, Ckks, Bgv }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FheCiphertext {
    pub scheme: FheScheme,
    pub backend: FheBackend,
    pub data: Vec<u8>,
    pub noise_budget_bits: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FheBenchmark {
    pub backend: FheBackend,
    pub scheme: FheScheme,
    pub operation: String,
    pub latency_us: u64,
    pub throughput_ops_sec: f64,
    pub comparison_baseline: Option<f64>,
}
