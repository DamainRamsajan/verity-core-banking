use constant_time_eq::constant_time_eq;

/// Constant-time byte comparison for cryptographic operations.
pub fn ct_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() { return false; }
    constant_time_eq(a, b)
}
