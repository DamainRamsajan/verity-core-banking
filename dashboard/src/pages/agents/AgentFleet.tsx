import { Card, CardHeader, CardTitle, CardContent } from '@components/ui/Card';

export default function AgentFleet() {
  return (
    <div className="space-y-6">
      <h1 className="text-2xl font-bold">Agent Fleet Management</h1>
      <div className="grid grid-cols-3 gap-4">
        {['Payment Agent #1', 'Fraud Agent #3', 'Loan Agent #2'].map(name => (
          <Card key={name}><CardHeader><CardTitle>{name}</CardTitle></CardHeader><CardContent><p className="text-sm text-green-600">Active · Capability Token Valid</p></CardContent></Card>
        ))}
      </div>
    </div>
  );
}
