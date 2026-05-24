#[derive(Debug, thiserror::Error)]
pub enum ClaimError {
    #[error("Cognitive budget exceeded: {remaining} remaining, {needed} needed")]
    CognitiveBudgetExceeded { remaining: u32, needed: u32 },
}
