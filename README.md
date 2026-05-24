# Verity Core Banking Platform (VCBP) & Verity Agent OS (VAOS)

**Sovereign. Formally Verified. Agent‑Native. Quantum‑Ready.**

[![License: BSL 1.1](https://img.shields.io/badge/License-BSL%201.1-blue)](./LICENSE)
[![Rust 1.85](https://img.shields.io/badge/Rust-1.85-orange)](https://rust-lang.org)
[![CI](https://github.com/DamainRamsajan/verity-core-banking/actions/workflows/ci.yml/badge.svg)](https://github.com/DamainRamsajan/verity-core-banking/actions)

Verity is the world’s first **formally verified core banking system** that treats AI agents as first‑class participants. It replaces traditional mutable‑balance ledgers with a **Merkle‑proofed, TLA+‑verified double‑entry ledger**, enforces **capability‑based security** at compile time, and deploys as a **single Rust binary** on air‑gapped hardware with **concurrent hardware‑enforced trusted execution**.

> **Architecture Blueprint:** [VERITY_ARC42.md](https://github.com/DamainRamsajan/verity-core-banking/blob/main/VERITY_ARC42.md)

## 🚀 Quick Start

# Clone
git clone https://github.com/DamainRamsajan/verity-core-banking.git
cd verity-core-banking

# Install one‑click
curl -fsSL https://install.verity.io | bash
verity-install --config config/default.toml
🧱 Architecture Highlights
Layer Technology Safety Guarantee
Agent Runtime ASL language + seedvm Compile‑time safety (P1‑P8), capability tokens, session types
Hardware Trust Intel TDX + AMD SEV‑SNP (concurrent) Remote attestation, NMI‑based kill switch, CVE‑driven failover
Ledger Merkle Double‑Entry Ledger TLA+‑verified Conservation of Value (Σ entries = 0)
Products ASL‑compiled smart contracts Reg DD, Reg Z, Reg E enforced at compile time
Privacy FHE (Intel Heracles ASIC) + SMPC + DP Encrypted balance computation, privacy‑preserving federated learning
Quantum ORCHID consensus, ML‑DSA‑44, QAOA optimizer Post‑quantum security, quantum‑accelerated portfolio optimization
Compliance Lean 4 regulatory proofs, real‑time reporting Microsecond‑latency compliance, ZK‑proof audit packages
📦 Repository Map
text
verity-core-banking/
├── crates/
│ ├── vaos/ # Verity Agent OS (14 crates)
│ ├── vcbp/ # Verity Core Banking Platform (22 crates)
│ ├── haip/ # Human‑Agent Interaction Plane (4 crates)
│ ├── asm/ # Agent Security Mesh (9 crates)
│ └── common/ # Shared utilities (3 crates)
├── dashboard/ # React 19 Mission Control UI
├── workers/ # Cloudflare Workers (Rust WASM + TypeScript)
├── ml/ # Python ML/LLM training pipelines
├── supabase/ # Supabase Edge Functions & config
├── config/ # Environment‑specific TOML configs
├── tests/ # Integration, contract, fuzz, load tests
└── migrations/ # PostgreSQL migrations
🔗 Related Repositories
ASL Language & seedvm

VeriChain

📄 License
Business Source License 1.1. See LICENSE for details.
