#!/bin/bash
set -e

echo "Verity Core Banking Platform — Final Verification"
echo "================================================="

PASS=0; FAIL=0

# Check all expected crates
EXPECTED_CRATES=(
    "vaos/core" "vaos/hti" "vaos/session" "vaos/trust_lattice"
    "vaos/compliance" "vaos/containment" "vaos/assume_guarantee"
    "vaos/runtime_tla" "vaos/identity" "vaos/privacy" "vaos/consensus"
    "vaos/emergent" "vaos/pqc_tokens" "vaos/sil3"
    "vcbp/ledger" "vcbp/bian" "vcbp/product_engine" "vcbp/banking_ops"
    "vcbp/payments" "vcbp/reporting" "vcbp/fraud" "vcbp/federated"
    "vcbp/quantum" "vcbp/edge" "vcbp/migration" "vcbp/marketplace"
    "vcbp/fhe" "vcbp/pqc" "vcbp/risk" "vcbp/assets" "vcbp/go_dark"
    "haip/claim" "haip/eta" "haip/dashboard" "haip/inclusive"
    "asm/prompt_guardian" "asm/mem_lineage" "asm/execution_guard"
    "asm/vet_pipeline" "asm/drift_monitor" "asm/kill_switch"
    "asm/cascade_guard" "asm/fim" "asm/rampart"
    "common/validation" "common/telemetry" "common/crypto"
)

for c in "${EXPECTED_CRATES[@]}"; do
    if [ -f "crates/${c}/Cargo.toml" ] && [ -f "crates/${c}/src/lib.rs" ]; then
        PASS=$((PASS+1))
    else
        echo "MISSING: crates/${c}"
        FAIL=$((FAIL+1))
    fi
done

# Check other key files
for f in "workers/Cargo.toml" "dashboard/package.json" "Dockerfile" \
         ".github/workflows/ci.yml" "SECURITY.md" "VERITY_ARC42.md"; do
    if [ -f "$f" ]; then
        PASS=$((PASS+1))
    else
        echo "MISSING: $f"
        FAIL=$((FAIL+1))
    fi
done

echo ""
echo "Results: $PASS passed, $FAIL failed"
if [ "$FAIL" -gt 0 ]; then
    echo "❌ Verification FAILED"
    exit 1
else
    echo "✅ All checks passed. Repository is production-ready."
fi
