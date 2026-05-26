# Verity Agent Security Mesh – LASM Layer Mapping

This document maps every Verity Agent Security Mesh (ASM) component and
v23 breakthrough crate to the corresponding layer of the **LASM framework**
(LLM Agent Security Model, Chu et al., arXiv:2604.23338, May 2026).

| LASM Layer | Verity Component | Coverage |
|:---|:---|:---|
| **Foundation** | Capability Microkernel, Hardware Trust Interface (TEE, NMI) | Full |
| **Cognitive** | PromptGuardian (input sanitisation, injection detection) | Full |
| **Memory** | MemLineage (memory integrity, quarantine, Merkle log) | Full |
| **Tool Execution** | ExecutionGuard (sandbox, MCP validation, trajectory analysis) | Full |
| **Multi‑Agent Coordination** | Session Type Checker, CascadeGuard (circuit breakers) | Full |
| **Ecosystem** | VetPipeline (marketplace vetting, SCH detection) | Full |
| **Governance** | DriftMonitor, Kill Switch Protocol, Financial Invariants Monitor, Lean‑Agent Compliance Verifier, EHV JIT Compiler | Full |
| **Self‑Evolution** | vaos/evolution (SEVerA‑verified FGGM) | Full |
| **Evidence** | vaos/evidence (EVE‑Agent contribution‑measured learning audit) | Full |
| **Identity** | Non‑Human Identity Manager (1A1A, zkVM, KYA) | Full |
| **Privacy** | FHE Service, SMPC Service, DP Service | Full |
| **Post‑Quantum** | PQC Token Engine, ML‑DSA‑44 migration | Full |
| **Consensus** | ORCHID quantum‑augmented consensus | Full |

All seven LASM layers are covered by Verity’s defence‑in‑depth architecture.
The OWASP Agentic Top 10 (ASI01‑ASI10) is fully addressed by the ASM components.
