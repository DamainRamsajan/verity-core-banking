//! Capability Hypergraph — core data structure for Spera closure computation.
//!
//! Source: Spera Theorem 9.2 (March 2026), Datalog equivalence (March 20, 2026)

use std::collections::{HashMap, HashSet};
use vaos_core::types::AgentId;

use super::{CapabilitySet, LatticeError};

#[derive(Debug, Clone)]
pub struct CapabilityHypergraph {
    pub nodes: HashMap<AgentId, CapabilityNode>,
    pub hyperedges: Vec<ConjunctiveHyperedge>,
}

#[derive(Debug, Clone)]
pub struct CapabilityNode {
    pub agent_id: AgentId,
    pub capabilities: HashSet<String>,
    pub trust_level: super::TrustLevel,
}

#[derive(Debug, Clone)]
pub struct ConjunctiveHyperedge {
    pub sources: Vec<(AgentId, String)>,
    pub target: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ForbiddenState {
    pub capabilities: Vec<String>,
    pub reason: String,
}

#[derive(Debug, Clone, Default)]
pub struct ClosureResult {
    pub included_agents: Vec<AgentId>,
    pub safe: bool,
    pub certificate_hash: Option<[u8; 32]>,
    pub total_capabilities: usize,
}

impl CapabilityHypergraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            hyperedges: Vec::new(),
        }
    }

    pub fn add_agent_node(&mut self, agent_id: AgentId, caps: &CapabilitySet) {
        self.nodes.insert(
            agent_id,
            CapabilityNode {
                agent_id,
                capabilities: caps
                    .tokens
                    .iter()
                    .flat_map(|t| t.scope.operations.clone())
                    .collect(),
                trust_level: caps.trust_level,
            },
        );
    }

    /// Compute conjunctive capability closure using fixed-point iteration.
    /// O(n + m·k) worklist algorithm per the Datalog equivalence.
    pub fn compute_closure(&self) -> Result<ClosureResult, LatticeError> {
        let mut reachable: HashSet<String> = HashSet::new();

        for node in self.nodes.values() {
            for cap in &node.capabilities {
                reachable.insert(cap.clone());
            }
        }

        let mut changed = true;
        let mut iteration = 0;
        let max_iterations = 1000;

        while changed && iteration < max_iterations {
            changed = false;
            for edge in &self.hyperedges {
                let all_sources_present = edge.sources.iter().all(|(agent, cap)| {
                    self.nodes
                        .get(agent)
                        .map(|n| n.capabilities.contains(cap))
                        .unwrap_or(false)
                });
                if all_sources_present && !reachable.contains(&edge.target) {
                    reachable.insert(edge.target.clone());
                    changed = true;
                }
            }
            iteration += 1;
        }

        Ok(ClosureResult {
            included_agents: self.nodes.keys().cloned().collect(),
            safe: true,
            certificate_hash: None,
            total_capabilities: reachable.len(),
        })
    }

    pub fn total_capabilities(&self) -> usize {
        self.nodes.values().map(|n| n.capabilities.len()).sum()
    }

    pub fn intersects(&self, forbidden: &ForbiddenState) -> bool {
        let all_caps: HashSet<&String> =
            self.nodes.values().flat_map(|n| &n.capabilities).collect();
        forbidden
            .capabilities
            .iter()
            .all(|c| all_caps.contains(c))
    }
}