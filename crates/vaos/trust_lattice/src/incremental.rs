//! Incremental Datalog maintenance for hypergraph closure.
//!
//! Source: Capability Safety as Datalog (March 20, 2026)
//!   When the underlying capability structure changes, incremental
//!   maintenance avoids full recomputation of the hypergraph closure.

use std::collections::{HashMap, HashSet};
use vaos_core::types::AgentId;

use super::hypergraph::CapabilityHypergraph;

/// Tracks which parts of the closure have been invalidated by structural changes.
#[derive(Debug, Default)]
pub struct IncrementalTracker {
    /// Agents whose capability sets have changed since last full closure
    dirty_agents: HashSet<AgentId>,
    /// Hyperedges that need re-evaluation
    dirty_edges: HashSet<usize>,
    /// Last known total closure size
    last_closure_size: usize,
}

impl IncrementalTracker {
    pub fn new() -> Self { Self::default() }

    pub fn mark_agent_dirty(&mut self, agent_id: AgentId) {
        self.dirty_agents.insert(agent_id);
    }

    pub fn mark_edge_dirty(&mut self, edge_index: usize) {
        self.dirty_edges.insert(edge_index);
    }

    pub fn is_dirty(&self) -> bool {
        !self.dirty_agents.is_empty() || !self.dirty_edges.is_empty()
    }

    pub fn clear(&mut self) {
        self.dirty_agents.clear();
        self.dirty_edges.clear();
    }
}
