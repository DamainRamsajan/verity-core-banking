use super::super::types::TransactionGraph;
use super::super::errors::FraudError;

pub struct ScafdsModel {
    loaded: bool,
}

impl ScafdsModel {
    pub fn new() -> Self { Self { loaded: false } }
    pub fn predict(&self, _graph: &TransactionGraph) -> Result<f64, FraudError> {
        // In production: tract-onnx ONNX inference with SCAFDS graph attention
        Ok(0.85)
    }
}
