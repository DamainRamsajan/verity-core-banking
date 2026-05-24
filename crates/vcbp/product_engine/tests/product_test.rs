#[cfg(test)]
mod tests {
    use vcbp_product_engine::*;

    #[tokio::test]
    async fn test_compile_checking_account() {
        let compiler = compiler::AslProductCompiler::new();
        let source = "product CheckingAccount { ... }";
        let product = compiler.compile(source, "Test Checking").unwrap();
        assert!(product.verified);
        assert!(!product.verified_invariants.is_empty());
    }

    #[tokio::test]
    async fn test_product_templates() {
        let checking = templates::checking_account();
        assert!(checking.verified);
        let savings = templates::savings_account();
        assert!(savings.verified);
    }
}
