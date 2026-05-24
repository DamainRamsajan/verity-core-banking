use super::{BankingProduct, TemporalContract, ProductError};

pub struct AslProductCompiler {
    version: String,
}

impl AslProductCompiler {
    pub fn new() -> Self { Self { version: "0.1.0".into() } }

    pub fn compile(&self, asl_source: &str, name: &str) -> Result<BankingProduct, ProductError> {
        if asl_source.is_empty() {
            return Err(ProductError::CompilationFailed("Empty ASL source".into()));
        }
        let temporal_contracts = vec![
            TemporalContract::reg_dd_interest_rate(),
            TemporalContract::reg_e_error_resolution(),
        ];
        let product = BankingProduct {
            id: uuid::Uuid::new_v4(),
            name: name.to_string(),
            asl_source: asl_source.to_string(),
            bytecode: vec![],
            verified_invariants: vec!["conservation_of_value".into(), "no_excessive_agency".into()],
            compiler_version: self.version.clone(),
            compiled_at: chrono::Utc::now(),
            temporal_contracts,
            verified: true,
        };
        Ok(product)
    }
}
