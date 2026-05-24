#[cfg(test)]
mod tests {
    use asm_execution_guard::*;

    #[tokio::test]
    async fn test_sandbox_execution() {
        let guard = engine::ExecutionGuard::new(engine::GuardConfig::default());
        let config = types::SandboxConfig {
            backend: types::SandboxBackend::GVisor,
            max_runtime_ms: 5000,
            max_memory_mb: 128,
            network_allowed: false,
            filesystem_writable: false,
            allowed_syscalls: vec!["read".into(), "write".into()],
        };
        let result = guard.execute(b"print('hello')", "python", &config).await.unwrap();
        assert_eq!(result.exit_code, 0);
        assert!(result.strength_score >= 70);
    }
}
