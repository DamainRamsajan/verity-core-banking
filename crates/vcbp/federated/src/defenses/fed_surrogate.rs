pub struct FedSurrogate;

impl FedSurrogate {
    pub fn new() -> Self { Self }
    pub fn filter(&self, _update: &[f64]) -> bool { true }
}
