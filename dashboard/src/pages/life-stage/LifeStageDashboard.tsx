import { Card, CardHeader, CardTitle, CardContent } from '@components/ui/Card';
import { Map, Home, Building2, Briefcase } from 'lucide-react';

export default function LifeStageDashboard() {
  return (
    <div className="space-y-6">
      <h1 className="text-2xl font-bold">Life-Stage Banking Orchestrator</h1>
      <div className="grid grid-cols-3 gap-4">
        {[
          { label: 'Buying a Home', icon: Home, desc: 'Mortgage pre-approval, insurance, renovation' },
          { label: 'Starting a Business', icon: Building2, desc: 'Business accounts, invoicing, tax prep' },
          { label: 'Building Wealth', icon: Briefcase, desc: 'Savings goals, investments, retirement' },
        ].map(({ label, icon: Icon, desc }) => (
          <Card key={label} className="cursor-pointer hover:border-verity-400 transition-colors">
            <CardHeader><Icon size={32} className="text-verity-600" /></CardHeader>
            <CardContent><CardTitle>{label}</CardTitle><p className="text-sm text-gray-500 mt-1">{desc}</p></CardContent>
          </Card>
        ))}
      </div>
    </div>
  );
}
