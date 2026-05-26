import { create } from 'zustand';
import type { AgentBoundary } from '@schemas/governance';

interface GovernanceState {
  boundaries: Map<string, AgentBoundary>;
  selectedAgent: string | null;
  setBoundary: (agentId: string, boundary: AgentBoundary) => void;
  selectAgent: (id: string | null) => void;
}

export const useGovernanceStore = create<GovernanceState>((set) => ({
  boundaries: new Map(),
  selectedAgent: null,
  setBoundary: (agentId, boundary) =>
    set((s) => {
      const next = new Map(s.boundaries);
      next.set(agentId, boundary);
      return { boundaries: next };
    }),
  selectAgent: (id) => set({ selectedAgent: id }),
}));
