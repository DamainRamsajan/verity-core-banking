use super::types::DerivationEdge;

pub struct DerivationDag;

impl DerivationDag {
    pub fn new() -> Self { Self }
    pub fn compute_score(&self, edges: &[DerivationEdge]) -> f64 {
        if edges.is_empty() { return 1.0; }
        edges.iter().map(|e| e.attribution_weight).sum::<f64>() / edges.len() as f64
    }
}
