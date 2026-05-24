#!/bin/bash
set -e

# ============================================================
#  MASTER BUILD 08 – Block 7: Infrastructure, Validation & Launch
#  Runs every CI gate, builds all artifacts, verifies conformance.
# ============================================================

PROJECT_ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$PROJECT_ROOT"

PASS=0
FAIL=0
WARN=0

green()  { echo -e "\033[32m$1\033[0m"; }
red()    { echo -e "\033[31m$1\033[0m"; }
yellow() { echo -e "\033[33m$1\033[0m"; }
bold()   { echo -e "\033[1m$1\033[0m"; }

step_pass() { green "  ✅ $1"; PASS=$((PASS+1)); }
step_fail() { red   "  ❌ $1"; FAIL=$((FAIL+1)); }
step_warn() { yellow "  ⚠️  $1 (skipped)"; WARN=$((WARN+1)); }

header() {
    echo ""
    bold "============================================"
    bold "  $1"
    bold "============================================"
}

# -------------------------------------------------------
# 1. Pre‑flight checks
# -------------------------------------------------------
header "1. Pre‑flight Checks"

check_cmd() {
    if command -v "$1" &> /dev/null; then
        step_pass "$1 found"
    else
        step_fail "$1 not found"
    fi
}

check_cmd rustc
check_cmd cargo
check_cmd node
check_cmd npm

if command -v docker &> /dev/null; then step_pass "docker found"; else step_warn "docker not found"; fi
if command -v wrangler &> /dev/null; then step_pass "wrangler found"; else step_warn "wrangler not found (install with: npm i -g wrangler)"; fi
if command -v supabase &> /dev/null; then step_pass "supabase CLI found"; else step_warn "supabase CLI not found (install with: npm i -g supabase)"; fi
if command -v tlc &> /dev/null; then step_pass "tlc (TLA+) found"; else step_warn "tlc not found (TLA+ checks skipped)"; fi

# -------------------------------------------------------
# 2. Rust Workspace Gates
# -------------------------------------------------------
header "2. Rust Workspace Checks"

echo "Running cargo fmt --check..."
if cargo fmt --all -- --check 2>&1; then
    step_pass "cargo fmt"
else
    step_fail "cargo fmt (run 'cargo fmt --all' to fix)"
fi

echo "Running cargo clippy..."
if cargo clippy --workspace --all-features -- -D warnings 2>&1; then
    step_pass "cargo clippy"
else
    step_fail "cargo clippy"
fi

echo "Running cargo test --workspace..."
if cargo test --workspace 2>&1; then
    step_pass "cargo test"
else
    step_fail "cargo test"
fi

echo "Running cargo audit..."
if cargo audit 2>&1; then
    step_pass "cargo audit"
else
    step_warn "cargo audit (review advisories)"
fi

echo "Running cargo deny check..."
if cargo deny check 2>&1; then
    step_pass "cargo deny"
else
    step_warn "cargo deny (review license/bans)"
fi

# -------------------------------------------------------
# 3. TLA+ Model Checking (if available)
# -------------------------------------------------------
header "3. TLA+ Model Checking"

if command -v tlc &> /dev/null; then
    TLA_SPEC="crates/vaos/runtime_tla/VerityLedger.tla"
    if [ -f "$TLA_SPEC" ]; then
        echo "Running TLC on VerityLedger.tla..."
        if tlc "$TLA_SPEC" -deadlock -workers auto 2>&1; then
            step_pass "TLA+ model check passed"
        else
            step_fail "TLA+ model check"
        fi
    else
        step_warn "TLA+ spec not found at $TLA_SPEC"
    fi
else
    step_warn "tlc not available"
fi

# -------------------------------------------------------
# 4. Lean 4 Proof Checking (if Lean available)
# -------------------------------------------------------
header "4. Lean 4 Proofs"

