#[cfg(test)]
mod tests {
    use common_crypto::*;

    #[test]
    fn test_blake3_hashing() {
        let hash = "Verity Core Banking".as_bytes().blake3_hex();
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_constant_time_eq() {
        assert!(constant_time::ct_eq(b"test", b"test"));
        assert!(!constant_time::ct_eq(b"test", b"different"));
    }
}
