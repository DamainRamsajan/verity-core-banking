use std::collections::HashSet;
use vaos_core::types::AgentId;

#[derive(Debug, Default)]
pub struct IncrementalTracker {
    pub dirty_agents: HashSet<AgentId>,
    pub dirty_edges: HashSet<usize>,
}

impl IncrementalTracker {
    pub fn new() -> Self { Self::default() }
    pub fn mark_agent_dirty(&mut self, agent_id: AgentId) { self.dirty_agents.insert(agent_id); }
    pub fn mark_edge_dirty(&mut self, edge_index: usize) { self.dirty_edges.insert(edge_index); }
    pub fn is_dirty(&self) -> bool { !self.dirty_agents.is_empty() || !self.dirty_edges.is_empty() }
    pub fn clear(&mut self) { self.dirty_agents.clear(); self.dirty_edges.clear(); }
}