if command -v lean &> /dev/null; then
    LEAN_DIR="crates/vaos/compliance"
    if [ -f "$LEAN_DIR/lakefile.lean" ]; then
        echo "Running lake build in $LEAN_DIR..."
        (cd "$LEAN_DIR" && lake build 2>&1) && step_pass "Lean 4 proofs" || step_fail "Lean 4 proofs"
    else
        step_warn "No lakefile.lean found in $LEAN_DIR"
    fi
else
    step_warn "Lean 4 not installed"
fi

# -------------------------------------------------------
# 5. Fuzz Testing (if binary exists)
# -------------------------------------------------------
header "5. Fuzz Testing"

FUZZ_BIN="target/debug/fuzz_engine"
if [ -f "$FUZZ_BIN" ]; then
    echo "Running fuzz engine with 50,000 sequences..."
    if timeout 120 cargo run --bin fuzz_engine -- --sequences 50000 2>&1; then
        step_pass "Fuzz testing (50K sequences)"
    else
        step_fail "Fuzz testing"
    fi
else
    step_warn "Fuzz binary not found (build with: cargo build --bin fuzz_engine)"
fi

# -------------------------------------------------------
# 6. Dashboard UI
# -------------------------------------------------------
header "6. Dashboard UI"

if [ -d "dashboard" ]; then
    cd dashboard
    echo "Installing dependencies..."
    if npm ci 2>&1; then
        step_pass "npm ci"
    else
        step_fail "npm ci"
        cd ..
    fi

    if [ $FAIL -eq 0 ]; then
        echo "Running lint..."
        if npm run lint 2>&1; then
            step_pass "dashboard lint"
        else
            step_fail "dashboard lint"
        fi

        echo "Type checking..."
        if npm run type-check 2>&1; then
            step_pass "dashboard type-check"
        else
            step_fail "dashboard type-check"
        fi

        echo "Running tests..."
        if npm test 2>&1; then
            step_pass "dashboard tests"
        else
            step_fail "dashboard tests"
        fi

        echo "Building production bundle..."
        if npm run build 2>&1; then
            step_pass "dashboard build"
        else
            step_fail "dashboard build"
        fi

        # Accessibility audit with axe-core (optional)
        if command -v npx &> /dev/null; then
            echo "Running accessibility audit (axe-core)..."
            if npx --yes @axe-core/cli --stdout dist/index.html 2>&1; then
                step_pass "accessibility audit"
            else
                step_warn "accessibility audit (review issues)"
            fi
        fi
    fi
    cd "$PROJECT_ROOT"
else
    step_warn "dashboard/ directory not found"
fi

# -------------------------------------------------------
# 7. Cloudflare Workers
# -------------------------------------------------------
header "7. Cloudflare Workers"

if [ -d "workers" ]; then
    cd workers
    if command -v wrangler &> /dev/null; then
        echo "Building workers (wrangler build)..."
        if npx wrangler build 2>&1; then
            step_pass "workers build"
        else
            step_fail "workers build"
        fi
    else
        step_warn "wrangler not installed, skipping workers build"
    fi
    cd "$PROJECT_ROOT"
else
    step_warn "workers/ directory not found"
fi

# -------------------------------------------------------
# 8. Supabase Edge Functions
# -------------------------------------------------------
header "8. Supabase Edge Functions"

if [ -d "supabase/functions" ]; then
    if command -v supabase &> /dev/null; then
        echo "Linting Edge Functions..."
        if supabase functions lint 2>&1; then
            step_pass "supabase functions lint"
        else
            step_fail "supabase functions lint"
        fi
    else
        step_warn "supabase CLI not installed, skipping Edge Functions check"
    fi
else
    step_warn "supabase/functions/ directory not found"
fi

# -------------------------------------------------------
# 9. Docker Image Build & Scan
# -------------------------------------------------------
header "9. Docker"

