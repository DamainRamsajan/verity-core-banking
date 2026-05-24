# Contributing to Verity

Thank you for your interest in contributing!

## Getting Started
1. Fork the repo
2. Open in GitHub Codespaces (fully configured)
3. Run `make build` and `make test`

## Development Workflow
- Create a feature branch from `main`
- Write tests for all new functionality
- Run `make lint` before committing
- Submit a PR with a clear description

## Code Standards
- Rust: follow `rustfmt` and `clippy` defaults, all `#![forbid(unsafe_code)]`
- TypeScript: strict mode, ESLint recommended rules
- All public interfaces must have documented pre/post conditions

## Commit Convention
Follow [Conventional Commits](https://www.conventionalcommits.org/):
`feat:`, `fix:`, `docs:`, `test:`, `ci:`, `refactor:`

## Architecture
All significant changes must reference the [ARC42 Blueprint](./VERITY_ARC42.md).

See [ARCHITECTURE.md](./ARCHITECTURE.md) for the implementation map.
