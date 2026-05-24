pub mod engine;
pub mod merkle;
pub mod types;
pub mod errors;

pub use engine::MemLineageEngine;
pub use types::{MemoryEntry, QuarantineStatus};
pub use errors::LineageError;
