#[derive(Debug, thiserror::Error)]
pub enum RampartError { #[error("RAMPART suite failed: {0}/{1} tests passing")] SuiteFailed(u64, u64), #[error("MTTD target exceeded: {0}ms > {1}ms")] MttdExceeded(u64, u64) }
