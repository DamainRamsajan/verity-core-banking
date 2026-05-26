import { useState } from 'react';
import { Card, CardHeader, CardTitle, CardContent } from '@components/ui/Card';
import { Button } from '@components/ui/Button';
import { ErrorBoundary } from '@components/feedback/ErrorBoundary';
import { TransactionChart } from '@components/charts/TransactionChart';
import { FraudBarChart } from '@components/charts/FraudBarChart';
import { useGovernanceStore } from '@stores/governanceStore';
import { useAgentActivity, useAgents, useUpdateBoundaries } from '@hooks/useApi';
import { Shield, AlertTriangle, CheckCircle, Activity } from 'lucide-react';
import { formatCurrency } from '@lib/utils';

export default function GovernanceDashboard() {
  const [selectedAgent, setSelectedAgent] = useState('');
  const { boundaries, setBoundary } = useGovernanceStore();
  const { data: agents } = useAgents();
  const { data: activity } = useAgentActivity(selectedAgent);
  const updateBoundaries = useUpdateBoundaries();

  const agentList = agents?.data ?? [];

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold">Delegative Governance</h1>
        <Button
          onClick={() =>
            updateBoundaries.mutate({
              agentId: selectedAgent,
              data: { spending_limit: 500 },
            })
          }
          disabled={!selectedAgent}
        >
          <Shield size={16} /> Update Boundaries
        </Button>
      </div>

      {/* Stats row */}
      <div className="grid grid-cols-4 gap-4">
        {[
          { label: 'Active Agents', value: agentList.length, icon: Activity, color: 'text-verity-600' },
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
            <ErrorBoundary>
              <TransactionChart data={[]} />
            </ErrorBoundary>
          </CardContent>
        </Card>
        <Card>
          <CardHeader><CardTitle>Fraud Detection</CardTitle></CardHeader>
          <CardContent>
            <ErrorBoundary>
              <FraudBarChart data={[]} />
            </ErrorBoundary>
          </CardContent>
        </Card>
      </div>

      {/* Agent selector */}
      <Card>
        <CardHeader><CardTitle>Agent Activity</CardTitle></CardHeader>
        <CardContent>
          <select
            className="mb-4 w-full rounded-lg border p-2"
            value={selectedAgent}
            onChange={(e) => setSelectedAgent(e.target.value)}
          >
            <option value="">-- Select an agent --</option>
            {agentList.map((a: any) => (
              <option key={a.agent_id} value={a.agent_id}>
                {a.name}
              </option>
            ))}
          </select>
          <div className="space-y-2">
            {(activity?.data ?? []).slice(0, 5).map((item: any, i: number) => (
              <div key={i} className="flex items-center justify-between rounded-lg border p-3 text-sm">
                <div>
                  <p className="font-medium">{item.agent_id}</p>
                  <p className="text-gray-500">{item.action}</p>
                </div>
                <div className="text-right">
                  <p className="font-mono font-medium">
                    {formatCurrency(item.amount ?? 0)}
                  </p>
                  <span className="text-xs text-verity-600">
                    {item.within_boundary ? 'Within Boundary' : 'Review'}
                  </span>
                </div>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
