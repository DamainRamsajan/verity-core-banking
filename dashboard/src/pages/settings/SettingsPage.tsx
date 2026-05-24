import { Card, CardHeader, CardTitle, CardContent } from '@components/ui/Card';

export default function SettingsPage() {
  return (
    <div className="space-y-6">
      <h1 className="text-2xl font-bold">Settings</h1>
      <Card><CardHeader><CardTitle>Platform Configuration</CardTitle></CardHeader><CardContent><p className="text-gray-500">TEE mode, PQC migration phase, DP epsilon, and observability settings.</p></CardContent></Card>
    </div>
  );
}
