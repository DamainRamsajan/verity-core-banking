.PHONY: build test lint fuzz docker deploy clean tla-check lean-prove ui-dev ui-build workers-dev workers-deploy help

CARGO = cargo --color=always

build:
	$(CARGO) build --workspace --release

test:
	$(CARGO) test --workspace

lint:
	$(CARGO) fmt --all --check
	$(CARGO) clippy --workspace -- -D warnings

fuzz:
	$(CARGO) run --bin fuzz_engine -- --sequences 500000

docker:
	docker build -t verity-core-banking:latest .

deploy:
	$(CARGO) build --release
	scp target/release/verity prod-server:/usr/local/bin/

clean:
	$(CARGO) clean
	rm -rf node_modules dist

# Run TLA+ model checking
tla-check:
	cd crates/vaos/runtime_tla && tlc VerityLedger.tla

# Generate Lean 4 compliance proofs
lean-prove:
	cd crates/vaos/compliance && lean --run ComplianceProofs.lean

# Dashboard
ui-dev:
	cd dashboard && npm run dev

ui-build:
	cd dashboard && npm run build

# Workers
workers-dev:
	cd workers && npx wrangler dev

workers-deploy:
	cd workers && npx wrangler deploy

help:
	@echo "Usage:"
	@echo "  make build        Build the workspace"
	@echo "  make test         Run all tests"
	@echo "  make lint         Format and lint"
	@echo "  make fuzz         Run fuzz engine (500K sequences)"
	@echo "  make docker       Build Docker image"
	@echo "  make deploy       Deploy binary to production server"
	@echo "  make clean        Clean build artifacts"
	@echo "  make tla-check    Run TLA+ model checker"
	@echo "  make lean-prove   Generate Lean 4 proofs"
	@echo "  make ui-dev       Start dashboard dev server"
	@echo "  make ui-build     Build dashboard for production"
	@echo "  make workers-dev  Start Workers dev server"
	@echo "  make workers-deploy Deploy Cloudflare Workers"
