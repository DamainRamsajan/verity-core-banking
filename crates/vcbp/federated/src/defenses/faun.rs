pub struct Faun;

impl Faun {
    pub fn new() -> Self { Self }
    pub fn unlearn(&self, _model: &[f64], _poisoned_indices: &[usize]) -> Vec<f64> { vec![] }
}
