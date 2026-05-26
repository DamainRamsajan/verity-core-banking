import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';

const API_BASE = '/api/v1';

async function fetchJson<T>(path: string, init?: RequestInit): Promise<T> {
  const res = await fetch(`${API_BASE}${path}`, {
    headers: { 'Content-Type': 'application/json', ...init?.headers },
    ...init,
  });
  if (!res.ok) {
    const body = await res.text();
    throw new Error(`API ${res.status}: ${body}`);
  }
  return res.json();
}

// ---------- Accounts ----------
export function useAccounts() {
  return useQuery({
    queryKey: ['accounts'],
    queryFn: () => fetchJson<{ success: boolean; data: any[] }>('/accounts'),
  });
}

export function useCreateAccount() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: any) =>
      fetchJson('/accounts', { method: 'POST', body: JSON.stringify(data) }),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['accounts'] }),
  });
}

// ---------- Transfers ----------
export function useCreateTransfer() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: any) =>
      fetchJson('/transfers', { method: 'POST', body: JSON.stringify(data) }),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['accounts'] }),
  });
}

// ---------- Agents ----------
export function useAgents() {
  return useQuery({
    queryKey: ['agents'],
    queryFn: () => fetchJson<{ success: boolean; data: any[] }>('/agents'),
  });
}

export function useAgentActivity(agentId: string) {
  return useQuery({
    queryKey: ['agent', agentId, 'activity'],
    queryFn: () =>
      fetchJson<{ success: boolean; data: any[] }>(`/agents/${agentId}/activity`),
    enabled: !!agentId,
    refetchInterval: 5_000,
  });
}

export function useAgentBoundaries(agentId: string) {
  return useQuery({
    queryKey: ['agent', agentId, 'boundaries'],
    queryFn: () =>
      fetchJson<{ success: boolean; data: any }>(`/agents/${agentId}`),
    enabled: !!agentId,
  });
}

export function useUpdateBoundaries() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ agentId, data }: { agentId: string; data: any }) =>
      fetchJson(`/agents/${agentId}/boundaries`, {
        method: 'PUT',
        body: JSON.stringify(data),
      }),
    onSuccess: (_, { agentId }) =>
      qc.invalidateQueries({ queryKey: ['agent', agentId] }),
  });
}

// ---------- Compliance ----------
export function useComplianceReports() {
  return useQuery({
    queryKey: ['compliance-reports'],
    queryFn: () =>
      fetchJson<{ success: boolean; data: any[] }>('/compliance/reports'),
  });
}

// ---------- Health ----------
export function useHealth() {
  return useQuery({
    queryKey: ['health'],
    queryFn: () => fetchJson<any>('/health'),
    refetchInterval: 30_000,
  });
}
