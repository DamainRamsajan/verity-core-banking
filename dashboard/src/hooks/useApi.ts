import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';

const API_BASE = import.meta.env.VITE_API_URL ?? '/api/v1';

async function fetchJson<T>(path: string, init?: RequestInit): Promise<T> {
  const res = await fetch(`${API_BASE}${path}`, {
    headers: { 'Content-Type': 'application/json', ...init?.headers },
    ...init,
  });
  if (!res.ok) throw new Error(`API ${res.status}: ${res.statusText}`);
  return res.json();
}

export function useAgentActivity(agentId: string) {
  return useQuery({
    queryKey: ['agent', agentId, 'activity'],
    queryFn: () => fetchJson<any[]>(`/agent/${agentId}/activity`),
    enabled: !!agentId,
    refetchInterval: 5_000,
  });
}

export function useAgentBoundaries(agentId: string) {
  return useQuery({
    queryKey: ['agent', agentId, 'boundaries'],
    queryFn: () => fetchJson<any>(`/agent/${agentId}/boundaries`),
    enabled: !!agentId,
  });
}

export function useUpdateBoundaries() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ agentId, data }: { agentId: string; data: any }) =>
      fetchJson(`/agent/${agentId}/boundaries`, { method: 'PUT', body: JSON.stringify(data) }),
    onSuccess: (_, { agentId }) => qc.invalidateQueries({ queryKey: ['agent', agentId] }),
  });
}
