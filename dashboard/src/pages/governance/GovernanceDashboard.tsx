import { useState } from 'react';
import { Card, CardHeader, CardTitle, CardContent } from '@components/ui/Card';
import { Button } from '@components/ui/Button';
import { ErrorBoundary } from '@components/feedback/ErrorBoundary';
import { TransactionChart } from '@components/charts/TransactionChart';
import { FraudBarChart } from '@components/charts/FraudBarChart';
import { useGovernanceStore } from '@stores/governanceStore';
import { useAgentActivity, useAgentBoundaries, useUpdateBoundaries } from '@hooks/useApi';
import { Shield, AlertTriangle, CheckCircle, Activity } from 'lucide-react';
import { formatCurrency } from '@lib/utils';

const MOCK_TX = Array.from({ length: 30 }, (_, i) => ({
  date: new Date(2026, 4, i + 1).toISOString().slice(0, 10),
  volume: Math.random() * 50_000 + 10_000,
}));

const MOCK_FRAUD = [
  { level: 'Low', count: 45 },
  { level: 'Medium', count: 18 },
  { level: 'High', count: 7 },
  { level: 'Critical', count: 2 },
];

export default function GovernanceDashboard() {
  const [selectedAgent, setSelectedAgent] = useState('agent-001');
  const { boundaries } = useGovernanceStore();
  const { data: activity } = useAgentActivity(selectedAgent);
  const { data: agentBoundary } = useAgentBoundaries(selectedAgent);
  const updateBoundaries = useUpdateBoundaries();

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold">Delegative Governance</h1>
        <Button onClick={() => updateBoundaries.mutate({ agentId: selectedAgent, data: {} })}>
          <Shield size={16} /> Update Boundaries
        </Button>
      </div>

      {/* Stats row */}
      <div className="grid grid-cols-4 gap-4">
        {[
          { label: 'Active Agents', value: '12', icon: Activity, color: 'text-verity-600' },
          { label: 'Actions Today', value: '1,247', icon: CheckCircle, color: 'text-green-600' },
          { label: 'Boundary Violations', value: '3', icon: AlertTriangle, color: 'text-red-600' },
          { label: 'Compliance Rate', value: '99.8%', icon: Shield, color: 'text-verity-600' },
        ].map(({ label, value, icon: Icon, color }) => (
          <Card key={label}>
            <CardContent>
              <div className="flex items-center gap-3">
                <Icon size={24} className={color} />
                <div>
                  <p className="text-sm text-gray-500">{label}</p>
                  <p className="text-2xl font-bold">{value}</p>
                </div>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>

      {/* Charts row */}
      <div className="grid grid-cols-2 gap-4">
        <Card>
          <CardHeader><CardTitle>Transaction Volume</CardTitle></CardHeader>
          <CardContent>
            <ErrorBoundary><TransactionChart data={MOCK_TX} /></ErrorBoundary>
          </CardContent>
        </Card>
        <Card>
          <CardHeader><CardTitle>Fraud Detection</CardTitle></CardHeader>
          <CardContent>
            <ErrorBoundary><FraudBarChart data={MOCK_FRAUD} /></ErrorBoundary>
          </CardContent>
        </Card>
      </div>

      {/* Agent Activity Feed */}
      <Card>
        <CardHeader><CardTitle>Recent Agent Activity</CardTitle></CardHeader>
        <CardContent>
          <div className="space-y-2">
            {[
              { agent: 'Payment Agent #1', action: 'Wire Transfer', amount: 12_500, status: 'Approved' },
              { agent: 'Fraud Agent #3', action: 'Flagged Transaction', amount: 2_300, status: 'Reviewing' },
              { agent: 'Loan Agent #2', action: 'Loan Approval', amount: 250_000, status: 'Dual Control' },
            ].map((item, i) => (
              <div key={i} className="flex items-center justify-between rounded-lg border p-3 text-sm">
                <div>
                  <p className="font-medium">{item.agent}</p>
                  <p className="text-gray-500">{item.action}</p>
                </div>
                <div className="text-right">
                  <p className="font-mono font-medium">{formatCurrency(item.amount)}</p>
                  <span className="text-xs text-verity-600">{item.status}</span>
                </div>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
