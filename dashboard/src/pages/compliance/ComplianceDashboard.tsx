import { Card, CardHeader, CardTitle, CardContent } from '@components/ui/Card';
import { FileCheck, AlertTriangle } from 'lucide-react';

export default function ComplianceDashboard() {
  return (
    <div className="space-y-6">
      <h1 className="text-2xl font-bold">Regulatory Compliance</h1>
      <div className="grid grid-cols-3 gap-4">
        {[
          { label: 'FFIEC Call Report', status: 'Filed', icon: FileCheck, color: 'text-green-600' },
          { label: 'DORA Register of Information', status: 'Due Jun 30', icon: AlertTriangle, color: 'text-amber-600' },
          { label: 'CFPB ECOA Review', status: 'Compliant', icon: FileCheck, color: 'text-green-600' },
        ].map(({ label, status, icon: Icon, color }) => (
          <Card key={label}><CardContent>
            <div className="flex items-center gap-3"><Icon size={24} className={color} /><div><p className="font-medium">{label}</p><p className="text-sm text-gray-500">{status}</p></div></div>
          </CardContent></Card>
        ))}
      </div>
    </div>
  );
}
