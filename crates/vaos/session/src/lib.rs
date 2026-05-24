//! Verity Agent OS — Session Type Checker (McDermott-Yoshida ESOP 2026)
//!
//! Source: ARC42 v20.0 §3 VAOS
//! Full implementation delivered in subsequent batches.

pub mod core;

/// Run a self-check to verify the crate compiles and links.
#[cfg(test)]
mod tests {
    #[test]
    fn crate_compiles() {
        assert!(true, "session crate is linkable");
    }
}
