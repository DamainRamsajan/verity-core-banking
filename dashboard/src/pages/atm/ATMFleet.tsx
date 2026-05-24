import { Card, CardHeader, CardTitle, CardContent } from '@components/ui/Card';

export default function ATMFleet() {
  return (
    <div className="space-y-6">
      <h1 className="text-2xl font-bold">ATM Fleet Management</h1>
      <div className="grid grid-cols-4 gap-4">
        {[
          { label: 'Total ATMs', value: '340' },
          { label: 'Online', value: '337' },
          { label: 'Cash Low', value: '12' },
          { label: 'Maintenance', value: '3' },
        ].map(({ label, value }) => (
          <Card key={label}><CardContent><p className="text-sm text-gray-500">{label}</p><p className="text-2xl font-bold">{value}</p></CardContent></Card>
        ))}
      </div>
    </div>
  );
}
