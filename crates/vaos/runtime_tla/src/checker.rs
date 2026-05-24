use tla_checker::TlaSpec;
use super::errors::TlaError;

pub struct RuntimeTlaChecker {
    spec: TlaSpec,
}

impl RuntimeTlaChecker {
    pub fn new(tla_spec: &str) -> Result<Self, TlaError> {
        let spec = TlaSpec::parse(tla_spec).map_err(|e| TlaError::SpecParseError(e.to_string()))?;
        Ok(Self { spec })
    }

    pub fn sample(&self, json_trace: &str) -> Result<(), TlaError> {
        self.spec.check(json_trace).map_err(|e| TlaError::InvariantViolation(e.to_string()))
    }
}