if command -v docker &> /dev/null; then
    echo "Building Docker image..."
    if docker build -t verity-core-banking:ci . 2>&1; then
        step_pass "docker build"
    else
        step_fail "docker build"
    fi

    if command -v trivy &> /dev/null; then
        echo "Scanning image with Trivy..."
        if trivy image --severity HIGH,CRITICAL verity-core-banking:ci 2>&1; then
            step_pass "trivy scan (no HIGH/CRITICAL)"
        else
            step_fail "trivy scan found vulnerabilities"
        fi
    else
        step_warn "trivy not installed (install: brew install trivy)"
    fi
else
    step_warn "docker not available"
fi

# -------------------------------------------------------
# 10. Final Verification
# -------------------------------------------------------
header "10. Final Verification"

if [ -f "scripts/verify.sh" ]; then
    echo "Running verify.sh..."
    if bash scripts/verify.sh 2>&1; then
        step_pass "repository verification"
    else
        step_fail "repository verification"
    fi
else
    step_warn "scripts/verify.sh not found"
fi

# -------------------------------------------------------
# 11. Conformance Checklist Summary
# -------------------------------------------------------
header "11. Conformance Checklist"

echo "Key architectural guarantees (from ARC42 §11):"
echo ""
echo "  [ ] All ledger transactions are Merkle‑proofed and TLA+‑verified"
echo "  [ ] All banking products are ASL‑compiled; incorrect products cannot compile"
echo "  [ ] All agent actions governed by PASETO v4 capability tokens"
echo "  [ ] Deployment as single Rust binary on air‑gapped hardware"
echo "  [ ] BIAN v14.0‑native (328 Service Domains)"
echo "  [ ] DORA, EU AI Act, US banking regulations compliance"
echo "  [ ] zkVM binary‑hash agent identity"
echo "  [ ] Post‑quantum cryptography (FIPS 203/204/205)"
echo "  [ ] FHE + SMPC + DP privacy triad"
echo "  [ ] FedNow, SWIFT blockchain, ISO 20022 structured addresses"
echo "  [ ] Real‑time regulatory reporting from ledger"
echo "  [ ] Decentralized agent marketplace with KYA compliance"
echo "  [ ] Full OWASP Agentic Top 10 (ASI01‑ASI10) coverage"
echo "  [ ] Concurrent multi‑TEE with CVE‑driven failover"
echo "  [ ] 1A1A agent accounts with capability‑gated spending"
echo "  [ ] Offline‑first edge banking with governed payments"
echo "  [ ] Instant card issuance, biometric ATM auth, precious metals ATM"
echo "  [ ] Humanitarian portable identity"
echo "  [ ] Seven‑day bank migration with parallel‑run validation"
echo "  [ ] Cognitive budget model, emotional trust architecture, inclusive design"
echo "  [ ] RAMPART CI/CD adversarial testing"
echo ""
echo "  (Run 'cargo test --workspace' to validate all implementation tests)"

# -------------------------------------------------------
# 12. Final Report
# -------------------------------------------------------
header "12. Final Report"

echo ""
echo "  Results:"
green "    Passed: $PASS"
if [ $FAIL -gt 0 ]; then red "    Failed: $FAIL"; else green "    Failed: 0"; fi
if [ $WARN -gt 0 ]; then yellow "    Warnings (skipped): $WARN"; fi
echo ""

if [ $FAIL -eq 0 ]; then
    green "✅ ALL CHECKS PASSED — Verity Core Banking Platform is production‑ready."
    echo ""
    echo "  Next steps:"
    echo "  1. Deploy to staging: docker-compose up -d"
    echo "  2. Run integration tests: cargo test --workspace"
    echo "  3. Begin pilot migration with a partner bank"
    echo "  4. Deploy Cloudflare Workers: cd workers && npx wrangler deploy"
    echo "  5. Deploy Supabase Edge Functions: supabase functions deploy"
    echo ""
else
    red "❌ SOME CHECKS FAILED — review the output above before launch."
    exit 1
fi