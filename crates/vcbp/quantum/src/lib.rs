//! # Verity Core Banking — Quantum Optimisation Accelerator
//!
//! Targets three core banking domains where quantum advantage is demonstrable:
//!
//! - **Portfolio Optimisation**: Two-step QAOA with JPMorgan Max‑k‑Cut
//!   formulation, surpassing classical SDP bounds at shallow depths
//! - **Stress Testing**: Quantum‑accelerated CECL/IFRS 9 expected loss and
//!   DFAST/CCAR scenario simulation
//! - **Derivative Pricing**: Hybrid quantum‑classical Monte Carlo acceleration
//!
//! ## Architecture
//! - **QAOA Solver**: ruqu-algorithms v2.0.5 provides production QAOA MaxCut
//!   with approximate quantum advantage
//! - **Hybrid Benchmark Framework**: invokes quantum backends only when
//!   demonstrable advantage exists; classical fallback via Gurobi/CPLEX
//! - **IonQ 64-qubit benchmark**: validated against S&P 500 portfolio data
//!
//! Source: ARC42 v20.0 §3 VCBP Quantum Optimisation Accelerator, ADR-027

pub mod engine;
pub mod solvers;
pub mod benchmark;
pub mod types;
pub mod errors;

pub use engine::QuantumEngine;
pub use solvers::{QaoaSolver, MaxKCutSolver, ClassicalSolver};
pub use benchmark::HybridBenchmark;
pub use types::{Portfolio, OptimizationResult, QubitBackend};
pub use errors::QuantumError;
