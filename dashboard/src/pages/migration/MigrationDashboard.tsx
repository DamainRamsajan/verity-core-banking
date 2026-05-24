import { Card, CardHeader, CardTitle, CardContent } from '@components/ui/Card';

export default function MigrationDashboard() {
  return (
    <div className="space-y-6">
      <h1 className="text-2xl font-bold">Migration Dashboard</h1>
      <Card><CardHeader><CardTitle>Parallel-Run Status</CardTitle></CardHeader>
        <CardContent>
          <div className="space-y-2">
            {[
              { domain: 'Term Deposits', status: 'Complete', days: 90 },
              { domain: 'Savings', status: 'In Progress', days: 45 },
              { domain: 'Checking', status: 'Pending', days: 0 },
              { domain: 'Payments', status: 'Pending', days: 0 },
            ].map(d => (
              <div key={d.domain} className="flex items-center justify-between rounded-lg border p-3">
                <span className="font-medium">{d.domain}</span>
                <div className="flex items-center gap-4">
                  <span className="text-sm text-gray-500">{d.days} days validated</span>
                  <span className={`rounded-full px-2 py-0.5 text-xs font-medium ${
                    d.status === 'Complete' ? 'bg-green-100 text-green-700' :
                    d.status === 'In Progress' ? 'bg-blue-100 text-blue-700' : 'bg-gray-100 text-gray-600'
                  }`}>{d.status}</span>
                </div>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
